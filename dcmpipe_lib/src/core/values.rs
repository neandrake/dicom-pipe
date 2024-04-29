use crate::{core::dcmelement::DicomElement, core::defn::vr::VRRef};

/// Wrapper around `&[u8]` for getting a slice of the element value without the padding values.
#[derive(Debug)]
pub(crate) struct BytesWithoutPadding<'b>(pub &'b [u8]);

/// For parsing an element value as a string with a specific VR.
#[derive(Debug)]
pub struct ElementWithVr<'e>(pub &'e DicomElement, pub VRRef);

/// Wrapper around `u32` for parsing DICOM Attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute(pub u32);

/// Wrapper around an element's value parsed into a native/raw type.
#[derive(Debug, Clone, PartialEq)]
pub enum RawValue {
    Attribute(Vec<Attribute>),
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
}

impl RawValue {
    /// Convenience for an empty value.
    pub fn empty() -> RawValue {
        RawValue::Bytes(Vec::with_capacity(0))
    }

    /// Convenience for `RawValue::Attribute(vec![attr])`
    pub fn attr(attr: Attribute) -> RawValue {
        RawValue::Attribute(vec![attr])
    }

    /// Convenience for `RawValue::Strings(vec![string])`
    pub fn string(string: String) -> RawValue {
        RawValue::Strings(vec![string])
    }

    /// Convenience for `RawValue::Shorts(vec![short])`
    pub fn short(short: i16) -> RawValue {
        RawValue::Shorts(vec![short])
    }

    /// Convenience for `RawValue::UShorts(vec![ushort])`
    pub fn ushort(ushort: u16) -> RawValue {
        RawValue::UShorts(vec![ushort])
    }

    /// Convenience for `RawValue::Ints(vec![int])`
    pub fn int(int: i32) -> RawValue {
        RawValue::Ints(vec![int])
    }

    /// Convenience for `RawValue::UInt(vec![uint])`
    pub fn uint(uint: u32) -> RawValue {
        RawValue::UInts(vec![uint])
    }

    /// Convenience for `RawValue::Longs(vec![long])`
    pub fn long(long: i64) -> RawValue {
        RawValue::Longs(vec![long])
    }

    /// Convenience for `RawValue::ULongs(vec![ulong])`
    pub fn ulong(ulong: u64) -> RawValue {
        RawValue::ULongs(vec![ulong])
    }

    /// Convenience for `RawValue::Floats(vec![float])`
    pub fn float(float: f32) -> RawValue {
        RawValue::Floats(vec![float])
    }

    /// Convenience for `RawValue::Doubles(vec![double])`
    pub fn double(double: f64) -> RawValue {
        RawValue::Doubles(vec![double])
    }

    /// Convenience for `RawValue::Bytes(vec![byte])`
    pub fn byte(byte: u8) -> RawValue {
        RawValue::Bytes(vec![byte])
    }

    /// Convenience for `RawValue::Words(vec![word])`
    pub fn word(word: u16) -> RawValue {
        RawValue::Words(vec![word])
    }

    /// Convenience for `RawValue::DWords(vec![dword])`
    pub fn dword(dword: u32) -> RawValue {
        RawValue::DWords(vec![dword])
    }

    /// Convenience for `RawValue::QWords(vec![qword])`
    pub fn qword(qword: u64) -> RawValue {
        RawValue::QWords(vec![qword])
    }
}
