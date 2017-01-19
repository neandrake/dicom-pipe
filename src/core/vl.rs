//! Value Length

use std::fmt;

pub const IMPLICIT_VL_VALUE: u32 = 0xFFFFFFFF;

pub enum ValueLength {
    UndefinedLength,
    Explicit(u32),
}

impl fmt::Debug for ValueLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValueLength::Explicit(value_length) => write!(f, "{}", value_length),
            ValueLength::UndefinedLength => write!(f, "UndefinedLength"),
        }
    }
}

pub fn from_value_length(vl: u32) -> ValueLength {
    if vl == IMPLICIT_VL_VALUE {
        ValueLength::UndefinedLength
    } else {
        ValueLength::Explicit(vl)
    }
}
