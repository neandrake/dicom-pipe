//! Value Length

use std::fmt;

const UNDEFINED_LENGTH: u32 = 0xFFFF_FFFF;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    if vl == UNDEFINED_LENGTH {
        ValueLength::UndefinedLength
    } else {
        ValueLength::Explicit(vl)
    }
}
