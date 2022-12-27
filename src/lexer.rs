use crate::value::Value;
use logos::{Lexer, Logos};
use std::str::FromStr;

/// A final token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  Value(Value),
  Identifier(String),
  Operator(String),
  LParen,
  RParen,
  Newline,
}

impl Token {
  pub fn is_value(&self) -> bool {
    matches!(self, Token::Value(_))
  }

  pub fn is_identifier(&self) -> bool {
    matches!(self, Token::Identifier(_))
  }

  pub fn is_operator(&self) -> bool {
    matches!(self, Token::Operator(_))
  }

  pub fn is_binary_op(&self) -> bool {
    matches!(self, Token::Operator(op) if !op.ends_with("u"))
  }

  pub fn is_unary_op(&self) -> bool {
    matches!(self, Token::Operator(op) if op.ends_with("u"))
  }

  pub fn is_lparen(&self) -> bool {
    matches!(self, Token::LParen)
  }

  pub fn is_rparen(&self) -> bool {
    matches!(self, Token::RParen)
  }

  pub fn is_newline(&self) -> bool {
    matches!(self, Token::Newline)
  }
}

impl From<Value> for Token {
  fn from(value: Value) -> Self {
    Token::Value(value)
  }
}

/// Raw tokens produced by the lexer.
#[derive(Logos, Clone, Debug, PartialEq)]
pub enum RawToken {
  // eg. 101, 0x1F, 0o777, 0b1101
  #[regex(r"0x[0-9a-fA-F]+|0o[0-7]+|0b[01]+|[0-9]+", conv_integer)]
  Integer(u64),
  // eg. 3.141, 0.0001, 2., .5
  #[regex(r"\d+\.\d*|\.\d+", conv_float)]
  Float(f64),

  // eg. sin, cos, PI
  #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
  Identifier,
  // eg. *, /, %, &
  #[regex(r"==|!=|>|<|>=|<=|&|\||\^|<<|>>|&&|\|\||~|!|\+|-|\*|/|%")]
  Operator,
  // eg. (
  #[token("(")]
  LParen,
  // eg. )
  #[token(")")]
  RParen,
  // eg. \n
  #[token("\n")]
  Newline,

  #[regex(r"[ \t]+", logos::skip)] // skip whitespace
  #[error]
  Error,
}

fn conv_integer(lex: &mut Lexer<RawToken>) -> Option<u64> {
  let slice = lex.slice();

  let (slice, radix) = match slice {
    s if s.starts_with("0x") => (&s[2..], 16),
    s if s.starts_with("0o") => (&s[2..], 8),
    s if s.starts_with("0b") => (&s[2..], 2),
    s => (s, 10),
  };

  return u64::from_str_radix(slice, radix).ok();
}

fn conv_float(lex: &mut Lexer<RawToken>) -> Option<f64> {
  let slice = lex.slice();
  f64::from_str(slice).ok()
}

//

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
  let mut lexer = RawToken::lexer(input);
  let mut tokens: Vec<Token> = Vec::new();

  while let Some(token) = lexer.next() {
    match token {
      RawToken::Integer(i) => tokens.push(Token::Value(Value::from(i))),
      RawToken::Float(f) => tokens.push(Token::Value(Value::from(f))),
      RawToken::Identifier => tokens.push(Token::Identifier(lexer.slice().to_string())),
      RawToken::Operator => {
        match lexer.slice() {
          // + and - are both binary and unary operators so look at the previous token
          "+" | "-" => {
            if tokens.is_empty()
              || matches!(tokens.last(), Some(t) if t.is_operator() || t.is_lparen() || t.is_newline())
            {
              tokens.push(Token::Operator(format!("{}u", lexer.slice())));
              continue;
            }
          }
          // both ! and ~ are exclusively unary operators
          "!" | "~" => {
            tokens.push(Token::Operator(format!("{}u", lexer.slice())));
            continue;
          }
          _ => (),
        };

        tokens.push(Token::Operator(lexer.slice().to_string()));
      }
      RawToken::LParen => tokens.push(Token::LParen),
      RawToken::RParen => tokens.push(Token::RParen),
      RawToken::Newline => tokens.push(Token::Newline),
      RawToken::Error => {
        let slice = lexer.slice();
        return Err(format!("Unexpected token in input '{}'", slice));
      }
    }
  }
  return Ok(tokens);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rustfmt::skip]
  macro_rules! u64_t { ($value:literal) => { Token::Value(Value::from($value as u64)) }; }
  #[rustfmt::skip]
  macro_rules! f64_t { ($value:literal) => { Token::Value(Value::from($value as f64)) }; }
  #[rustfmt::skip]
  macro_rules! id_t { ($value:literal) => { Token::Identifier($value.to_string()) }; }
  #[rustfmt::skip]
  macro_rules! op_t { ($value:literal) => { Token::Operator($value.to_string()) }; }

  #[test]
  fn test_tokenize_integer() {
    let input = "0x1F 0o777 0b1101 101";
    let expected = vec![u64_t!(0x1F), u64_t!(0o777), u64_t!(0b1101), u64_t!(101)];

    let tokens = tokenize(input);
    assert_eq!(tokens, Ok(expected));
  }

  #[test]
  fn test_tokenize_float() {
    let input = "3.141 0.0001 2. .5";
    let expected = vec![f64_t!(3.141), f64_t!(0.0001), f64_t!(2.), f64_t!(0.5)];

    let tokens = tokenize(input);
    assert_eq!(tokens, Ok(expected));
  }

  #[test]
  fn test_tokenize_identifier() {
    let input = "sin cos PI U64_MAX";
    let expected = vec![id_t!("sin"), id_t!("cos"), id_t!("PI"), id_t!("U64_MAX")];

    let tokens = tokenize(input);
    assert_eq!(tokens, Ok(expected));
  }

  #[test]
  fn test_tokenize_unary() {
    let input = "+1\n-2 !3 ~4";
    let expected = vec![
      op_t!("+u"),
      u64_t!(1),
      Token::Newline,
      op_t!("-u"),
      u64_t!(2),
      op_t!("!u"),
      u64_t!(3),
      op_t!("~u"),
      u64_t!(4),
    ];

    let tokens = tokenize(input);
    assert_eq!(tokens, Ok(expected));
  }

  #[test]
  fn test_tokenize_binary() {
    let input = "1+2 3-4 5*6 7/8";
    let expected = vec![
      u64_t!(1),
      op_t!("+"),
      u64_t!(2),
      u64_t!(3),
      op_t!("-"),
      u64_t!(4),
      u64_t!(5),
      op_t!("*"),
      u64_t!(6),
      u64_t!(7),
      op_t!("/"),
      u64_t!(8),
    ];

    let tokens = tokenize(input);
    assert_eq!(tokens, Ok(expected));
  }
}
