use float_cmp::approx_eq;
use std::cmp::Ordering;
use std::{fmt, fmt::Display};

/// A number that is either a fixed-width integer or a float.
#[derive(Debug, Copy, Clone)]
pub enum Number {
  Integer(u64, Width),
  Float(f64),
}

// macros for working with Number

macro_rules! number_cast {
  ($v: expr, $w: expr, $t: tt) => {
    match $w {
      Width::U64 => ($v as u64) as $t,
      Width::U32 => ($v as u32) as $t,
      Width::U16 => ($v as u16) as $t,
      Width::U8 => ($v as u8) as $t,
      Width::I64 => ($v as i64) as $t,
      Width::I32 => ($v as i32) as $t,
      Width::I16 => ($v as i16) as $t,
      Width::I8 => ($v as i8) as $t,
    }
  };
}

macro_rules! number_fmt {
  ($v: expr, $w: expr, $f: expr) => {
    match $w {
      Width::U64 => format!($f, $v as u64),
      Width::U32 => format!($f, $v as u32),
      Width::U16 => format!($f, $v as u16),
      Width::U8 => format!($f, $v as u8),
      Width::I64 => format!($f, $v as i64),
      Width::I32 => format!($f, $v as i32),
      Width::I16 => format!($f, $v as i16),
      Width::I8 => format!($f, $v as i8),
    }
  };
}

macro_rules! integer_cmp {
  ($v1: expr, $v2: expr, $w: expr) => {
    match $w {
      Width::U64 => (($v1 as u64).cmp(&($v2 as u64))),
      Width::U32 => (($v1 as u32).cmp(&($v2 as u32))),
      Width::U16 => (($v1 as u16).cmp(&($v2 as u16))),
      Width::U8 => (($v1 as u8).cmp(&($v2 as u8))),
      Width::I64 => (($v1 as i64).cmp(&($v2 as i64))),
      Width::I32 => (($v1 as i32).cmp(&($v2 as i32))),
      Width::I16 => (($v1 as i16).cmp(&($v2 as i16))),
      Width::I8 => (($v1 as i8).cmp(&($v2 as i8))),
    }
  };
}

macro_rules! impl_arithmetic_op {
  ($ops: tt, $func: tt, $op: tt) => {
    impl std::ops::$ops<Number> for Number {
      type Output = Number;
      fn $func(self, rhs: Number) -> Number {
        match self {
          Number::Integer(v1, w) => match rhs {
            Number::Integer(v2, _) => Number::new_integer(v1 $op w.mask(v2), w),
            Number::Float(v2) => Number::new_integer(v1 $op w.mask(v2 as u64), w),
          },
          Number::Float(v1) => match rhs {
            Number::Integer(v2, w) => Number::new_float(v1 $op number_cast!(v2, w, f64)),
            Number::Float(v2) => Number::new_float(v1 $op v2),
          },
        }
      }
    }
  };
}

macro_rules! impl_bitwise_op {
  ($ops: tt, $func: tt, $op: tt) => {
    impl std::ops::$ops<Number> for Number {
      type Output = Number;
      fn $func(self, rhs: Number) -> Number {
        match self {
          Number::Integer(v1, w) => match rhs {
            Number::Integer(v2, _) => Number::new_integer(v1 $op v2, w),
            Number::Float(v2) => Number::new_integer(v1 $op w.mask(v2 as u64), w),
          },
          Number::Float(_) => match rhs {
            Number::Integer(_, _) => Number::new_float(f64::NAN),
            Number::Float(_) => Number::new_float(f64::NAN),
          },
        }
      }
    }
  };
}

macro_rules! impl_number_from {
  ($t: tt) => {
    impl From<$t> for Number {
      fn from(v: $t) -> Self {
        Number::new_float(v as f64)
      }
    }
  };
  ($t: tt, $w: expr) => {
    impl From<$t> for Number {
      fn from(v: $t) -> Self {
        Number::new_integer(v as u64, $w)
      }
    }
  };
}

