//! This module contains implementations for decodign values from a DICOM element's value field
//! bytes, based on the element's value representation and transfer syntax.

use std::{any::type_name, fmt::Display, mem::size_of, str::FromStr};

use crate::core::{
    dcmelement::DicomElement,
    defn::{
        tag::Tag,
        vr::{
            VRRef, AT, CS_SEPARATOR, DS, FD, FL, IS, NULL_PADDING, OD, OF, OL, OV, OW, SL,
            SPACE_PADDING, SS, SV, UI, UL, UN, US, UV,
        },
    },
    read::{ParseError, ParseResult},
    values::{Attribute, BytesWithoutPadding, ElementWithVr, RawValue},
};

use super::error::ParseErrorInfo;

impl<'e> TryFrom<ElementWithVr<'e>> for RawValue<'e> {
    type Error = ParseError;

    fn try_from(value: ElementWithVr<'e>) -> Result<Self, Self::Error> {
        let elem = value.0;
        let vr = value.1;
        if elem.data().is_empty() {
            Ok(RawValue::Bytes(Vec::with_capacity(0)))
        } else if vr == &AT {
            Ok(RawValue::Attributes(Vec::<Attribute>::try_from(elem)?))
        } else if vr == &UI {
            Ok(RawValue::Uid(String::try_from(elem)?))
        } else if vr == &SS {
            Ok(RawValue::Shorts(Vec::<i16>::try_from(elem)?))
        } else if vr == &US {
            Ok(RawValue::UShorts(Vec::<u16>::try_from(elem)?))
        } else if vr == &SL {
            Ok(RawValue::Ints(Vec::<i32>::try_from(elem)?))
        } else if vr == &UL {
            Ok(RawValue::UInts(Vec::<u32>::try_from(elem)?))
        } else if vr == &SV {
            Ok(RawValue::Longs(Vec::<i64>::try_from(elem)?))
        } else if vr == &UV {
            Ok(RawValue::ULongs(Vec::<u64>::try_from(elem)?))
        } else if vr == &OW {
            Ok(RawValue::Words(Vec::<u16>::try_from(elem)?))
        } else if vr == &OL {
            Ok(RawValue::DWords(Vec::<u32>::try_from(elem)?))
        } else if vr == &OV {
            Ok(RawValue::QWords(Vec::<u64>::try_from(elem)?))
        } else if vr == &IS {
            let possible_i32s = Vec::<i32>::try_from(elem);
            if let Ok(i32s) = possible_i32s {
                Ok(RawValue::Ints(i32s))
            } else {
                // Sometimes decimal elems are (incorrectly) encoded with VR of IS.
                Ok(RawValue::Doubles(Vec::<f64>::try_from(elem)?))
            }
        } else if vr == &OF || vr == &FL {
            Ok(RawValue::Floats(Vec::<f32>::try_from(elem)?))
        } else if vr == &DS || vr == &OD || vr == &FD {
            Ok(RawValue::Doubles(Vec::<f64>::try_from(elem)?))
        } else if vr.is_character_string {
            // Check is_character_string last, as that will be true for a number of VRs above which
            // whose try_from will attempt to parse stringified numeric elems into native values.
            Ok(RawValue::Strings(Vec::<String>::try_from(elem)?))
        } else if vr == &UN && Tag::is_private_creator(elem.tag()) {
            // See Part 5 Section 6.2.2
            // Some dicom datasets seem to explicitly encode their private creator UIDs with VR of UN
            // and in the case of Implicit VR the private tag will also not be known/lookup.
            let possible_uid = String::try_from(elem);
            if let Ok(uid) = possible_uid {
                Ok(RawValue::Uid(uid))
            } else {
                Ok(RawValue::BytesView(elem.data()))
            }
        } else {
            Ok(RawValue::BytesView(elem.data()))
        }
    }
}

impl<'e> TryFrom<&'e DicomElement> for RawValue<'e> {
    type Error = ParseError;

    /// Based on the VR of this element, parses the binary data into a `RawValue`.
    fn try_from(value: &'e DicomElement) -> ParseResult<Self> {
        Self::try_from(ElementWithVr(value, value.vr()))
    }
}

impl TryFrom<&DicomElement> for Vec<Attribute> {
    type Error = ParseError;

    /// Parses the value for this element as an attribute (aka a tag)
    /// Associated VRs: AT
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        let shorts: Vec<u16> = value.try_into()?;
        if shorts.len() % 2 != 0 {
            return Err(ParseErrorInfo(value, "value is not a multiple of 4 bytes", None).into());
        }

        let num_attrs = shorts.len() / 2;
        let mut attrs: Vec<Attribute> = Vec::with_capacity(num_attrs);
        for chunk in shorts.chunks(2) {
            let attr: u32 = (u32::from(chunk[0]) << 16) + u32::from(chunk[1]);
            attrs.push(Attribute(attr));
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
        String::try_from(ElementWithVr(value, value.vr()))
    }
}

impl<'e> TryFrom<ElementWithVr<'e>> for String {
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
            .cs()
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
        Vec::<String>::try_from(ElementWithVr(value, value.vr()))
    }
}

impl<'e> TryFrom<ElementWithVr<'e>> for Vec<String> {
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
            .cs()
            .decode(data)
            .map_err(|e| ParseError::CharsetError { source: e })
            .map(|multivalue: String| {
                if vr.allows_backslash_text_value {
                    vec![multivalue]
                } else {
                    multivalue
                        .split(CS_SEPARATOR)
                        .map(str::to_owned)
                        .collect::<Vec<String>>()
                }
            })
    }
}

