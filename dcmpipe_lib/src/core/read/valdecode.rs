//! This module contains implementations for decodign values from a DICOM element's value field
//! bytes, based on the element's value representation and transfer syntax.

use crate::{
    core::{
        dcmelement::DicomElement,
        read::{ParseError, ParseResult},
        values::{Attribute, BytesWithoutPadding, ElementWithVr, RawValue},
    },
    defn::{
        tag::Tag,
        vr::{self, VRRef, CS_SEPARATOR},
    },
};

use super::error::ParseErrorInfo;

pub(crate) const I16_SIZE: usize = std::mem::size_of::<i16>();
pub(crate) const U16_SIZE: usize = std::mem::size_of::<u16>();
pub(crate) const I32_SIZE: usize = std::mem::size_of::<i32>();
pub(crate) const U32_SIZE: usize = std::mem::size_of::<u32>();
pub(crate) const I64_SIZE: usize = std::mem::size_of::<i64>();
pub(crate) const U64_SIZE: usize = std::mem::size_of::<u64>();
pub(crate) const F32_SIZE: usize = std::mem::size_of::<f32>();
pub(crate) const F64_SIZE: usize = std::mem::size_of::<f64>();

impl TryFrom<&DicomElement> for RawValue {
    type Error = ParseError;

    /// Based on the VR of this element, parses the binary data into a RawValue.
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_data().is_empty() {
            Ok(RawValue::Bytes(Vec::with_capacity(0)))
        } else if value.get_vr() == &vr::AT {
            Ok(RawValue::Attribute(Vec::<Attribute>::try_from(value)?))
        } else if value.get_vr() == &vr::UI {
            Ok(RawValue::Uid(String::try_from(value)?))
        } else if value.get_vr() == &vr::SS {
            Ok(RawValue::Shorts(Vec::<i16>::try_from(value)?))
        } else if value.get_vr() == &vr::US {
            Ok(RawValue::UnsignedShorts(Vec::<u16>::try_from(value)?))
        } else if value.get_vr() == &vr::SL {
            Ok(RawValue::Integers(Vec::<i32>::try_from(value)?))
        } else if value.get_vr() == &vr::UL {
            Ok(RawValue::UnsignedIntegers(Vec::<u32>::try_from(value)?))
        } else if value.get_vr() == &vr::SV {
            Ok(RawValue::Longs(Vec::<i64>::try_from(value)?))
        } else if value.get_vr() == &vr::UV {
            Ok(RawValue::UnsignedLongs(Vec::<u64>::try_from(value)?))
        } else if value.get_vr() == &vr::OW {
            Ok(RawValue::Words(Vec::<u16>::try_from(value)?))
        } else if value.get_vr() == &vr::OL {
            Ok(RawValue::DoubleWords(Vec::<u32>::try_from(value)?))
        } else if value.get_vr() == &vr::OV {
            Ok(RawValue::QuadWords(Vec::<u64>::try_from(value)?))
        } else if value.get_vr() == &vr::IS {
            let possible_i32s = Vec::<i32>::try_from(value);
            if let Ok(i32s) = possible_i32s {
                Ok(RawValue::Integers(i32s))
            } else {
                // Sometimes decimal values are (incorrectly) encoded with VR of IS.
                Ok(RawValue::Doubles(Vec::<f64>::try_from(value)?))
            }
        } else if value.get_vr() == &vr::OF || value.get_vr() == &vr::FL {
            Ok(RawValue::Floats(Vec::<f32>::try_from(value)?))
        } else if value.get_vr() == &vr::DS {
            Ok(RawValue::Doubles(Vec::<f64>::try_from(value)?))
        } else if value.get_vr() == &vr::OD || value.get_vr() == &vr::FD {
            Ok(RawValue::Doubles(Vec::<f64>::try_from(value)?))
        } else if value.get_vr().is_character_string {
            // Check is_character_string last, as that will be true for a number of VRs above which
            // whose try_from will attempt to parse stringified numeric values into native values.
            Ok(RawValue::Strings(Vec::<String>::try_from(value)?))
        } else if value.get_vr() == &vr::UN && Tag::is_private_creator(value.get_tag()) {
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_data().len() % U32_SIZE != 0 {
            return Err(ParseErrorInfo(value, "value is not a multiple of 4 bytes", None).into());
        }

        let num_attrs = value.get_data().len() / U32_SIZE;
        let mut buf: [u8; 2] = [0; 2];

        let mut attrs: Vec<Attribute> = Vec::with_capacity(num_attrs);
        for i in 0..num_attrs {
            let idx = i * U32_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + 2)]);
            let first: u32 = if value.get_ts().is_big_endian() {
                u32::from(u16::from_be_bytes(buf)) << 16
            } else {
                u32::from(u16::from_le_bytes(buf)) << 16
            };

            buf.copy_from_slice(&value.get_data()[(idx + 2)..(idx + 4)]);
            let second: u32 = if value.get_ts().is_big_endian() {
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        String::try_from(ElementWithVr(value, value.get_vr()))
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
    fn try_from(value: ElementWithVr<'_>) -> ParseResult<Self> {
        let element: &DicomElement = value.0;
        let data: &[u8] = BytesWithoutPadding::from(value).0;
        element
            .get_cs()
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        Vec::<String>::try_from(ElementWithVr(value, value.get_vr()))
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
    fn try_from(value: ElementWithVr<'_>) -> ParseResult<Self> {
        let element: &DicomElement = value.0;
        let vr: VRRef = value.1;
        let data: &[u8] = BytesWithoutPadding::from(value).0;
        element
            .get_cs()
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

        let data: &[u8] = value.0.get_data().as_slice();
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

impl TryFrom<&DicomElement> for i16 {
    type Error = ParseError;

    /// Parses the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        Vec::<i16>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| ParseErrorInfo(value, "no i16's parsed", None).into())
    }
}

impl TryFrom<&DicomElement> for Vec<i16> {
    type Error = ParseError;

    /// Parses the value for this element as a list of signed 16bit integer values
    /// Associated VRs: SS
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeShorts = Vec<ParseResult<i16>>;
            let (values, errors): (MaybeShorts, MaybeShorts) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<i16>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<i16> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<i16>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % I16_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of i16", None).into(),
            );
        }

        let mut buf: [u8; I16_SIZE] = [0; I16_SIZE];
        let num_i16s: usize = num_bytes / I16_SIZE;
        let mut result: Vec<i16> = Vec::with_capacity(num_i16s);
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        for i in 0..num_i16s {
            let idx = i * I16_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + I16_SIZE)]);
            let val: i16 = if value.get_ts().is_big_endian() {
                i16::from_be_bytes(buf)
            } else {
                i16::from_le_bytes(buf)
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        Vec::<u16>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| ParseErrorInfo(value, "no u16's parsed", None).into())
    }
}