macro_rules! impl_from_number {
  ($t: tt) => {
    impl From<Number> for $t {
      fn from(src: Number) -> $t {
        match src {
          Number::Integer(v, w) => number_cast!(v, w, $t),
          Number::Float(v) => v as $t,
        }
      }
    }
  };
}

// `From` traits
impl_number_from!(u64, Width::U64);
impl_number_from!(u32, Width::U32);
impl_number_from!(u16, Width::U16);
impl_number_from!(u8, Width::U8);

impl_number_from!(i64, Width::I64);
impl_number_from!(i32, Width::I32);
impl_number_from!(i16, Width::I16);
impl_number_from!(i8, Width::I8);

impl_number_from!(bool, Width::U8);
impl_number_from!(f64);

impl_from_number!(u64);
impl_from_number!(u32);
impl_from_number!(u16);
impl_from_number!(u8);

impl_from_number!(i64);
impl_from_number!(i32);
impl_from_number!(i16);
impl_from_number!(i8);

impl_from_number!(f64);

impl From<Number> for bool {
  fn from(src: Number) -> bool {
    match src {
      Number::Integer(v, _) => v != 0,
      Number::Float(v) => v != 0f64,
    }
  }
}

impl_arithmetic_op!(Add, add, +);
impl_arithmetic_op!(Sub, sub, -);
impl_arithmetic_op!(Mul, mul, *);
impl_arithmetic_op!(Div, div, /);
impl_arithmetic_op!(Rem, rem, %);

impl_bitwise_op!(BitAnd, bitand, &);
impl_bitwise_op!(BitOr, bitor, |);
impl_bitwise_op!(BitXor, bitxor, ^);
impl_bitwise_op!(Shl, shl, <<);
impl_bitwise_op!(Shr, shr, >>);

impl std::ops::Neg for Number {
  type Output = Number;
  fn neg(self) -> Number {
    match self {
      Number::Integer(v, w) => Number::new_integer(v.wrapping_neg(), w),
      Number::Float(v) => Number::new_float(-v),
    }
  }
}
impl std::ops::Not for Number {
  type Output = Number;
  fn not(self) -> Number {
    match self {
      Number::Integer(v, w) => Number::new_integer(!v, w),
      Number::Float(v) => Number::new_integer((v != 0f64) as u64, Width::U8),
    }
  }
}

// Comparison traits
impl Eq for Number {}

impl Ord for Number {
  fn cmp(&self, other: &Self) -> Ordering {
    match self {
      Number::Integer(v1, w) => match other {
        Number::Integer(v2, _) => integer_cmp!(*v1, *v2, w),
        Number::Float(v2) => integer_cmp!(*v1, *v2, w),
      },
      Number::Float(v1) => match other {
        Number::Integer(v2, w) => {
          let vf = number_cast!(*v2, w, f64);
          if approx_eq!(f64, *v1, vf) {
            Ordering::Equal
          } else if v1 < &vf {
            Ordering::Less
          } else {
            Ordering::Greater
          }
        }
        Number::Float(v2) => {
          if approx_eq!(f64, *v1, *v2) {
            Ordering::Equal
          } else if v1 < &v2 {
            Ordering::Less
          } else {
            Ordering::Greater
          }
        }
      },
    }
  }
}

