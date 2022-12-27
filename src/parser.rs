use crate::functions::{get_constant, get_function, Function};
use crate::lexer::Token;
use crate::value::{Value, Width};
use phf::phf_map;

const PRECEDENCE_TABLE: phf::Map<&'static str, (i32, Assoc)> = phf_map! {
  "+u" => (11, Assoc::Right), // unary plus
  "-u" => (11, Assoc::Right), // unary minus
  "!u" => (10, Assoc::Right), // logical not
  "~u" => (10, Assoc::Right), // bitwise not

  "*" => (9, Assoc::Left),   // multiplication
  "/" => (9, Assoc::Left),   // division
  "%" => (9, Assoc::Left),   // modulo

  "+" => (8, Assoc::Left),   // addition
  "-" => (8, Assoc::Left),   // subtraction

  "<<" => (7, Assoc::Left),  // bitwise left shift
  ">>" => (7, Assoc::Left),  // bitwise right shift

  ">" => (6, Assoc::Left),   // greater than
  "<" => (6, Assoc::Left),   // less than
  ">=" => (6, Assoc::Left),  // greater than or equal to
  "<=" => (6, Assoc::Left),  // greater than or equal to
  "==" => (5, Assoc::Left),  // equal to
  "!=" => (5, Assoc::Left),  // not equal to

  "&" => (4, Assoc::Left),   // bitwise and
  "^" => (3, Assoc::Left),   // bitwise xor
  "|" => (2, Assoc::Left),   // bitwise or

  "&&" => (1, Assoc::Left),  // logical and
  "||" => (1, Assoc::Left),  // logical or

  "(" => (0, Assoc::Right),   // parentheses
};

#[derive(Copy, Clone, Debug)]
pub enum Assoc {
  Left,
  Right,
}

/// Converts an infix expression to postfix notation.
/// It also checks that all identifiers are valid and that the expression is well-formed.
fn convert_expr_posfix(expr: Vec<Token>) -> Result<Vec<Token>, String> {
  let mut op_stack: Vec<Token> = vec![];
  let mut rpn_expr: Vec<Token> = vec![];

  for token in expr.into_iter() {
    match token {
      Token::Value(_) => rpn_expr.push(token),
      Token::Identifier(id) => {
        if let Some(value) = get_constant(&id) {
          rpn_expr.push(Token::Value(value));
        } else if let Some(_) = get_function(&id) {
          op_stack.push(Token::Identifier(id));
        } else {
          return Err(format!("Unknown identifier '{}'", id));
        }
      }
      Token::Operator(op) => {
        // pop operators off the stack until we find one with a lower precedence
        let (prec, assoc) = PRECEDENCE_TABLE[&op];

        while let Some(other) = op_stack.last() {
          let (o_prec, _) = match other {
            Token::Operator(t_op) => PRECEDENCE_TABLE[t_op],
            _ => break,
          };

          if o_prec >= prec && matches!(assoc, Assoc::Left) {
            rpn_expr.push(op_stack.pop().unwrap());
          } else {
            break;
          }
        }
        op_stack.push(Token::Operator(op));
      }
      Token::LParen => op_stack.push(token),
      Token::RParen => {
        // pop operators off the stack until we find a '('
        while let Some(t) = op_stack.pop() {
          if t.is_lparen() {
            op_stack.push(t);
            break;
          }
          rpn_expr.push(t);
        }

        // the stack isn't empty and we didn't find a '(' then there's a mismatched ')'
        if op_stack.is_empty() || !op_stack.last().unwrap().is_lparen() {
          return Err("Encountered ')' without matching '('".to_string());
        }
        op_stack.pop();

        // if the next token is a function then pop it into the output array
        if matches!(op_stack.last(), Some(Token::Identifier(_))) {
          rpn_expr.push(op_stack.pop().unwrap());
        }
      }
      Token::Newline => unreachable!(),
    }
  }

  while let Some(t) = op_stack.pop() {
    if t.is_lparen() {
      return Err("Unmatched ')'".to_string());
    }
    rpn_expr.push(t);
  }
  Ok(rpn_expr)
}

/// Evaluates a postfix expression and returns the result.
fn evaluate_expr_postfix(expr: &Vec<Token>) -> Result<Value, String> {
  if expr.is_empty() {
    panic!("empty expression");
  }

  let mut stack: Vec<Value> = vec![];
  let mut nargs: usize = 0;

  for token in expr.into_iter() {
    if let Token::Value(v) = token {
      stack.push(*v);
      nargs += 1;
      continue;
    }

    let name = match token {
      Token::Identifier(name) | Token::Operator(name) => name,
      _ => unreachable!(),
    };

    let func = get_function(name).unwrap();
    match func {
      Function::Unary(func) => {
        if nargs < 1 {
          return Err(format!("Expected one argument to {}", name));
        }

        let arg = stack.pop().unwrap();
        stack.push(func(arg));
      }
      Function::Binary(func) => {
        if nargs < 2 {
          return Err(format!("Expected two arguments to {}", name));
        }

        let arg2 = stack.pop().unwrap();
        let arg1 = stack.pop().unwrap();
        stack.push(func(arg1, arg2));
        nargs -= 1; // we popped two but added one back
      }
    }
  }

  if stack.len() != 1 {
    eprintln!("stack: {:?}", stack);
    panic!("unexpected stack state");
  }
  let value = stack.pop().unwrap();
  return Ok(value);
}

pub fn parse(tokens: Vec<Token>) -> Result<Value, String> {
  let mut values: Vec<Value> = vec![];
  for expr in tokens.split(|t| t.is_newline()) {
    if expr.is_empty() {
      continue;
    }

    // println!("--- tokens ---");
    // println!("infix: {:?}", expr);
    let rpn_expr = convert_expr_posfix(expr.to_vec())?;
    if rpn_expr.is_empty() {
      // empty expression like "()"
      continue;
    }

    // println!("postfix: {:?}", rpn_expr);
    let value = evaluate_expr_postfix(&rpn_expr)?;
    // println!("value: {}", value.to_string());
    values.push(value);
  }

  let last = values.last();
  if last.is_some() {
    return Ok(*last.unwrap());
  }
  return Ok(Value::Integer(0, Width::U64));
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tokenize;
  use test_case::test_case;

  #[test_case("()" => Ok(Value::Integer(0, Width::U64)))]
  #[test_case("1" => Ok(Value::Integer(1, Width::U64)))]
  #[test_case("1 + 2" => Ok(Value::Integer(3, Width::U64)))]
  #[test_case("1.5 * 3" => Ok(Value::Float(4.5)))]
  #[test_case("3 * 1.5" => Ok(Value::Integer(3, Width::U64)))]
  #[test_case("(1 + 2) * 3" => Ok(Value::Integer(9, Width::U64)))]
  #[test_case("sin(deg(90))" => Ok(Value::Float(1.0)))]
  #[test_case("u32(1)" => Ok(Value::Integer(1, Width::U32)))]
  #[test_case("u32(1) + 1" => Ok(Value::Integer(2, Width::U32)))]
  fn test_parse(input: &str) -> Result<Value, String> {
    let tokens = tokenize(input)?;
    parse(tokens)
  }
}
