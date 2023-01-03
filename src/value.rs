pub use crate::number::{Number, Width};
pub use crate::unit::Unit;

/// A value is a number plus a unit.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Value {
  pub number: Number,
  pub unit: Unit,
}

impl Value {
  pub fn new(number: Number, unit: Unit) -> Self {
    let number = Unit::normalize(number, unit);
    Self { number, unit }
  }

  pub fn new_raw(number: Number) -> Self {
    let unit = Unit::Raw;
    Self { number, unit }
  }

  pub const fn new_number(number: Number) -> Self {
    let unit = Unit::Raw;
    Self { number, unit }
  }

  pub const fn new_integer(value: u64, width: Width) -> Self {
    let number = Number::new_integer(value, width);
    let unit = Unit::Raw;
    Self { number, unit }
  }

  pub const fn new_float(value: f64) -> Self {
    let number = Number::new_float(value);
    let unit = Unit::Raw;
    Self { number, unit }
  }

  pub fn is_integer(&self) -> bool {
    self.number.is_integer()
  }

  pub fn is_raw(&self) -> bool {
    self.unit.is_raw()
  }

  pub fn convert(self, unit: Unit) -> Option<Self> {
    let number = Unit::convert(self.number, self.unit, unit)?;
    Some(Self { number, unit })
  }
}

impl Default for Value {
  fn default() -> Self {
    Self::new_integer(0, Width::U64)
  }
}

impl From<Number> for Value {
  fn from(number: Number) -> Self {
    Self::new_number(number)
  }
}

impl From<(Number, Unit)> for Value {
  fn from((number, unit): (Number, Unit)) -> Self {
    Self { number, unit }
  }
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let number = Unit::specialize(self.number, self.unit);
    write!(f, "{}{}", number.as_pretty_string(), self.unit)
  }
}