impl PartialEq<Number> for Number {
  fn eq(&self, other: &Number) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl PartialOrd<Number> for Number {
  fn partial_cmp(&self, other: &Number) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Number {
  pub const fn new_integer(v: u64, w: Width) -> Number {
    Number::Integer(w.mask(v), w)
  }

  pub const fn new_float(v: f64) -> Number {
    Number::Float(v)
  }

  pub fn is_integer(&self) -> bool {
    matches!(self, Number::Integer(_, _))
  }

  pub fn is_float(&self) -> bool {
    matches!(self, Number::Float(_))
  }

  pub fn abs(&self) -> Number {
    match self {
      Number::Integer(v, w) => match w {
        Width::I64 | Width::I32 | Width::I16 | Width::I8 => {
          if self < &Number::from(0) {
            Number::new_integer(v.wrapping_neg(), *w)
          } else {
            Number::new_integer(*v, *w)
          }
        }
        _ => Number::new_integer(*v, *w),
      },
      Number::Float(v) => Number::new_float(v.abs()),
    }
  }

  pub fn pow(&self, other: &Number) -> Number {
    let exp = u32::from((*other).abs());
    match self {
      Number::Integer(v, w) => match w {
        Width::U64 => Number::new_integer(v.pow(exp), Width::U64),
        Width::U32 => Number::new_integer((*v as u32).pow(exp) as u64, Width::U32),
        Width::U16 => Number::new_integer((*v as u16).pow(exp) as u64, Width::U16),
        Width::U8 => Number::new_integer((*v as u8).pow(exp) as u64, Width::U8),
        Width::I64 => Number::new_integer((*v as i64).pow(exp) as u64, Width::I64),
        Width::I32 => Number::new_integer((*v as i32).pow(exp) as u64, Width::I32),
        Width::I16 => Number::new_integer((*v as i16).pow(exp) as u64, Width::I16),
        Width::I8 => Number::new_integer((*v as i8).pow(exp) as u64, Width::I8),
      },
      Number::Float(v) => Number::new_float(v.powf(f64::from(*other))),
    }
  }

  pub fn to_signed(&self) -> Number {
    use Width::*;
    match self {
      Number::Integer(v, w) => match w {
        U64 => Number::new_integer(*v as i64 as u64, I64),
        U32 => Number::new_integer(*v as i32 as u64, I32),
        U16 => Number::new_integer(*v as i16 as u64, I16),
        U8 => Number::new_integer(*v as i8 as u64, I8),
        _ => Number::new_integer(*v, *w),
      },
      Number::Float(v) => Number::new_integer(*v as i64 as u64, I64),
    }
  }

  pub fn to_unsigned(&self) -> Number {
    use Width::*;
    match self {
      Number::Integer(v, w) => match w {
        I64 | I32 | I16 | I8 => Number::new_integer(number_cast!(*v, w, u64) * v, *w),
        _ => Number::new_integer(*v, *w),
      },
      Number::Float(v) => Number::new_integer(*v as u64, U64),
    }
  }

  pub fn to_float(&self) -> Number {
    match self {
      Number::Integer(v, w) => Number::new_float(number_cast!(*v, w, f64)),
      Number::Float(v) => Number::new_float(*v),
    }
  }

  pub fn to_width(&self, w: Width) -> Number {
    match self {
      Number::Integer(v, _) => Number::new_integer(number_cast!(*v, w, u64), w),
      Number::Float(v) => Number::new_integer(number_cast!(*v, w, u64), w),
    }
  }

  pub fn as_pretty_string(&self) -> String {
    match self {
      Number::Integer(v, w) => number_fmt!(*v, w, "{}"),
      Number::Float(v) => {
        if v.fract() == 0.0 {
          format!("{}", *v)
        } else {
          format!("{:.2}", *v)
        }
      }
    }
  }

  pub fn as_string(&self) -> String {
    match self {
      Number::Integer(v, w) => number_fmt!(*v, w, "{}"),
      Number::Float(v) => format!("{}", v),
    }
  }
}

impl Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Binary for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Number::Integer(v, w) => write!(f, "{}", number_fmt!(*v, w, "{:#b}")),
      Number::Float(v) => write!(f, "{}", v), // no binary for floats
    }
  }
}

impl std::fmt::Octal for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Number::Integer(v, w) => write!(f, "{}", number_fmt!(*v, w, "{:#o}")),
      Number::Float(v) => write!(f, "{}", v), // no octal for floats
    }
  }
}

impl std::fmt::LowerHex for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Number::Integer(v, w) => write!(f, "{}", number_fmt!(*v, w, "{:#x}")),
      Number::Float(v) => write!(f, "{}", v), // no hex for floats
    }
  }
}

//

#[derive(Debug, Clone, Copy)]
pub enum Width {
  U64,
  U32,
  U16,
  U8,

