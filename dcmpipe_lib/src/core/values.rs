use crate::{core::dcmelement::DicomElement, defn::vr::VRRef};

/// Wrapper around `&[u8]` for getting a slice of the element value without the padding values.
pub(crate) struct BytesWithoutPadding<'bytes>(pub &'bytes [u8]);

/// For parsing an element value as a string with a specific VR.
pub struct ElementWithVr<'elem>(pub &'elem DicomElement, pub VRRef);

/// Wrapper around `u32` for parsing DICOM Attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute(pub u32);

/// Wrapper around an element's value parsed into a native/raw type.
#[derive(Debug, Clone)]
pub enum RawValue {
    Attribute(Vec<Attribute>),
    Uid(String),
    Strings(Vec<String>),
    Shorts(Vec<i16>),
    UnsignedShorts(Vec<u16>),
    Integers(Vec<i32>),
    UnsignedIntegers(Vec<u32>),
    Longs(Vec<i64>),
    UnsignedLongs(Vec<u64>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    Bytes(Vec<u8>),
    Words(Vec<u16>),
    DoubleWords(Vec<u32>),
    QuadWords(Vec<u64>),
}