impl TryFrom<&DicomElement> for Vec<u16> {
    type Error = ParseError;

    /// Parses the value for this element as a list of unsigned 16bit integer values
    /// Associated VRs: US, OW
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeUshorts = Vec<ParseResult<u16>>;
            let (values, errors): (MaybeUshorts, MaybeUshorts) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<u16>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<u16> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<u16>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % U16_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of u16", None).into(),
            );
        }

        let mut buf: [u8; U16_SIZE] = [0; U16_SIZE];
        let num_u16s: usize = num_bytes / U16_SIZE;
        let mut result: Vec<u16> = Vec::with_capacity(num_u16s);
        for i in 0..num_u16s {
            let idx = i * U16_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + U16_SIZE)]);
            let val: u16 = if value.get_ts().is_big_endian() {
                u16::from_be_bytes(buf)
            } else {
                u16::from_le_bytes(buf)
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        Vec::<i32>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| ParseErrorInfo(value, "no i32's parsed", None).into())
    }
}

impl TryFrom<&DicomElement> for Vec<i32> {
    type Error = ParseError;

    /// Parses the value for this element as a list of signed 32bit integer values
    /// Associated VRs: IS, SL
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeInts = Vec<ParseResult<i32>>;
            let (values, errors): (MaybeInts, MaybeInts) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<i32>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<i32> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<i32>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % I32_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of i32", None).into(),
            );
        }

        let mut buf: [u8; I32_SIZE] = [0; I32_SIZE];
        let num_i32s: usize = num_bytes / I32_SIZE;
        let mut result: Vec<i32> = Vec::with_capacity(num_i32s);
        for i in 0..num_i32s {
            let idx = i * I32_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + I32_SIZE)]);
            let val: i32 = if value.get_ts().is_big_endian() {
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        Vec::<u32>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| ParseErrorInfo(value, "no u32's parsed", None).into())
    }
}

impl TryFrom<&DicomElement> for Vec<u32> {
    type Error = ParseError;

