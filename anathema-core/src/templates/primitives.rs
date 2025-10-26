use std::fmt::Display;

use anathema_state::Hex;

/// Literal values in templates.
///
/// Compile-time constants: `true`, `'a'`, `123`, `3.14`, `#ff0000`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Primitive {
    /// Boolean.
    Bool(bool),
    /// Character.
    Char(char),
    /// 64-bit signed integer.
    Int(i64),
    /// 64-bit float.
    Float(f64),
    /// RGB color.
    Hex(Hex),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{v}"),
            Self::Char(v) => write!(f, "{v}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Hex(Hex { r, g, b }) => write!(f, "r: {r}, g: {g}, b: {b}"),
        }
    }
}

macro_rules! from_value {
    ($from_type:tt, $variant:ident) => {
        impl From<$from_type> for Primitive {
            fn from(value: $from_type) -> Self {
                Self::$variant(value)
            }
        }
    };
}

from_value!(f64, Float);
from_value!(i64, Int);
from_value!(bool, Bool);
from_value!(char, Char);
from_value!(Hex, Hex);

impl From<(u8, u8, u8)> for Primitive {
    fn from(value: (u8, u8, u8)) -> Self {
        let (r, g, b) = value;
        Self::Hex(Hex { r, g, b })
    }
}