  I64,
  I32,
  I16,
  I8,
}

impl Width {
  pub const fn mask(&self, value: u64) -> u64 {
    value & self.as_mask()
  }

  pub const fn as_mask(&self) -> u64 {
    use Width::*;
    match self {
      U64 => 0xFFFFFFFFFFFFFFFF,
      U32 => 0xFFFFFFFF,
      U16 => 0xFFFF,
      U8 => 0xFF,

      I64 => 0xFFFFFFFFFFFFFFFF,
      I32 => 0xFFFFFFFF,
      I16 => 0xFFFF,
      I8 => 0xFF,
    }
  }

  pub fn as_string(&self) -> &str {
    use Width::*;
    match self {
      U64 => "u64",
      U32 => "u32",
      U16 => "u16",
      U8 => "u8",

      I64 => "i64",
      I32 => "i32",
      I16 => "i16",
      I8 => "i8",
    }
  }
}

impl Display for Width {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

// // Alfred workflow xml output
// fn generate_alfred_output(number: Number) -> String {
//   match number {
//     Number::Integer(_, _) => {
//       let expr = number.as_format_string(Format::Default);
//       let expr_hex = number.as_format_string(Format::Hex);
//       let expr_oct = number.as_format_string(Format::Octal);
//       let expr_bin = number.as_format_string(Format::Binary);
//       format!(
//         "
// <?xml version=\"1.0\"?>
// <items>
//   <item arg=\"{expr}\" valid=\"YES\" autocomplete=\"{expr}\" type=\"default\">
//     <title>{expr}</title>
//     <subtitle>copy+paste as \"{expr}\"</subtitle>
//     <mod key=\"shift\" subtitle=\"copy+paste as &quot;{expr}&quot;\" valid=\"yes\" arg=\"{expr}\"/>
//     <icon>dec.png</icon>
//   </item>
//   <item arg=\"{hexstr}\" valid=\"YES\" autocomplete=\"{hexstr}\" type=\"default\">
//     <title>{hexstr}</title>
//     <subtitle>copy+paste as \"{hexstr}\"</subtitle>
//     <mod key=\"shift\" subtitle=\"copy+paste as &quot;{hexstr}&quot;\" valid=\"yes\" arg=\"{hexstr}\"/>
//     <icon>hex.png</icon>
//   </item>
//   <item arg=\"{octstr}\" valid=\"YES\" autocomplete=\"{octstr}\" type=\"default\">
//     <title>{octstr}</title>
//     <subtitle>copy+paste as \"{octstr}\"</subtitle>
//     <mod key=\"shift\" subtitle=\"copy+paste as &quot;{octstr}&quot;\" valid=\"yes\" arg=\"{octstr}\"/>
//     <icon>oct.png</icon>
//   </item>
//   <item arg=\"{binstr}\" valid=\"YES\" autocomplete=\"{binstr}\" type=\"default\">
//     <title>{binstr}</title>
//     <subtitle>copy+paste as \"{binstr}\"</subtitle>
//     <mod key=\"shift\" subtitle=\"copy+paste as &quot;{binstr}&quot;\" valid=\"yes\" arg=\"{binstr}\"/>
//     <icon>bin.png</icon>
//   </item>
// </items>",
//         expr = expr,
//         hexstr = expr_hex,
//         octstr = expr_oct,
//         binstr = expr_bin,
//       )
//     }
//     Number::Float(v) => {
//       format!(
//         "\
// <?xml version=\"1.0\"?>
// <items>
//   <item arg=\"{expr}\" valid=\"YES\" autocomplete=\"{expr}\" type=\"default\">
//     <title>{expr}</title>
//     <subtitle>copy+paste as \"{expr}\"</subtitle>
//     <mod key=\"shift\" subtitle=\"copy+paste as &quot;{expr}&quot;\" valid=\"yes\" arg=\"{expr}\"/>
//     <icon>dec.png</icon>
//   </item>
// </items>",
//         expr = v,
//       )
//     }
//   }
// }
