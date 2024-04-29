//! DICOM Element Definition and Value Parsing

use std::convert::TryFrom;
use std::fmt;
use std::iter::once;

use crate::core::charset::CSRef;
use crate::core::dcmsqelem::SequenceElement;
use crate::core::read;
use crate::core::read::error::ParseError;
use crate::core::read::parser::Result;
use crate::defn::constants::tags;
use crate::defn::tag::{Tag, TagNode, TagPath};
use crate::defn::ts::TSRef;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef, CS_SEPARATOR};

use super::charset::DEFAULT_CHARACTER_SET;

const I16_SIZE: usize = std::mem::size_of::<i16>();
const U16_SIZE: usize = std::mem::size_of::<u16>();
const I32_SIZE: usize = std::mem::size_of::<i32>();
const U32_SIZE: usize = std::mem::size_of::<u32>();
const I64_SIZE: usize = std::mem::size_of::<i64>();
const U64_SIZE: usize = std::mem::size_of::<u64>();
const F32_SIZE: usize = std::mem::size_of::<f32>();
const F64_SIZE: usize = std::mem::size_of::<f64>();

const MAX_BYTES_IN_ERROR: usize = 16;

/// Wrapper around `&[u8]` for getting a slice of the element value without the padding values
struct BytesWithoutPadding<'bytes>(&'bytes [u8]);

/// For parsing an element value as a string with a specific VR
pub struct ElementWithVr<'elem>(pub &'elem DicomElement, pub VRRef);

/// Wrapper around `u32` for parsing DICOM Attributes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute(pub u32);

/// Wrapper around an element's value parsed into a native/raw type
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

fn error(message: &str, value: &DicomElement) -> ParseError {
    // TODO: How to get a dicom dictionary here for better error messages?
    let tagstring = TagPath::format_tagpath_to_display(&value.get_tagpath(), None);
    ParseError::ValueParseError {
        message: message.to_owned(),
        tagstring,
        vr: value.vr,
        cs: value.cs,
        bytes: value
            .data
            .iter()
            .take(MAX_BYTES_IN_ERROR)
            .cloned()
            .collect::<Vec<u8>>(),
    }
}

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
pub struct DicomElement {
    tag: u32,
    vr: VRRef,
    vl: ValueLength,

    data: Vec<u8>,
    sq_path: Vec<SequenceElement>,

    ts: TSRef,
    cs: CSRef,
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: VR[{:?}], VL[{:?}], TS[{:?}]",
            Tag::format_tag_to_display(self.get_tag()),
            self.get_vr(),
            self.get_vl(),
            self.get_ts().get_uid().get_ident()
        )
    }
}

impl DicomElement {
    pub fn new<T>(
        tag: T,
        vr: VRRef,
        vl: ValueLength,
        ts: TSRef,
        cs: CSRef,
        data: Vec<u8>,
        sq_path: Vec<SequenceElement>,
    ) -> Self
    where
        T: Into<u32>,
    {
        let cs: CSRef = vr.get_proper_cs(cs);
        DicomElement {
            tag: Into::<u32>::into(tag),
            vr,
            vl,
            data,
            sq_path,
            ts,
            cs,
        }
    }