    /// Parses the value for this element as a list of unsigned 32bit integer values
    /// Associated VRs: UL, OL
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeUints = Vec<ParseResult<u32>>;
            let (values, errors): (MaybeUints, MaybeUints) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<u32>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<u32> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<u32>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % U32_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of u32", None).into(),
            );
        }

        let mut buf: [u8; U32_SIZE] = [0; U32_SIZE];
        let num_u32s: usize = num_bytes / U32_SIZE;
        let mut result: Vec<u32> = Vec::with_capacity(num_u32s);
        for i in 0..num_u32s {
            let idx = i * U32_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + U32_SIZE)]);
            let val: u32 = if value.get_ts().is_big_endian() {
                u32::from_be_bytes(buf)
            } else {
                u32::from_le_bytes(buf)
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeLongs = Vec<ParseResult<i64>>;
            let (values, errors): (MaybeLongs, MaybeLongs) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<i64>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<i64> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<i64>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % I64_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of i64", None).into(),
            );
        }

        let mut buf: [u8; I64_SIZE] = [0; I64_SIZE];
        let num_i64s: usize = num_bytes / I64_SIZE;
        let mut result: Vec<i64> = Vec::with_capacity(num_i64s);
        for i in 0..num_i64s {
            let idx = i * I64_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + I64_SIZE)]);
            let val: i64 = if value.get_ts().is_big_endian() {
                i64::from_be_bytes(buf)
            } else {
                i64::from_le_bytes(buf)
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeUlongs = Vec<ParseResult<u64>>;
            let (values, errors): (MaybeUlongs, MaybeUlongs) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<u64>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<u64> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<u64>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % U64_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of u64", None).into(),
            );
        }

        let mut buf: [u8; U64_SIZE] = [0; U64_SIZE];
        let num_u64s: usize = num_bytes / U64_SIZE;
        let mut result: Vec<u64> = Vec::with_capacity(num_u64s);
        for i in 0..num_u64s {
            let idx = i * U64_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + U64_SIZE)]);
            let val: u64 = if value.get_ts().is_big_endian() {
                u64::from_be_bytes(buf)
            } else {
                u64::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}

impl TryFrom<&DicomElement> for f32 {
    type Error = ParseError;

    /// Parses the value for this element as a 32bit floating point
    /// Associated VRs: FL
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        Vec::<f32>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| ParseErrorInfo(value, "no f32's parsed", None).into())
    }
}

impl TryFrom<&DicomElement> for Vec<f32> {
    type Error = ParseError;

    /// Parses the value for this element as a list of 32bit floating point values
    /// Associated VRs: FD, OF
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeFloats = Vec<ParseResult<f32>>;
            let (values, errors): (MaybeFloats, MaybeFloats) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<f32>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<f32> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<f32>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % F32_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of f32", None).into(),
            );
        }

        let mut buf: [u8; F32_SIZE] = [0; F32_SIZE];
        let num_bytes: usize = value.get_data().len();
        let num_f32s: usize = num_bytes / F32_SIZE;
        let mut result: Vec<f32> = Vec::with_capacity(num_f32s);
        for i in 0..num_f32s {
            let idx = i * F32_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + F32_SIZE)]);
            let val: f32 = if value.get_ts().is_big_endian() {
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
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        Vec::<f64>::try_from(value)?
            .into_iter()
            .next()
            .ok_or_else(|| ParseErrorInfo(value, "no f64's parsed", None).into())
    }
}

impl TryFrom<&DicomElement> for Vec<f64> {
    type Error = ParseError;

    /// Parses the value for this element as a list of 64bit floating point values
    /// Associated VRs: DS, OD, FL -- and a fallback for IS.
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.get_vr().is_character_string {
            type MaybeDoubles = Vec<ParseResult<f64>>;
            let (values, errors): (MaybeDoubles, MaybeDoubles) = Vec::<String>::try_from(value)?
                .into_iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<f64>()
                        .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
                })
                .partition(ParseResult::is_ok);
            if let Some(Err(e)) = errors.into_iter().last() {
                return Err(e);
            }
            let values: Vec<f64> = values
                .into_iter()
                .map(ParseResult::unwrap)
                .collect::<Vec<f64>>();
            return Ok(values);
        }

        let num_bytes: usize = value.get_data().len();
        if num_bytes == 0 {
            return Ok(Vec::with_capacity(0));
        }
        if num_bytes % F64_SIZE != 0 {
            return Err(
                ParseErrorInfo(value, "num bytes not multiple of size of f64", None).into(),
            );
        }

        let mut buf: [u8; F64_SIZE] = [0; F64_SIZE];
        let num_f64s: usize = num_bytes / F64_SIZE;
        let mut result: Vec<f64> = Vec::with_capacity(num_f64s);
        for i in 0..num_f64s {
            let idx = i * F64_SIZE;
            buf.copy_from_slice(&value.get_data()[idx..(idx + F64_SIZE)]);
            let val: f64 = if value.get_ts().is_big_endian() {
                f64::from_be_bytes(buf)
            } else {
                f64::from_le_bytes(buf)
            };
            result.push(val);
        }
        Ok(result)
    }
}
