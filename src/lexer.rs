use crate::parser::{Assoc, OPERATORS, LONGEST_OPERATOR, is_unary, get_prec, get_assoc};
use crate::value::{Value, Width};
use crate::names::{Name};
use std::cmp::min;
use std::str::FromStr;
use regex::{Regex, Match};

macro_rules! char_at {
  ($p: expr, $i: expr) => {
    if $i < $p.len() { $p[$i] as char } else { '\0' as char }
  };
}

macro_rules! bin_chars {
  () => { '0'..='1' };
}
macro_rules! oct_chars {
  () => { '0'..='7' };
}
macro_rules! digit_chars {
  () => { '0'..='9' };
}
macro_rules! hex_chars {
  () => { '0'..='9' | 'a'..='f' | 'A'..='F' }
}
macro_rules! ident_chars {
  () => { 'a'..='z' | 'A'..='Z' | '_' };
}

#[derive(Debug, Clone)]
pub enum Token {
  // base tokens
  Integer(u64),       // eg. 10, 0xA, 0o777, 0b00101
  Float(f64),         // eg. 3.141, 0.0001, .5
  Reference(u64),     // eg. $1, $2, $3
  Identifier(String), // eg. sin, cos, PI
  BinaryOp(String),   // eg. *, /, %, &
  UnaryOp(String),    // eg. +, -, !, ~
  LParen(),           // eg. (
  RParen(),           // eg. )
  Newline(),          // eg. \n

  // intermediate tokens
  Value(Value),
  Function(String, fn(Value) -> Value),
}

impl Token {
  pub fn prec(&self) -> i32 {
    use Token::*;
    match self {
      UnaryOp(str) | BinaryOp(str) => get_prec(str),
      _ => -1,
    }
  }

  pub fn assoc(&self) -> Assoc {
    use Token::*;
    match self {
      UnaryOp(str) | BinaryOp(str) => get_assoc(str),
      _ => Assoc::Left,
    }
  }

  pub fn as_value(&self) -> Option<Value> {
    match self {
      Token::Integer(val) => Some(Value::Integer(*val, Width::U64)),
      Token::Float(val) => Some(Value::Float(*val)),
      _ => None,
    }
  }
}


fn get_op_regex() -> Regex {
  let mut operators: Vec<String> = OPERATORS
      .to_vec()
      .iter()
      .map(|s| String::from(regex::escape(s)))
      .collect();

  operators.sort_by(|a, b| b.len().cmp(&a.len()));
  let pattern: String = format!("^({})", operators.join("|"));
  return Regex::new(&*(pattern)).unwrap();
}

//

// Forms an integer token
fn lex_integer_literal(program: &[u8], i: usize, base: u8) -> Result<(Token, usize), String> {
  // i points to the first character following the prefix (if there is one)
  assert!(base == 2 || base == 8 || base == 10 || base == 16);
  if i >= program.len() {
    return Err(String::from("Unexpected end of input"));
  }

  let mut j = i;
  while j < program.len() {
    match program[j] as char {
      bin_chars!() if base == 2 => j += 1,
      oct_chars!() if base == 8 => j += 1,
      digit_chars!() if base == 10 => j += 1,
      hex_chars!() if base == 16 => j += 1,
      ident_chars!() => {
        return Err(String::from("Invalid character in integer literal"));
      },
      _ => break,
    }
  }

  if i == j {
    return Err(String::from("Expected digit in integer literal"));
  }

  let slice = std::str::from_utf8(&program[i..j])
      .or(Err("Input is not valid utf8"));
  return u64::from_str_radix(slice?, base as u32)
      .and_then(|res| Ok((Token::Integer(res), j)))
      .or(Err(String::from("Failed to parse integer literal")));
}

// Forms a float token
fn lex_float_literal(program: &[u8], i: usize, j: usize) -> Result<(Token, usize), String> {
  // i points to the start of the value
  // j points to the first character following the '.'
  if i >= program.len() {
    return Err(String::from("Unexpected end of input"));
  }

  let mut k = j;
  while k < program.len() {
    match program[k] as char {
      digit_chars!() => k += 1,
      ident_chars!() => {
        return Err(String::from("Invalid character in float literal"));
      }
      _ => break,
    }
  }

  if j == k {
    return Err(String::from("Expected digit in float literal"));
  }

  let slice = std::str::from_utf8(&program[i..k])
      .or(Err("Input is not valid utf8"));
  return f64::from_str(&slice?)
      .and_then(|res| Ok((Token::Float(res), k)))
      .or(Err(String::from("Failed to parse float literal")));
}

