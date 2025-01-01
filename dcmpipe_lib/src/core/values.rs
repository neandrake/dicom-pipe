/*
   Copyright 2024-2025 Christopher Speck

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

use crate::{core::dcmelement::DicomElement, core::defn::vr::VRRef};

/// Wrapper around `&[u8]` for getting a slice of the element value without the padding values.
#[derive(Debug)]
pub(crate) struct BytesWithoutPadding<'b>(pub &'b [u8]);

/// For parsing an element value as a string with a specific VR.
#[derive(Debug, Clone)]
pub struct ElementWithVr<'e>(pub &'e DicomElement, pub VRRef);

impl<'e> ElementWithVr<'e> {
    #[must_use]
    pub fn of(elem: &'e DicomElement) -> Self {
        ElementWithVr(elem, elem.vr())
    }
}

/// Wrapper around `u32` for parsing DICOM Attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute(pub u32);

/// Wrapper around an element's value parsed into a native/raw type.
#[derive(Debug, Clone, PartialEq)]
pub enum RawValue<'e> {
    Attributes(Vec<Attribute>),
    Uid(String),
    Strings(Vec<String>),
    Shorts(Vec<i16>),
    UShorts(Vec<u16>),
    Ints(Vec<i32>),
    UInts(Vec<u32>),
    Longs(Vec<i64>),
    ULongs(Vec<u64>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    Bytes(Vec<u8>),
    Words(Vec<u16>),
    DWords(Vec<u32>),
    QWords(Vec<u64>),

    BytesView(&'e Vec<u8>),
}

impl<'e> RawValue<'e> {
    /// Convenience for an empty value.
    #[must_use]
    pub fn empty() -> RawValue<'e> {
        RawValue::Bytes(Vec::with_capacity(0))
    }

    /// Convenience for `RawValue::Attribute(vec![attr])`
    #[must_use]
    pub fn of_attr(attr: Attribute) -> RawValue<'e> {
        RawValue::Attributes(vec![attr])
    }

    /// Convenience for `RawValue::Uid(String)`
    #[must_use]
    pub fn of_uid<S>(string: S) -> RawValue<'e>
    where
        String: From<S>,
    {
        RawValue::Uid(String::from(string))
    }

    /// Convenience for `RawValue::Strings(vec![string])`
    #[must_use]
    pub fn of_string<S>(string: S) -> RawValue<'e>
    where
        String: From<S>,
    {
        RawValue::Strings(vec![String::from(string)])
    }

    /// Convenience for `RawValue::Shorts(vec![short])`
    #[must_use]
    pub fn of_short(short: i16) -> RawValue<'e> {
        RawValue::Shorts(vec![short])
    }

    /// Convenience for `RawValue::UShorts(vec![ushort])`
    #[must_use]
    pub fn of_ushort(ushort: u16) -> RawValue<'e> {
        RawValue::UShorts(vec![ushort])
    }

    /// Convenience for `RawValue::Ints(vec![int])`
    #[must_use]
    pub fn of_int(int: i32) -> RawValue<'e> {
        RawValue::Ints(vec![int])
    }

    /// Convenience for `RawValue::UInt(vec![uint])`
    #[must_use]
    pub fn of_uint(uint: u32) -> RawValue<'e> {
        RawValue::UInts(vec![uint])
    }

    /// Convenience for `RawValue::Longs(vec![long])`
    #[must_use]
    pub fn of_long(long: i64) -> RawValue<'e> {
        RawValue::Longs(vec![long])
    }

    /// Convenience for `RawValue::ULongs(vec![ulong])`
    #[must_use]
    pub fn of_ulong(ulong: u64) -> RawValue<'e> {
        RawValue::ULongs(vec![ulong])
    }

    /// Convenience for `RawValue::Floats(vec![float])`
    #[must_use]
    pub fn of_float(float: f32) -> RawValue<'e> {
        RawValue::Floats(vec![float])
    }

    /// Convenience for `RawValue::Doubles(vec![double])`
    #[must_use]
    pub fn of_double(double: f64) -> RawValue<'e> {
        RawValue::Doubles(vec![double])
    }

    /// Convenience for `RawValue::Bytes(vec![byte])`
    #[must_use]
    pub fn of_byte(byte: u8) -> RawValue<'e> {
        RawValue::Bytes(vec![byte])
    }

    /// Convenience for `RawValue::Words(vec![word])`
    #[must_use]
    pub fn of_word(word: u16) -> RawValue<'e> {
        RawValue::Words(vec![word])
    }

    /// Convenience for `RawValue::DWords(vec![dword])`
    #[must_use]
    pub fn of_dword(dword: u32) -> RawValue<'e> {
        RawValue::DWords(vec![dword])
    }

    /// Convenience for `RawValue::QWords(vec![qword])`
    #[must_use]
    pub fn of_qword(qword: u64) -> RawValue<'e> {
        RawValue::QWords(vec![qword])
    }

    #[must_use]
    pub fn attr(&self) -> Option<&Attribute> {
        match self {
            RawValue::Attributes(attrs) if !attrs.is_empty() => Some(&attrs[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn string(&self) -> Option<&String> {
        match self {
            RawValue::Strings(strings) if !strings.is_empty() => Some(&strings[0]),
            RawValue::Uid(uid) => Some(uid),
            _ => None,
        }
    }

    #[must_use]
    pub fn short(&self) -> Option<i16> {
        match self {
            RawValue::Shorts(shorts) if !shorts.is_empty() => Some(shorts[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn ushort(&self) -> Option<u16> {
        match self {
            RawValue::UShorts(ushorts) if !ushorts.is_empty() => Some(ushorts[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn int(&self) -> Option<i32> {
        match self {
            RawValue::Ints(ints) if !ints.is_empty() => Some(ints[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn uint(&self) -> Option<u32> {
        match self {
            RawValue::UInts(uints) if !uints.is_empty() => Some(uints[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn long(&self) -> Option<i64> {
        match self {
            RawValue::Longs(longs) if !longs.is_empty() => Some(longs[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn ulong(&self) -> Option<u64> {
        match self {
            RawValue::ULongs(ulongs) if !ulongs.is_empty() => Some(ulongs[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn float(&self) -> Option<f32> {
        match self {
            RawValue::Floats(floats) if !floats.is_empty() => Some(floats[0]),
            _ => None,
        }
    }

    #[must_use]
    pub fn double(&self) -> Option<f64> {
        match self {
            RawValue::Doubles(doubles) if !doubles.is_empty() => Some(doubles[0]),
            _ => None,
        }
    }
}
