use crate::value::Value;
use phf::phf_map;

#[rustfmt::skip] macro_rules! constant { ($value:expr) => { || Value::from($value) }; }
#[rustfmt::skip] macro_rules! unary { ($func:expr) => { Function::Unary(|v: Value| Value::from($func(v))) }; }
#[rustfmt::skip] macro_rules! binary { ($func:expr) => { Function::Binary(|v1: Value, v2: Value| Value::from($func(v1, v2))) }; }

#[derive(Copy, Clone, Debug)]
pub enum Function {
  Unary(fn(Value) -> Value),
  Binary(fn(Value, Value) -> Value),
}

//

const CONST_TABLE: phf::Map<&'static str, fn() -> Value> = phf_map! {
  "PI" => constant!(std::f64::consts::PI),
  "E" => constant!(std::f64::consts::E),
  "NAN" => constant!(f64::NAN),
  "INF" => constant!(f64::INFINITY),
  "NEG_INF" => constant!(f64::NEG_INFINITY),

  "F64_MIN" => constant!(f64::MIN),
  "F64_MAX" => constant!(f64::MAX),
  "U64_MIN" => constant!(u64::MIN),
  "U64_MAX" => constant!(u64::MAX),
  "U32_MIN" => constant!(u32::MIN),
  "U32_MAX" => constant!(u32::MAX),
  "U16_MIN" => constant!(u16::MIN),
  "U16_MAX" => constant!(u16::MAX),
  "U8_MIN" => constant!(u8::MIN),
  "U8_MAX" => constant!(u8::MAX),
  "I64_MIN" => constant!(i64::MIN),
  "I64_MAX" => constant!(i64::MAX),
  "I32_MIN" => constant!(i32::MIN),
  "I32_MAX" => constant!(i32::MAX),
  "I16_MIN" => constant!(i16::MIN),
  "I16_MAX" => constant!(i16::MAX),
  "I8_MIN" => constant!(i8::MIN),
  "I8_MAX" => constant!(i8::MAX),
};

const FUNC_TABLE: phf::Map<&'static str, Function> = phf_map! {
  // operators
  "+u" => unary!(|v| v),
  "-u" => unary!(|v| -(v as Value).to_signed()),
  "!u" => unary!(|v| !bool::from(v)),
  "~u" => unary!(|v| !(v as Value)),

  "+" => binary!(|a, b| a + b),
  "-" => binary!(|a, b| (a as Value).to_signed() - (b as Value).to_signed()),
  "*" => binary!(|a, b| a * b),
  "/" => binary!(|a, b| a / b),
  "%" => binary!(|a, b| a % b),

  "&" => binary!(|a, b| a & b),
  "|" => binary!(|a, b| a | b),
  "^" => binary!(|a, b| a ^ b),

  "<<" => binary!(|a, b| a << b),
  ">>" => binary!(|a, b| a >> b),

  "<" => binary!(|a, b| Value::from(a < b)),
  ">" => binary!(|a, b| Value::from(a > b)),
  ">=" => binary!(|a, b| Value::from(a >= b)),
  "<=" => binary!(|a, b| Value::from(a <= b)),
  "==" => binary!(|a, b| Value::from(a == b)),
  "!=" => binary!(|a, b| Value::from(a != b)),

  "&&" => binary!(|a, b| Value::from(bool::from(a) && bool::from(b))),
  "||" => binary!(|a, b| Value::from(bool::from(a) || bool::from(b))),

  // casting
  "u64" => unary!(|v| u64::from(v)),
  "u32" => unary!(|v| u32::from(v)),
  "u16" => unary!(|v| u16::from(v)),
  "u8" => unary!(|v| u8::from(v)),
  "i64" => unary!(|v| i64::from(v)),
  "i32" => unary!(|v| i32::from(v)),
  "i16" => unary!(|v| i16::from(v)),
  "i8" => unary!(|v| i8::from(v)),
  "f64" => unary!(|v| f64::from(v)),

  // unary functions
  "abs" => unary!(|v| v), /* v.abs() */
  "sin" => unary!(|v| f64::from(v).sin()),
  "cos" => unary!(|v| f64::from(v).cos()),
  "tan" => unary!(|v| f64::from(v).tan()),
  "asin" => unary!(|v| f64::from(v).asin()),
  "acos" => unary!(|v| f64::from(v).asin()),
  "atan" => unary!(|v| f64::from(v).asin()),
  "floor" => unary!(|v| f64::from(v).floor()),
  "ceil" => unary!(|v| f64::from(v).ceil()),
  "round" => unary!(|v| f64::from(v).round()),
  "sqrt" => unary!(|v| f64::from(v).sqrt()),
  "exp" => unary!(|v| f64::from(v).exp()),
  "ln" => unary!(|v| f64::from(v).ln()),
  "log2" => unary!(|v| f64::from(v).log2()),
  "log10" => unary!(|v| f64::from(v).log10()),
  "deg" => unary!(|v| f64::from(v) / (std::f64::consts::FRAC_1_PI * 180.0)),
  "rad" => unary!(|v| f64::from(v) * (std::f64::consts::FRAC_1_PI * 180.0)),
};

pub fn get_constant(name: &str) -> Option<Value> {
  CONST_TABLE.get(name).map(|f| f())
}

pub fn get_function(name: &str) -> Option<Function> {
  FUNC_TABLE.get(name).cloned()
}