// Forms an integer or float token
fn lex_numeric_literal(program: &[u8], i: usize) -> Result<(Token, usize), String> {
  // this function only handles the ambiguous case
  // of decimal integers and floating point values
  if i == program.len() - 1 {
    // only a single digit
    let slice = std::str::from_utf8(&program[i..])
        .or(Err("Input is not valid utf8"));
    return u64::from_str(slice?)
        .and_then(|res| Ok((Token::Integer(res), i + 1)))
        .or(Err(String::from("Failed to parse integer literal")));
  }

  let mut period: bool = false;
  let mut j = i;
  while j < program.len() {
    match program[j] as char {
      digit_chars!() => j += 1,
      ident_chars!() => {
        return Err(String::from("Invalid character in numeric literal"));
      },
      '.' => {
        j += 1;
        period = true;
        break;
      },
      _ => break,
    }
  }

  if period {
    return lex_float_literal(program, i, j);
  }
  return lex_integer_literal(program, i, 10);
}

// Forms a reference token
fn lex_reference(program: &[u8], i: usize) -> Result<(Token, usize), String> {
  // i points to the first character following the '$'
  if i >= program.len() {
    return Err(String::from("Unexpected end of input after '$'"));
  }

  let mut zero = false;
  let mut start = true;
  let mut j = i;
  while j < program.len() {
    if zero && start {
      return Err(String::from("Invalid reference"));
    }

    match program[j] as char {
      '0' => {
        j += 1;
        zero = true;
      },
      digit_chars!() => {
        j += 1;
        zero = false;
        start = false;
      },
      ident_chars!() => {
        return Err(String::from("Invalid character in reference"));
      },
      _ => break,
    }
  }

  if i == j {
    return Err(String::from("Invalid reference"));
  }

  let slice = std::str::from_utf8(&program[i..j])
      .or(Err("Input is not valid utf8"));
  return u64::from_str(slice?)
      .and_then(|res| Ok((Token::Reference(res), j)))
      .or(Err(String::from("Failed to parse reference")));
}

// Forms an identifier token
fn lex_identifier(program: &[u8], i: usize) -> Result<(Token, usize), String> {
  // i points to the first character of the identifier
  let mut j: usize = i;
  while j < program.len() {
    match program[j] as char {
      ident_chars!() => j += 1,
      digit_chars!() => {
        return Err(String::from("Invalid character in identifier"));
      },
      _ => break,
    }
  }

  let id = std::str::from_utf8(&program[i..j])
      .and_then(|s| Ok(String::from(s)))
      .or(Err("Input is not valid utf8"));
  return Ok((Token::Identifier(id?), j));
}

//

pub fn tokenize(program: &[u8]) -> Result<Vec<Token>, String> {
  let mut tokens: Vec<Token> = Vec::new();
  let op_regex: Regex = get_op_regex();

  let mut i: usize = 0;
  let mut result: Result<(Token, usize), String>;
  while i < program.len() {
    match program[i] as char {
      '0' => {
        // integer with prefix or numeric literal
        let mut base: u8 = 10;
        let mut integer = true;
        match char_at!(program, i + 1) {
          'b' => base = 2,
          'o' => base = 8,
          'x' => base = 16,
          _ => integer = false,
        }

        if integer {
          result = lex_integer_literal(program, i + 2, base);
        } else {
          result = lex_numeric_literal(program, i);
        }
      },
      digit_chars!() => {
        // numeric literal
        result = lex_numeric_literal(program, i);
      },
      '.' => {
        // float
        result = lex_float_literal(program, i, i + 1);
      },
      '$' => {
        // reference
        result = lex_reference(program, i + 1);
      },
      ident_chars!() => {
        // identifier
        result = lex_identifier(program, i);
      },
      '(' => {
        // left parenthesis
        tokens.push(Token::LParen());
        i += 1;
        continue;
      },
      ')' => {
        // right parenthesis
        tokens.push(Token::RParen());
        i += 1;
        continue;
      },
      '\n' => {
        // newline
        tokens.push(Token::Newline());
        i += 1;
        continue;
      },
      ' ' | '\t' => {
        // whitespace
        i += 1;
        continue;
      }
      _ => {
        // possibly an operator
        let end = min(i + LONGEST_OPERATOR, program.len());
        let slice = std::str::from_utf8(&program[i..end])
            .or(Err(String::from("Failed to convert slice to UTF-8")))
            .unwrap();

        let m = op_regex.find(slice);
        if m.is_none() {
          return Err(String::from("Unexpected character in input"));
        }

        let op = m.unwrap();
        let start = i + op.start();
        let end = i + op.end();
        let op_str = std::str::from_utf8(&program[start..end])
            .or(Err(String::from("Input is not valid utf8")))?;

        if is_unary(op_str) {
          let unary = match tokens.last() {
            Some(Token::Integer(_)) |
            Some(Token::Float(_)) |
            Some(Token::Reference(_)) => false,
            _ => true,
          };

          if unary {
            result = Ok((Token::UnaryOp(format!("{}u", op_str)), end));
          } else {
            result = Ok((Token::BinaryOp(String::from(op_str)), end));
          }
        } else {
          result = Ok((Token::BinaryOp(String::from(op_str)), end));
        }
      }
    }

    let (token, new_i) = result?;
    tokens.push(token);
    i = new_i;
  }

  return Ok(tokens);
}
