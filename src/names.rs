use crate::value::{Value};
use phf::phf_map;

#[derive(Debug, Clone)]
pub enum Name {
  Constant(fn() -> Value),
  Function(fn(Value) -> Value)
}

use Name::*;
use crate::value::Value::Float;

const IDENTIFIER_TABLE: phf::Map<&'static str, Name> = phf_map! {
  // Constants
  "PI" => Constant(|| Value::from(std::f64::consts::PI)),
  "E" => Constant(|| Value::from(std::f64::consts::E)),
  // Functions
  "sin" => Function(|v| Value::from(f64::from(v).sin())),
  "cos" => Function(|v| Value::from(f64::from(v).cos())),
  "tan" => Function(|v| Value::from(f64::from(v).tan())),
};

//

pub fn get_name(name: &str) -> Option<Name> {
  let res = IDENTIFIER_TABLE.get(name)?;
  return Some(res.clone());
}