impl<'e> From<ElementWithVr<'e>> for BytesWithoutPadding<'e> {
    /// Returns the value as a slice with the padding character
    /// removed per the specification of whether the VR indicates leading/trailing
    /// padding is significant.
    fn from(value: ElementWithVr<'e>) -> Self {
        // grab the position to start reading bytes from prior to computing the new bytes_read
        let mut left_index: usize = 0;

        let data: &[u8] = value.0.data().as_slice();
        if data.is_empty() {
            return BytesWithoutPadding(data);
        }

        let mut right_index: usize = data.len() - 1;
        if value.1.can_pad_end {
            if value.1.padding == SPACE_PADDING {
                // space padding should strip all trailing spaces
                while right_index > left_index {
                    // character string vr's also sometimes seem to be zero-padded
                    if data[right_index] == SPACE_PADDING || data[right_index] == NULL_PADDING {
                        right_index -= 1;
                    } else {
                        break;
                    }
                }
            } else if value.1.padding == NULL_PADDING {
                // null byte padding is only singular and only if used to achieve even length
                if data.len() % 2 == 0 && data[right_index] == NULL_PADDING {
                    right_index -= 1;
                }
            }
        }

        if value.1.can_pad_front {
            // space padding should strip all leading spaces
            if value.1.padding == SPACE_PADDING {
                while left_index < right_index {
                    if data[left_index] == SPACE_PADDING {
                        left_index += 1;
                    } else {
                        break;
                    }
                }
            }
            // no such thing as leading padding of null bytes
        }

        // if a character string is trimmed down to just a null byte then return empty data.
        // use `..` to return empty slice instead of `..=`.
        if left_index == right_index
            && value.1.is_character_string
            && data[left_index] == NULL_PADDING
        {
            BytesWithoutPadding(&data[left_index..right_index])
        } else {
            BytesWithoutPadding(&data[left_index..=right_index])
        }
    }
}

fn str_parse_nums<T>(value: &DicomElement) -> ParseResult<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let (values, errors): (Vec<ParseResult<T>>, Vec<ParseResult<T>>) =
        Vec::<String>::try_from(value)?
            .into_iter()
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.trim()
                    .parse::<T>()
                    .map_err(|e| ParseErrorInfo(value, e.to_string().as_str(), None).into())
            })
            .partition(ParseResult::is_ok);
    if let Some(Err(e)) = errors.into_iter().last() {
        return Err(e);
    }
    let values: Vec<T> = values
        .into_iter()
        .map(ParseResult::unwrap)
        .collect::<Vec<T>>();
    Ok(values)
}

fn bin_parse_nums<T, FLE, FBE>(value: &DicomElement, le: FLE, be: FBE) -> ParseResult<Vec<T>>
where
    FLE: Fn(&[u8]) -> ParseResult<T>,
    FBE: Fn(&[u8]) -> ParseResult<T>,
{
    let data = value.data();
    let num_bytes: usize = data.len();
    if num_bytes == 0 {
        return Ok(Vec::with_capacity(0));
    }

    let t_size = size_of::<T>();
    if num_bytes % t_size != 0 {
        let t_name = type_name::<T>();
        return Err(ParseErrorInfo(
            value,
            &format!("num bytes not multiple of size of {t_name}"),
            None,
        )
        .into());
    }

    let num_items: usize = num_bytes / t_size;
    let mut result: Vec<T> = Vec::with_capacity(num_items);
    for item_num in 0..num_items {
        let idx = item_num * t_size;
        let buf = &data[idx..(idx + t_size)];
        let val: T = if value.ts().big_endian() {
            be(buf)?
        } else {
            le(buf)?
        };
        result.push(val);
    }
    Ok(result)
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
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(i16::from_le_bytes(
                TryInto::<[u8; 2]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(i16::from_be_bytes(
                TryInto::<[u8; 2]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
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
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(u16::from_le_bytes(
                TryInto::<[u8; 2]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(u16::from_be_bytes(
                TryInto::<[u8; 2]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
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
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(i32::from_le_bytes(
                TryInto::<[u8; 4]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(i32::from_be_bytes(
                TryInto::<[u8; 4]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
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
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(u32::from_le_bytes(
                TryInto::<[u8; 4]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(u32::from_be_bytes(
                TryInto::<[u8; 4]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
    }
}

impl TryFrom<&DicomElement> for Vec<i64> {
    type Error = ParseError;

    /// Parses the value for this element as a list of signed 64bit integer values
    /// Associated VRs: SV
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(i64::from_le_bytes(
                TryInto::<[u8; 8]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(i64::from_be_bytes(
                TryInto::<[u8; 8]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
    }
}

impl TryFrom<&DicomElement> for Vec<u64> {
    type Error = ParseError;

    /// Parses the value for this element as a list of unsigned 64bit integer values
    /// Associated VRs: UV
    fn try_from(value: &DicomElement) -> ParseResult<Self> {
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(u64::from_le_bytes(
                TryInto::<[u8; 8]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(u64::from_be_bytes(
                TryInto::<[u8; 8]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
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
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(f32::from_le_bytes(
                TryInto::<[u8; 4]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(f32::from_be_bytes(
                TryInto::<[u8; 4]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
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
        if value.vr().is_character_string {
            return str_parse_nums(value);
        }
        let le = |b: &[u8]| {
            Ok(f64::from_le_bytes(
                TryInto::<[u8; 8]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        let be = |b: &[u8]| {
            Ok(f64::from_be_bytes(
                TryInto::<[u8; 8]>::try_into(b)
                    .map_err(|e| ParseError::from(ParseErrorInfo(value, &e.to_string(), None)))?,
            ))
        };
        bin_parse_nums(value, le, be)
    }
}