    pub fn new_empty<T>(tag: T, vr: VRRef, ts: TSRef) -> Self
    where
        T: Into<u32>,
    {
        let cs: CSRef = vr.get_proper_cs(DEFAULT_CHARACTER_SET);
        DicomElement {
            tag: Into::<u32>::into(tag),
            vr,
            vl: ValueLength::UndefinedLength,
            data: Vec::with_capacity(0),
            sq_path: Vec::with_capacity(0),
            ts,
            cs,
        }
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_vl(&self) -> ValueLength {
        self.vl
    }

    pub fn get_vr(&self) -> VRRef {
        self.vr
    }

    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    pub fn get_cs(&self) -> CSRef {
        self.cs
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_sequence_path(&self) -> &Vec<SequenceElement> {
        &self.sq_path
    }

    /// Returns if this element is a `SQ` or if it should be parsed as though it were a sequence
    pub fn is_seq_like(&self) -> bool {
        self.vr == &vr::SQ || read::util::is_non_standard_seq(self.tag, self.vr, self.vl)
    }

    /// Returns whether the the size of the value for this element is zero
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Creates a `TagPath` for the current element
    pub fn get_tagpath(&self) -> TagPath {
        self.sq_path
            .iter()
            .filter(|sq| sq.get_seq_tag() != tags::ITEM)
            .map(|sq| sq.get_node().clone())
            .chain(once(self.tag.into()))
            .collect::<Vec<TagNode>>()
            .into()
    }

    /// Parses this element's data into native/raw value type.
    pub fn parse_value(&self) -> Result<RawValue> {
        RawValue::try_from(self)
    }

    /// Encodes a RawValue into the binary data for this element.
    ///
    /// This will overwrite any existing value in this element in `self.data`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be encoded according to `self.vr`.
    /// * `vl` - The value length to use. If `None` then the value length will be computed and
    /// `ValueLength::Explicit` will be assigned to `self.vl`. If `Some` then it will only be used
    /// if `self.is_seq_like()` would return true, otherwise the value length will be computed and
    /// `ValueLength::Explicit` will be assigned to `self.vl`. Unconditionally, `self.vl` will be
    /// assigned `ValueLength::Explicit(0)` if this element is `Item`, `ItemDelimitationItem`, or
    /// `SequenceDelimitationItem`.
    pub fn encode_value(&mut self, value: RawValue, vl: Option<ValueLength>) -> Result<()> {
        let mut bytes: Vec<u8> = match value {
            RawValue::Attribute(attrs) => {
                let num_attrs = attrs.len();
                let mut bytes: Vec<u8> = vec![0u8; U32_SIZE * num_attrs];
                for (i, attr) in attrs.iter().enumerate() {
                    let Attribute(attr) = attr;
                    let group_number: u16 = ((attr >> 16) & 0xFFFF) as u16;
                    let elem_number: u16 = (attr & 0xFFFF) as u16;
                    let idx = i * U32_SIZE;
                    if self.ts.is_big_endian() {
                        bytes[idx..(idx + 2)].copy_from_slice(&group_number.to_be_bytes());
                        bytes[(idx + 2)..(idx + 4)].copy_from_slice(&elem_number.to_be_bytes());
                    } else {
                        bytes[idx..(idx + 2)].copy_from_slice(&group_number.to_le_bytes());
                        bytes[(idx + 2)..(idx + 4)].copy_from_slice(&elem_number.to_le_bytes());
                    }
                }
                bytes
            }
            RawValue::Uid(uid) => self
                .cs
                .encode(&uid)
                .map_err(|e| ParseError::CharsetError { source: e })?,
            RawValue::Strings(strings) => {
                type MaybeBytes = Vec<Result<Vec<u8>>>;
                let (values, errs): (MaybeBytes, MaybeBytes) = strings
                    .iter()
                    .map(|s| {
                        self.cs
                            .encode(s)
                            // Add the separator after each encoded value. Below the last separator
                            // will be popped off.
                            .map(|mut v| {
                                v.push(CS_SEPARATOR as u8);
                                v
                            })
                            .map_err(|e| ParseError::CharsetError { source: e })
                    })
                    .partition(Result::is_ok);

                if let Some(Err(e)) = errs.into_iter().last() {
                    return Err(e);
                }

                // Flatten the bytes for all strings.
                let mut bytes: Vec<u8> = values
                    .into_iter()
                    .flat_map(Result::unwrap)
                    .collect::<Vec<u8>>();
                // Remove last separator.
                bytes.pop();
                bytes
            }
            RawValue::Shorts(shorts) => {
                if self.vr.is_character_string {
                    // This should only be the case for a VR of IS.
                    let mut encoded = shorts
                        .into_iter()
                        // In theory this should use the default character set, but this
                        // relies on i16::to_string only using ascii which falls under that.
                        .map(|short: i16| short.to_string().into_bytes())
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of SS
                    shorts
                        .into_iter()
                        .flat_map(|short: i16| {
                            if self.ts.is_big_endian() {
                                short.to_be_bytes()
                            } else {
                                short.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::UnsignedShorts(ushorts) => {
                if self.vr.is_character_string {
                    // This should only be the case for a VR of IS.
                    let mut encoded = ushorts
                        .into_iter()
                        // In theory this should use the default character set, but this
                        // relies on u16::to_string only using ascii which falls under that.
                        .map(|ushort: u16| ushort.to_string().into_bytes())
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of US
                    ushorts
                        .into_iter()
                        .flat_map(|ushort: u16| {
                            if self.ts.is_big_endian() {
                                ushort.to_be_bytes()
                            } else {
                                ushort.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::Integers(ints) => {
                if self.vr.is_character_string {
                    // This should only be the case for a VR of IS.
                    let mut encoded = ints
                        .into_iter()
                        // In theory this should use the default character set, but this
                        // relies on i32::to_string only using ascii which falls under that.
                        .map(|int: i32| int.to_string().into_bytes())
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of SL.
                    ints.into_iter()
                        .flat_map(|int: i32| {
                            if self.ts.is_big_endian() {
                                int.to_be_bytes()
                            } else {
                                int.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::UnsignedIntegers(uints) => {
                if self.vr.is_character_string {
                    // XXX: This shouldn't happen. Unsigned integers should only ever be encoded
                    // as binary.
                    let mut encoded = uints
                        .into_iter()
                        // In theory this should use the default character set, but this
                        // relies on i16::to_string only using ascii which falls under that.
                        .map(|uint: u32| uint.to_string().into_bytes())
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of UL.
                    uints
                        .into_iter()
                        .flat_map(|uint: u32| {
                            if self.ts.is_big_endian() {
                                uint.to_be_bytes()
                            } else {
                                uint.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::Longs(longs) => {
                if self.vr.is_character_string {
                    // This should only be the case for a VR of IS.
                    let mut encoded = longs
                        .into_iter()
                        // In theory this should use the default character set, but this
                        // relies on i32::to_string only using ascii which falls under that.
                        .map(|long: i64| long.to_string().into_bytes())
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of SL.
                    longs
                        .into_iter()
                        .flat_map(|long: i64| {
                            if self.ts.is_big_endian() {
                                long.to_be_bytes()
                            } else {
                                long.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::UnsignedLongs(ulongs) => {
                if self.vr.is_character_string {
                    // XXX: This shouldn't happen. Unsigned integers should only ever be encoded
                    // as binary.
                    let mut encoded = ulongs
                        .into_iter()
                        // In theory this should use the default character set, but this
                        // relies on i16::to_string only using ascii which falls under that.
                        .map(|ulong: u64| ulong.to_string().into_bytes())
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of UL.
                    ulongs
                        .into_iter()
                        .flat_map(|ulong: u64| {
                            if self.ts.is_big_endian() {
                                ulong.to_be_bytes()
                            } else {
                                ulong.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::Floats(floats) => {
                if self.vr.is_character_string {
                    // This should only be the case for a VR of DS.
                    let mut encoded = floats
                        .into_iter()
                        .filter(|float: &f32| float.is_finite())
                        // In theory this should use the default character set, but this
                        // relies on f32::to_string only using ascii which falls under that.
                        .map(|float: f32| {
                            // Force at least one digit of precision.
                            if float.fract() == 0.0 {
                                format!("{float:.1}").into_bytes()
                            } else {
                                float.to_string().into_bytes()
                            }
                        })
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of FL.
                    floats
                        .into_iter()
                        .filter(|float: &f32| float.is_finite())
                        .flat_map(|float: f32| {
                            if self.ts.is_big_endian() {
                                float.to_be_bytes()
                            } else {
                                float.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::Doubles(doubles) => {
                if self.vr.is_character_string {
                    // This should only be the case for a VR of DS.
                    let mut encoded = doubles
                        .into_iter()
                        .filter(|double: &f64| double.is_finite())
                        // In theory this should use the default character set, but this
                        // relies on f64::to_string only using ascii which falls under that.
                        .map(|double: f64| {
                            // Force at least one digit of precision.
                            if double.fract() == 0.0 {
                                format!("{double:.1}").into_bytes()
                            } else {
                                double.to_string().into_bytes()
                            }
                        })
                        .flat_map(|v| v.into_iter().chain(once(vr::CS_SEPARATOR_BYTE)))
                        .collect::<Vec<u8>>();
                    if !encoded.is_empty() {
                        encoded.remove(encoded.len() - 1);
                    }
                    encoded
                } else {
                    // This should only be the case for a VR of FL.
                    doubles
                        .into_iter()
                        .filter(|double: &f64| double.is_finite())
                        .flat_map(|double: f64| {
                            if self.ts.is_big_endian() {
                                double.to_be_bytes()
                            } else {
                                double.to_le_bytes()
                            }
                        })
                        .collect::<Vec<u8>>()
                }
            }
            RawValue::Bytes(bytes) => bytes,
            RawValue::Words(words) => words
                .into_iter()
                .map(|word| {
                    if self.ts.is_big_endian() {
                        word.to_be_bytes()
                    } else {
                        word.to_le_bytes()
                    }
                })
                .flatten()
                .collect::<Vec<u8>>(),
            RawValue::DoubleWords(dwords) => dwords
                .into_iter()
                .map(|dword| {
                    if self.ts.is_big_endian() {
                        dword.to_be_bytes()
                    } else {
                        dword.to_le_bytes()
                    }
                })
                .flatten()
                .collect::<Vec<u8>>(),
            RawValue::QuadWords(qwords) => qwords
                .into_iter()
                .map(|qword| {
                    if self.ts.is_big_endian() {
                        qword.to_be_bytes()
                    } else {
                        qword.to_le_bytes()
                    }
                })
                .flatten()
                .collect::<Vec<u8>>(),
        };

        // All fields are required to be of even length, with padding added as necessary. Note
        // that the standard refers to values of "character string" however binary values are
        // expected to always result in even number of bytes.
        //
        // Part 5, Ch 6.4:
        // Each string Value in a multiple valued character string may be of even or odd length,
        // but the length of the entire Value Field (including "\" delimiters) shall be of even
        // length. If padding is required to make the Value Field of even length, a single padding
        // character shall be applied to the end of the Value Field (to the last Value), in which
        // case the length of the last Value may exceed the Length of Value by 1.
        if bytes.len() % 2 != 0 {
            bytes.push(self.vr.padding);
        }

        self.data = bytes;

        self.vl = if vl.is_some() && self.is_seq_like() || self.tag == tags::ITEM {
            vl.unwrap()
        } else if self.tag == tags::ITEM_DELIMITATION_ITEM
            || self.tag == tags::SEQUENCE_DELIMITATION_ITEM
        {
            ValueLength::Explicit(0)
        } else {
            ValueLength::Explicit(self.data.len() as u32)
        };

        Ok(())
    }
}

impl TryFrom<&DicomElement> for RawValue {
    type Error = ParseError;

    /// Based on the VR of this element, parses the binary data into a RawValue.
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.get_data().is_empty() {
            Ok(RawValue::Bytes(Vec::with_capacity(0)))
        } else if value.vr == &vr::AT {
            Ok(RawValue::Attribute(Vec::<Attribute>::try_from(value)?))
        } else if value.vr == &vr::UI {
            Ok(RawValue::Uid(String::try_from(value)?))
        } else if value.vr == &vr::SS {
            Ok(RawValue::Shorts(Vec::<i16>::try_from(value)?))
        } else if value.vr == &vr::US {
            Ok(RawValue::UnsignedShorts(Vec::<u16>::try_from(value)?))
        } else if value.vr == &vr::SL {
            Ok(RawValue::Integers(Vec::<i32>::try_from(value)?))
        } else if value.vr == &vr::UL {
            Ok(RawValue::UnsignedIntegers(Vec::<u32>::try_from(value)?))
        } else if value.vr == &vr::SV {
            Ok(RawValue::Longs(Vec::<i64>::try_from(value)?))
        } else if value.vr == &vr::UV {
            Ok(RawValue::UnsignedLongs(Vec::<u64>::try_from(value)?))
        } else if value.vr == &vr::OW {
            Ok(RawValue::Words(Vec::<u16>::try_from(value)?))
        } else if value.vr == &vr::OL {
            Ok(RawValue::DoubleWords(Vec::<u32>::try_from(value)?))
        } else if value.vr == &vr::OV {
            Ok(RawValue::QuadWords(Vec::<u64>::try_from(value)?))
        } else if value.vr == &vr::IS {
            let possible_i32s = Vec::<i32>::try_from(value);
            if let Ok(i32s) = possible_i32s {
                Ok(RawValue::Integers(i32s))
            } else {
                // Sometimes decimal values are (incorrectly) encoded with VR of IS.
                Ok(RawValue::Doubles(Vec::<f64>::try_from(value)?))
            }
        } else if value.vr == &vr::OF || value.vr == &vr::FL {
            Ok(RawValue::Floats(Vec::<f32>::try_from(value)?))
        } else if value.vr == &vr::DS {
            Ok(RawValue::Doubles(Vec::<f64>::try_from(value)?))
        } else if value.vr == &vr::OD || value.vr == &vr::FD {
            Ok(RawValue::Doubles(Vec::<f64>::try_from(value)?))
        } else if value.vr.is_character_string {
            // Check is_character_string last, as that will be true for a number of VRs above which
            // whose try_from will attempt to parse stringified numeric values into native values.
            Ok(RawValue::Strings(Vec::<String>::try_from(value)?))
        } else if value.vr == &vr::UN && Tag::is_private_creator(value.get_tag()) {
            // See Part 5 Section 6.2.2
            // Some dicom datasets seem to explicitly encode their private creator UIDs with VR of UN
            // and in the case of Implicit VR the private tag will also not be known/lookup.
            let possible_uid = String::try_from(value);
            if let Ok(uid) = possible_uid {
                Ok(RawValue::Uid(uid))
            } else {
                Ok(RawValue::Bytes(value.get_data().clone()))
            }
        } else {
            Ok(RawValue::Bytes(value.get_data().clone()))
        }
    }
}

impl TryFrom<&DicomElement> for Vec<Attribute> {
    type Error = ParseError;

    /// Parses the value for this element as an attribute (aka a tag)
    /// Associated VRs: AT
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.data.len() % U32_SIZE != 0 {
            return Err(error("value is not a multiple of 4 bytes", value));
        }

        let num_attrs = value.data.len() / U32_SIZE;
        let mut buf: [u8; 2] = [0; 2];

        let mut attrs: Vec<Attribute> = Vec::with_capacity(num_attrs);
        for i in 0..num_attrs {
            let idx = i * U32_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + 2)]);
            let first: u32 = if value.ts.is_big_endian() {
                u32::from(u16::from_be_bytes(buf)) << 16
            } else {
                u32::from(u16::from_le_bytes(buf)) << 16
            };

            buf.copy_from_slice(&value.data[(idx + 2)..(idx + 4)]);
            let second: u32 = if value.ts.is_big_endian() {
                u32::from(u16::from_be_bytes(buf))
            } else {
                u32::from(u16::from_le_bytes(buf))
            };
            let result: u32 = first + second;
            attrs.push(Attribute(result));
        }
        Ok(attrs)
    }
}

impl TryFrom<&DicomElement> for String {
    type Error = ParseError;

    /// Parses the value of this element as a string using the element's encoding and VR
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: &DicomElement) -> Result<Self> {
        String::try_from(ElementWithVr(value, value.vr))
    }
}

impl<'elem> TryFrom<ElementWithVr<'elem>> for String {
    type Error = ParseError;

    /// Parses the value of this element as a string using the element's encoding and the specified
    /// VR. This can be used in the event the parser of the element did not have an appropriate
    /// tag dictionary for resolving implicit VRs.
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: ElementWithVr<'_>) -> Result<Self> {
        let element: &DicomElement = value.0;
        let data: &[u8] = BytesWithoutPadding::from(value).0;
        element
            .cs
            .decode(data)
            .map_err(|e| ParseError::CharsetError { source: e })
    }
}

impl TryFrom<&DicomElement> for Vec<String> {
    type Error = ParseError;

    /// Parses the value of this element as a list of strings using the element's encoding and vr.
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: &DicomElement) -> Result<Self> {
        Vec::<String>::try_from(ElementWithVr(value, value.vr))
    }
}

impl<'elem> TryFrom<ElementWithVr<'elem>> for Vec<String> {
    type Error = ParseError;

    /// Parses the value of this element as a list of strings using the element's encoding and the
    /// specified VR. This can be used in the event the parser of the element did not have an appropriate
    /// tag dictionary for resolving implicit VRs.
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: ElementWithVr<'_>) -> Result<Self> {
        let element: &DicomElement = value.0;
        let vr: VRRef = value.1;
        let data: &[u8] = BytesWithoutPadding::from(value).0;
        element
            .cs
            .decode(data)
            .map_err(|e| ParseError::CharsetError { source: e })
            .map(|multivalue: String| {
                if !vr.allows_backslash_text_value {
                    multivalue
                        .split(CS_SEPARATOR)
                        .map(str::to_owned)
                        .collect::<Vec<String>>()
                } else {
                    vec![multivalue]
                }
            })
    }
}

impl<'elem> From<ElementWithVr<'elem>> for BytesWithoutPadding<'elem> {
    /// Returns the value as a slice with the padding character
    /// removed per the specification of whether the VR indicates leading/trailing
    /// padding is significant.
    fn from(value: ElementWithVr<'elem>) -> Self {
        // grab the position to start reading bytes from prior to computing the new bytes_read
        let mut lindex: usize = 0;

        let data: &[u8] = value.0.data.as_slice();
        if data.is_empty() {
            return BytesWithoutPadding(data);
        }

        let mut rindex: usize = data.len() - 1;
        if value.1.can_pad_end {
            if value.1.padding == vr::SPACE_PADDING {
                // space padding should strip all trailing spaces
                while rindex > lindex {
                    // character string vr's also sometimes seem to be zero-padded
                    if data[rindex] == vr::SPACE_PADDING || data[rindex] == vr::NULL_PADDING {
                        rindex -= 1;
                    } else {
                        break;
                    }
                }
            } else if value.1.padding == vr::NULL_PADDING {
                // null byte padding is only singular and only if used to achieve even length
                if data.len() % 2 == 0 && data[rindex] == vr::NULL_PADDING {
                    rindex -= 1;
                }
            }
        }

        if value.1.can_pad_front {
            // space padding should strip all leading spaces
            if value.1.padding == vr::SPACE_PADDING {
                while lindex < rindex {
                    if data[lindex] == vr::SPACE_PADDING {
                        lindex += 1;
                    } else {
                        break;
                    }
                }
            }
            // no such thing as leading padding of null bytes
        }

        // if a character string is trimmed down to just a null byte then return empty data.
        // use `..` to return empty slice instead of `..=`.
        if lindex == rindex && value.1.is_character_string && data[lindex] == vr::NULL_PADDING {
            BytesWithoutPadding(&data[lindex..rindex])
        } else {
            BytesWithoutPadding(&data[lindex..=rindex])
        }
    }
}

impl TryFrom<&DicomElement> for f32 {
    type Error = ParseError;

    /// Parses the value for this element as a 32bit floating point
    /// Associated VRs: FL
    fn try_from(value: &DicomElement) -> Result<Self> {
        Vec::<f32>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| error("no f32's parsed", value))
    }
}

impl TryFrom<&DicomElement> for Vec<f32> {
    type Error = ParseError;

    /// Parses the value for this element as a list of 32bit floating point values
    /// Associated VRs: FD, OF
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeFloats = Vec<Result<f32>>;
            let (values, errors): (MaybeFloats, MaybeFloats) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<f32>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<f32> = values.into_iter().map(Result::unwrap).collect::<Vec<f32>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % F32_SIZE != 0 {
            return Err(error("num bytes not multiple of size of f32", value));
        }

        let mut buf: [u8; F32_SIZE] = [0; F32_SIZE];
        let num_bytes: usize = value.data.len();
        let num_f32s: usize = num_bytes / F32_SIZE;
        let mut result: Vec<f32> = Vec::with_capacity(num_f32s);
        for i in 0..num_f32s {
            let idx = i * F32_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + F32_SIZE)]);
            let val: f32 = if value.ts.is_big_endian() {
                f32::from_be_bytes(buf)
            } else {
                f32::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for f64 {
    type Error = ParseError;

    /// Parses the value for this element as a 64bit floating point
    /// Associated VRs: FD
    fn try_from(value: &DicomElement) -> Result<Self> {
        Vec::<f64>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| error("no f64's parsed", value))
    }
}

impl TryFrom<&DicomElement> for Vec<f64> {
    type Error = ParseError;

    /// Parses the value for this element as a list of 64bit floating point values
    /// Associated VRs: DS, OD, FL -- and a fallback for IS.
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeDoubles = Vec<Result<f64>>;
            let (values, errors): (MaybeDoubles, MaybeDoubles) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<f64>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<f64> = values.into_iter().map(Result::unwrap).collect::<Vec<f64>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % F64_SIZE != 0 {
            return Err(error("num bytes not multiple of size of f64", value));
        }

        let mut buf: [u8; F64_SIZE] = [0; F64_SIZE];
        let num_f64s: usize = num_bytes / F64_SIZE;
        let mut result: Vec<f64> = Vec::with_capacity(num_f64s);
        for i in 0..num_f64s {
            let idx = i * F64_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + F64_SIZE)]);
            let val: f64 = if value.ts.is_big_endian() {
                f64::from_be_bytes(buf)
            } else {
                f64::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for i16 {
    type Error = ParseError;

    /// Parses the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    fn try_from(value: &DicomElement) -> Result<Self> {
        Vec::<i16>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| error("no i16's parsed", value))
    }
}

impl TryFrom<&DicomElement> for Vec<i16> {
    type Error = ParseError;

    /// Parses the value for this element as a list of signed 16bit integer values
    /// Associated VRs: SS
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeShorts = Vec<Result<i16>>;
            let (values, errors): (MaybeShorts, MaybeShorts) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<i16>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<i16> = values.into_iter().map(Result::unwrap).collect::<Vec<i16>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % I16_SIZE != 0 {
            return Err(error("num bytes not multiple of size of i16", value));
        }

        let mut buf: [u8; I16_SIZE] = [0; I16_SIZE];
        let num_i16s: usize = num_bytes / I16_SIZE;
        let mut result: Vec<i16> = Vec::with_capacity(num_i16s);
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        for i in 0..num_i16s {
            let idx = i * I16_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + I16_SIZE)]);
            let val: i16 = if value.ts.is_big_endian() {
                i16::from_be_bytes(buf)
            } else {
                i16::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for i32 {
    type Error = ParseError;

    /// Parses the value for this element as a signed 32bit integer
    /// Associated VRs: SL
    fn try_from(value: &DicomElement) -> Result<Self> {
        Vec::<i32>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| error("no i32's parsed", value))
    }
}

impl TryFrom<&DicomElement> for Vec<i32> {
    type Error = ParseError;

    /// Parses the value for this element as a list of signed 32bit integer values
    /// Associated VRs: IS, SL
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeInts = Vec<Result<i32>>;
            let (values, errors): (MaybeInts, MaybeInts) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<i32>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<i32> = values.into_iter().map(Result::unwrap).collect::<Vec<i32>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % I32_SIZE != 0 {
            return Err(error("num bytes not multiple of size of i32", value));
        }

        let mut buf: [u8; I32_SIZE] = [0; I32_SIZE];
        let num_i32s: usize = num_bytes / I32_SIZE;
        let mut result: Vec<i32> = Vec::with_capacity(num_i32s);
        for i in 0..num_i32s {
            let idx = i * I32_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + I32_SIZE)]);
            let val: i32 = if value.ts.is_big_endian() {
                i32::from_be_bytes(buf)
            } else {
                i32::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for u32 {
    type Error = ParseError;

    /// Parses the value for this element as an unsigned 32bit integer
    /// Associated VRs: UL
    fn try_from(value: &DicomElement) -> Result<Self> {
        Vec::<u32>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| error("no u32's parsed", value))
    }
}

impl TryFrom<&DicomElement> for Vec<u32> {
    type Error = ParseError;

    /// Parses the value for this element as a list of unsigned 32bit integer values
    /// Associated VRs: UL, OL
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeUints = Vec<Result<u32>>;
            let (values, errors): (MaybeUints, MaybeUints) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<u32>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<u32> = values.into_iter().map(Result::unwrap).collect::<Vec<u32>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % U32_SIZE != 0 {
            return Err(error("num bytes not multiple of size of u32", value));
        }

        let mut buf: [u8; U32_SIZE] = [0; U32_SIZE];
        let num_u32s: usize = num_bytes / U32_SIZE;
        let mut result: Vec<u32> = Vec::with_capacity(num_u32s);
        for i in 0..num_u32s {
            let idx = i * U32_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + U32_SIZE)]);
            let val: u32 = if value.ts.is_big_endian() {
                u32::from_be_bytes(buf)
            } else {
                u32::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for Vec<u64> {
    type Error = ParseError;

    /// Parses the value for this element as a list of unsigned 64bit integer values
    /// Associated VRs: UV
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeUlongs = Vec<Result<u64>>;
            let (values, errors): (MaybeUlongs, MaybeUlongs) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<u64>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<u64> = values.into_iter().map(Result::unwrap).collect::<Vec<u64>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % U64_SIZE != 0 {
            return Err(error("num bytes not multiple of size of u64", value));
        }

        let mut buf: [u8; U64_SIZE] = [0; U64_SIZE];
        let num_u64s: usize = num_bytes / U64_SIZE;
        let mut result: Vec<u64> = Vec::with_capacity(num_u64s);
        for i in 0..num_u64s {
            let idx = i * U64_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + U64_SIZE)]);
            let val: u64 = if value.ts.is_big_endian() {
                u64::from_be_bytes(buf)
            } else {
                u64::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for Vec<i64> {
    type Error = ParseError;

    /// Parses the value for this element as a list of signed 64bit integer values
    /// Associated VRs: SV
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeLongs = Vec<Result<i64>>;
            let (values, errors): (MaybeLongs, MaybeLongs) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<i64>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<i64> = values.into_iter().map(Result::unwrap).collect::<Vec<i64>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % I64_SIZE != 0 {
            return Err(error("num bytes not multiple of size of i64", value));
        }

        let mut buf: [u8; I64_SIZE] = [0; I64_SIZE];
        let num_i64s: usize = num_bytes / I64_SIZE;
        let mut result: Vec<i64> = Vec::with_capacity(num_i64s);
        for i in 0..num_i64s {
            let idx = i * I64_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + I64_SIZE)]);
            let val: i64 = if value.ts.is_big_endian() {
                i64::from_be_bytes(buf)
            } else {
                i64::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for u16 {
    type Error = ParseError;

    /// Parses the value for this element as an unsigned 16bit integer
    /// Associated VRs: US
    fn try_from(value: &DicomElement) -> Result<Self> {
        Vec::<u16>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| error("no u16's parsed", value))
    }
}

impl TryFrom<&DicomElement> for Vec<u16> {
    type Error = ParseError;

    /// Parses the value for this element as a list of unsigned 16bit integer values
    /// Associated VRs: US, OW
    fn try_from(value: &DicomElement) -> Result<Self> {
        if value.vr.is_character_string {
            type MaybeUshorts = Vec<Result<u16>>;
            let (values, errors): (MaybeUshorts, MaybeUshorts) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<u16>()
                        .map_err(|e| error(&e.to_string(), value))
                })
                .partition(Result::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<u16> = values.into_iter().map(Result::unwrap).collect::<Vec<u16>>();
            return Ok(values);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % U16_SIZE != 0 {
            return Err(error("num bytes not multiple of size of u16", value));
        }

        let mut buf: [u8; U16_SIZE] = [0; U16_SIZE];
        let num_u16s: usize = num_bytes / U16_SIZE;
        let mut result: Vec<u16> = Vec::with_capacity(num_u16s);
        for i in 0..num_u16s {
            let idx = i * U16_SIZE;
            buf.copy_from_slice(&value.data[idx..(idx + U16_SIZE)]);
            let val: u16 = if value.ts.is_big_endian() {
                u16::from_be_bytes(buf)
            } else {
                u16::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}
