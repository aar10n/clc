use crate::lexer::Token;
use crate::value::Value;
use phf::phf_map;

#[derive(Copy, Clone)]
pub enum Assoc {
  Left,
  Right,
}

pub const LONGEST_OPERATOR: usize = 2;
pub const OPERATORS: &[&str] = &[
  "+", "-", "*", "/", "%", "==", "!=", ">", "<", ">=", "<=", "&", "|", "^", "<<", ">>", "&&", "||", "~", "!",
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

const UNARY_OPERATORS: phf::Map<&'static str, fn(Value) -> Value> = phf_map! {
  "+u" => |v| v,
  "-u" => |v| -v.to_signed(),
  "!u" => |v| Value::from(!bool::from(v)),
  "~u" => |v| !v,
};

const BINARY_OPERATORS: phf::Map<&'static str, fn(Value, Value) -> Value> = phf_map! {
  "+" => |a, b| a + b,
  "-" => |a, b| a.to_signed() - b.to_signed(),
  "*" => |a, b| a * b,
  "/" => |a, b| a / b,
  "%" => |a, b| a % b,

  "&" => |a, b| a & b,
  "|" => |a, b| a | b,
  "^" => |a, b| a ^ b,

  "<<" => |a, b| a << b,
  ">>" => |a, b| a >> b,

  "<" => |a, b| Value::from(a < b),
  ">" => |a, b| Value::from(a > b),
  ">=" => |a, b| Value::from(a >= b),
  "<=" => |a, b| Value::from(a <= b),
  "==" => |a, b| Value::from(a == b),
  "!=" => |a, b| Value::from(a != b),

  "&&" => |a, b| Value::from(bool::from(a) && bool::from(b)),
  "||" => |a, b| Value::from(bool::from(a) || bool::from(b)),
};

//

pub fn is_unary(s: &str) -> bool {
  let us = format!("{}u", s);
  return PRECEDENCE_TABLE.get(us.as_str()).is_some();
}

pub fn get_prec(s: &str) -> i32 {
  match PRECEDENCE_TABLE.get(s) {
    Some((prec, _)) => *prec,
    None => -1,
  }
}

pub fn get_assoc(s: &str) -> Assoc {
  match PRECEDENCE_TABLE.get(s) {
    Some((_, assoc)) => *assoc,
    None => Assoc::Left,
  }
}

pub fn conv_op_token(t: &Token) -> Token {
  use Token::*;
  match t {
    UnaryOp(name) => {
      let func = *UNARY_OPERATORS.get(name.as_str()).unwrap();
      UnaryFunction((*name).clone(), func)
    }
    BinaryOp(name) => {
      let func = *BINARY_OPERATORS.get(name.as_str()).unwrap();
      BinaryFunction((*name).clone(), func)
    }
    _ => panic!("expected unary or binary operator token"),
  }
}
