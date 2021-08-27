use crate::buffer::Buffer;
use crate::lexer::Token;
use crate::names::{get_name, Name};
use crate::operators::{conv_op_token, Assoc};
use crate::value::{Value, Width};

//

fn process_tokens(tokens: &Vec<Token>, buffer: &Buffer) -> Result<Vec<Token>, String> {
  let mut out_tokens: Vec<Token> = vec![];
  let mut nl_count = 0;
  for token in tokens.iter() {
    match token {
      Token::Newline() => nl_count += 1,
      _ => nl_count = 0,
    }

    match token {
      Token::Integer(_) | Token::Float(_) => {
        out_tokens.push(Token::Value(token.as_value().unwrap()));
        continue;
      }
      Token::BinaryOp(_) => {
        out_tokens.push(conv_op_token(token));
        continue;
      }
      Token::UnaryOp(op) => {
        if op == "+u" {
          // ignore unary +
          continue;
        }
        out_tokens.push(conv_op_token(token));
        continue;
      }
      Token::Reference(num) => {
        let val = buffer.get(*num as usize);
        out_tokens.push(Token::Value(val));
        continue;
      }
      Token::Identifier(id) => {
        let name = get_name(id);
        if name.is_none() {
          return Err(format!("Unknown name '{}'", id));
        }

        match name.unwrap() {
          Name::Constant(val) => out_tokens.push(Token::Value(val())),
          Name::Function(func) => out_tokens.push(Token::UnaryFunction(id.clone(), func)),
        }
        continue;
      }
      Token::Newline() => {
        if nl_count > 1 {
          // ignore extra newlines
          continue;
        }
      }
      _ => (),
    }
    out_tokens.push(token.clone());
  }

  if out_tokens.last().map_or(false, |v| !matches!(v, Token::Newline())) {
    out_tokens.push(Token::Newline());
  }
  return Ok(out_tokens);
}

fn eval_expression(tokens: &Vec<&Token>) -> Result<Value, String> {
  // expects the expression in rpn form
  let mut stack: Vec<Value> = vec![];
  let mut args: i32 = 0;
  for token in tokens.iter() {
    match token {
      Token::Value(v) => {
        stack.push(*v);
        args += 1;
      }
      Token::UnaryFunction(name, func) => {
        if args < 1 {
          return Err(format!("Expected one argument to {}", name));
        }

        let arg = stack.pop().unwrap();
        args -= 1;
        let res = func(arg);
        // println!("{} | {:?} -> {:?}", name, arg, res);
        stack.push(res);
        args += 1;
      }
      Token::BinaryFunction(name, func) => {
        if args < 2 {
          return Err(format!("Expected two arguments to {}", name));
        }

        let arg2 = stack.pop().unwrap();
        let arg1 = stack.pop().unwrap();
        args -= 2;
        let res = func(arg1, arg2);
        // println!("{} | {:?} {:?} -> {:?}", name, arg1, arg2, res);
        stack.push(res);
        args += 1;
      }
      _ => panic!("unexpected token"),
    }
  }

  if stack.len() != 1 {
    eprintln!("stack: {:?}", stack);
    panic!("unexpected stack state");
  }

  let value = stack.pop().unwrap();
  return Ok(value);
}

//

pub fn parse(tokens: &Vec<Token>, buffer: &mut Buffer) -> Result<Value, String> {
  let mut op_stack: Vec<&'_ Token> = vec![];
  let mut output: Vec<&'_ Token> = vec![];
  let mut values: Vec<Value> = vec![];

  // println!("--- tokens ---");
  // println!("{:?}", tokens);

  let toks = process_tokens(tokens, buffer)?;
  // println!("--- processed ---");
  // println!("{:?}", toks);

  // convert expression to rpn
  for token in toks.iter() {
    match token {
      Token::Value(_) => {
        output.push(token);
      }
      Token::UnaryFunction(_, _) => {
        op_stack.push(token);
      }
      Token::BinaryFunction(_, _) => {
        while !op_stack.is_empty() {
          let op = *op_stack.last().unwrap();
          if op.prec() >= token.prec() && matches!(token.assoc(), Assoc::Left) {
            op_stack.pop();
            output.push(op);
          } else {
            break;
          }
        }
        op_stack.push(token);
      }
      Token::LParen() => {
        op_stack.push(token);
      }
      Token::RParen() => {
        while !op_stack.is_empty() {
          let op = *op_stack.last().unwrap();
          if matches!(op, Token::LParen()) {
            break;
          }
          op_stack.pop();
          output.push(op);
        }

        if op_stack.is_empty() || !matches!(op_stack.last().unwrap(), Token::LParen()) {
          return Err(String::from("Encountered ')' without matching '('"));
        }
        op_stack.pop();

        if op_stack.last().map_or(false, |t| {
          matches!(t, Token::UnaryFunction(_, _)) || matches!(t, Token::BinaryFunction(_, _))
        }) {
          let func = *op_stack.last().unwrap();
          op_stack.pop();
          output.push(func);
        }
      }
      Token::Newline() => {
        while !op_stack.is_empty() {
          let op = *op_stack.last().unwrap();
          match op {
            Token::LParen() => return Err(String::from("Unmatched ')'")),
            _ => {
              op_stack.pop();
              output.push(op);
            }
          }
        }

        // println!("--- output ---");
        // println!("{:?}", output);
        // println!("--- evaluating ---");
        let value = eval_expression(&output)?;
        // println!("--- value ---");
        // println!("{:?}", value);
        values.push(value);
        buffer.add(value);
      }
      _ => panic!("unexpected token {:?}", token),
    }
  }

  let last = values.last();
  if last.is_some() {
    return Ok(*last.unwrap());
  }
  return Ok(Value::Integer(0, Width::U64));
}
