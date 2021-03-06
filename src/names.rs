use crate::value::Value;
use phf::phf_map;

#[derive(Debug, Clone)]
pub enum Name {
  Constant(fn() -> Value),
  Function(fn(Value) -> Value),
}

use Name::*;

const IDENTIFIER_TABLE: phf::Map<&'static str, Name> = phf_map! {
  // Constants
  "PI" => Constant(|| Value::from(std::f64::consts::PI)),
  "E" => Constant(|| Value::from(std::f64::consts::E)),
  "NAN" => Constant(|| Value::from(f64::NAN)),
  "INF" => Constant(|| Value::from(f64::INFINITY)),
  "NEG_INF" => Constant(|| Value::from(f64::NEG_INFINITY)),

  "MIN_F64" => Constant(|| Value::from(f64::MIN)),
  "MAX_F64" => Constant(|| Value::from(f64::MAX)),
  "MIN_U64" => Constant(|| Value::from(u64::MIN)),
  "MAX_U64" => Constant(|| Value::from(u64::MAX)),
  "MIN_U32" => Constant(|| Value::from(u32::MIN)),
  "MAX_U32" => Constant(|| Value::from(u32::MAX)),
  "MIN_U16" => Constant(|| Value::from(u16::MIN)),
  "MAX_U16" => Constant(|| Value::from(u16::MAX)),
  "MIN_U8" => Constant(|| Value::from(u8::MIN)),
  "MAX_U8" => Constant(|| Value::from(u8::MAX)),
  "MIN_I64" => Constant(|| Value::from(i64::MIN)),
  "MAX_I64" => Constant(|| Value::from(i64::MAX)),
  "MIN_I32" => Constant(|| Value::from(i32::MIN)),
  "MAX_I32" => Constant(|| Value::from(i32::MAX)),
  "MIN_I16" => Constant(|| Value::from(i16::MIN)),
  "MAX_I16" => Constant(|| Value::from(i16::MAX)),
  "MIN_I8" => Constant(|| Value::from(i8::MIN)),
  "MAX_I8" => Constant(|| Value::from(i8::MAX)),

  // Functions
  "abs" => Function(|v| v.abs()),
  "sin" => Function(|v| Value::from(f64::from(v).sin())),
  "cos" => Function(|v| Value::from(f64::from(v).cos())),
  "tan" => Function(|v| Value::from(f64::from(v).tan())),
  "asin" => Function(|v| Value::from(f64::from(v).asin())),
  "acos" => Function(|v| Value::from(f64::from(v).asin())),
  "atan" => Function(|v| Value::from(f64::from(v).asin())),
  "floor" => Function(|v| Value::from(f64::from(v).floor())),
  "ceil" => Function(|v| Value::from(f64::from(v).ceil())),
  "round" => Function(|v| Value::from(f64::from(v).round())),
  "sqrt" => Function(|v| Value::from(f64::from(v).sqrt())),
  "exp" => Function(|v| Value::from(f64::from(v).exp())),
  "ln" => Function(|v| Value::from(f64::from(v).ln())),
  "log2" => Function(|v| Value::from(f64::from(v).log2())),
  "log10" => Function(|v| Value::from(f64::from(v).log10())),
  "deg" => Function(|v| Value::from(f64::from(v) / (std::f64::consts::FRAC_1_PI * 180.0))),

  // Casting
  "u64" => Function(|v| Value::from(u64::from(v))),
  "u32" => Function(|v| Value::from(u32::from(v))),
  "u16" => Function(|v| Value::from(u16::from(v))),
  "u8" => Function(|v| Value::from(u8::from(v))),
  "i64" => Function(|v| Value::from(i64::from(v))),
  "i32" => Function(|v| Value::from(i32::from(v))),
  "i16" => Function(|v| Value::from(i16::from(v))),
  "i8" => Function(|v| Value::from(i8::from(v))),
  "f64" => Function(|v| Value::from(f64::from(v))),
};

//

pub fn get_name(name: &str) -> Option<Name> {
  // f64::from(4).asin()
  let res = IDENTIFIER_TABLE.get(name)?;
  return Some(res.clone());
}
