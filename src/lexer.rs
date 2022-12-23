use crate::value::Value;
use logos::{Lexer, Logos};
use std::str::FromStr;

/// A final token produced by the lexer.
#[derive(Debug, Clone)]
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

/// Raw tokens produced by the lexer.
#[derive(Logos, Clone, Debug, PartialEq)]
pub enum RawToken {
  // eg. 101, 0x1F, 0o777, 0b1101
  #[regex(r"0x[0-9a-fA-F]+|0o[0-7]+|0b[01]|[0-9]+", conv_integer)]
  Integer(u64),
  // eg. 3.141, 0.0001, 2., .5
  #[regex(r"\d+\.\d*|\.\d+", conv_float)]
  Float(f64),

  // eg. sin, cos, PI
  #[regex(r"[a-zA-Z]+")]
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
        // distinguish between unary and binary operators by looking at the previous token
        match lexer.slice() {
          "+" | "-" | "!" | "~" => {
            if tokens.is_empty() || matches!(tokens.last(), Some(t) if t.is_operator() || t.is_lparen()) {
              tokens.push(Token::Operator(format!("{}u", lexer.slice())));
              continue;
            }
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
        let slice_bytes = slice.as_bytes();
        return Err(format!("Unexpected token in input '{}' [{:?}]", slice, slice_bytes));
      }
    }
  }
  return Ok(tokens);
}

//

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
