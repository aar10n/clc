use crate::value::{Value};
use crate::names::{Name};
use crate::lexer::Token;
use crate::names::get_name;
use crate::lexer::Token::Function;
use std::slice::{Iter};
use phf::phf_map;
use std::borrow::{Borrow, BorrowMut};

#[derive(Copy, Clone)]
pub enum Assoc {
  Left,
  Right
}

pub const LONGEST_OPERATOR: usize = 2;
pub const OPERATORS: &[&str] = &[
  "+", "-", "*", "/", "%",
  "==", "!=", ">", "<", ">=", "<=",
  "&", "|", "^", "<<", ">>",
  "&&", "||",
  "~", "!",
];

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

//

pub fn is_unary(s: &str) -> bool {
  let us = format!("{}u", s);
  return PRECEDENCE_TABLE.get(us.as_str()).is_some();
}

pub fn get_prec(s: &str) -> i32 {
  return match PRECEDENCE_TABLE.get(s) {
    Some((prec, _)) => *prec,
    None => -1,
  }
}

pub fn get_assoc(s: &str) -> Assoc {
  return match PRECEDENCE_TABLE.get(s) {
    Some((_, assoc)) => *assoc,
    None => Assoc::Left,
  }
}

//

//
fn process_tokens<'a>(tokens: &Vec<Token>) -> Result<Vec<Token>, String> {
  use Token::*;
  let mut out_tokens: Vec<Token> = vec!();

  for token in tokens.iter() {
    match token {
      Integer(_) | Float(_) => {
        out_tokens.push(Value(token.as_value().unwrap()));
        continue;
      },
      UnaryOp(op) => {
        if op == "+u" {
          // ignore unary +
          continue;
        }
      },
      Identifier(id) => {
        let name = get_name(id);
        if name.is_none() {
          return Err(format!("Unknown name '{}'", id))
        }


        match name.unwrap() {
          Name::Constant(val) => out_tokens.push(Value(val())),
          Name::Function(func) => out_tokens.push(Function(id.clone(), func))
        }
        continue;
      },
      _ => (),
    }
    out_tokens.push(token.clone());
  }

  return Ok(out_tokens);
}

pub fn parse(tokens: &Vec<Token>) -> Result<Value, String> {
  use Token::*;

  let mut op_stack: Vec<&'_ Token> = vec!();
  let mut output: Vec<&'_ Token> = vec!();

  println!("--- tokens ---");
  println!("{:?}", tokens);

  let toks = process_tokens(tokens)?;
  println!("--- processed ---");
  println!("{:?}", toks);

  for token in toks.iter() {
    match token {
      Value(_) => {
        output.push(token);
      },
      Function(_, _) => {
        op_stack.push(token);
      },
      BinaryOp(_) | UnaryOp(_) => {
        while !op_stack.is_empty() {
          let op = op_stack.last().unwrap().borrow_mut();
          if op.prec() >= token.prec() && matches!(token.assoc(), Assoc::Left) {
            op_stack.pop();
            output.push(op);
          } else {
            break;
          }
        }
        op_stack.push(token);
      },
      LParen() => {
        op_stack.push(token);
      },
      RParen() => {
        while !op_stack.is_empty() {
          let op = op_stack.last().unwrap();
          if matches!(op, LParen()) {
            break;
          }
          // op_stack.pop();
          op_stack.remove(op_stack.len() - 1);
          output.push(op);
        }

        if op_stack.is_empty() || !matches!(op_stack.last().unwrap(), LParen()) {
          return Err(String::from("Encountered ')' without matching '('"));
        }
        // op_stack.pop();
        op_stack.remove(op_stack.len() - 1);

        if op_stack.last().map_or(false, |t| matches!(t, Function(_, _))) {
          let func = op_stack.last().unwrap();
          op_stack.remove(op_stack.len() - 1);
          output.push(func);
        }
      }
      Newline() => {
        while !op_stack.is_empty() {
          let op = op_stack.last().unwrap();
          match op {
            LParen() => {
              return Err(String::from("Unmatched ')'"))
            },
            _ => {
              // op_stack.pop();
              op_stack.remove(op_stack.len() - 1);
              output.push(op);
            }
          }
        }

        println!("Line parsing done");
      }
      _ => panic!("unexpected token")
    }
  }

  println!("--- output ---");
  println!("{:?}", output);
  return Ok(crate::value::Value::Float(3.14));
}
