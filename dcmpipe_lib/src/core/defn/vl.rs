/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

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

impl From<u32> for ValueLength {
    fn from(value: u32) -> Self {
        if value == UNDEFINED_LENGTH {
            ValueLength::UndefinedLength
        } else {
            ValueLength::Explicit(value)
        }
    }
}

impl fmt::Debug for ValueLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ValueLength::Explicit(value_length) => write!(f, "{value_length}"),
            ValueLength::UndefinedLength => write!(f, "UndefinedLength"),
        }
    }
}
