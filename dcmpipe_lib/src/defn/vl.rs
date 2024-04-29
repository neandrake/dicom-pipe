//! Value Length

use std::fmt;

/// A sentinel value defined by the DICOM standard indicating that the actual
/// length of the value would be undefined/unspecified.
pub const UNDEFINED_LENGTH: u32 = 0xFFFF_FFFF;

/// Value Length Definition
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueLength {
    UndefinedLength,
    Explicit(u32),
}

impl fmt::Debug for ValueLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ValueLength::Explicit(value_length) => write!(f, "{}", value_length),
            ValueLength::UndefinedLength => write!(f, "UndefinedLength"),
        }
    }
}

/// Creates a value length from the given numeric value.
pub fn from_u32(vl: u32) -> ValueLength {
    if vl == UNDEFINED_LENGTH {
        ValueLength::UndefinedLength
    } else {
        ValueLength::Explicit(vl)
    }
}
