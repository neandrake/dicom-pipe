use crate::core::charset::CSRef;
use crate::core::dcmparser_util;
use crate::core::dcmsqelem::SequenceElement;
use crate::core::tagpath::{TagPath, TagPathElement};
use crate::defn::ts::TSRef;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef, CHARACTER_STRING_SEPARATOR};
use encoding::types::DecoderTrap;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

const U16_SIZE: usize = std::mem::size_of::<u16>();
const I16_SIZE: usize = std::mem::size_of::<i16>();
const U32_SIZE: usize = std::mem::size_of::<u32>();
const I32_SIZE: usize = std::mem::size_of::<i32>();
const F32_SIZE: usize = std::mem::size_of::<f32>();
const F64_SIZE: usize = std::mem::size_of::<f64>();

/// Wrapper around `&[u8]` for getting a slice of the element value without the padding values
struct BytesWithoutPadding<'me>(&'me [u8]);

/// For parsing an element value as a string with a specific VR
pub struct ElementWithVr<'me>(pub &'me DicomElement, pub VRRef);

/// Wrapper around `u32` for parsing DICOM Attributes
pub struct Attribute(pub u32);

/// Wrapper around an element's value parsed into a native/raw type
pub enum RawValue {
    Attribute(Attribute),
    Uid(String),
    Strings(Vec<String>),
    Doubles(Vec<f64>),
    Shorts(Vec<i16>),
    Integers(Vec<i32>),
    UnsignedIntegers(Vec<u32>),
    Bytes(Vec<u8>),
}

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
pub struct DicomElement {
    pub tag: u32,
    pub vr: VRRef,
    pub vl: ValueLength,

    data: Vec<u8>,
    sq_path: Vec<SequenceElement>,

    ts: TSRef,
    cs: CSRef,
}

impl DicomElement {
    pub fn new(
        tag: u32,
        vr: VRRef,
        vl: ValueLength,
        ts: TSRef,
        cs: CSRef,
        data: Vec<u8>,
        sq_path: Vec<SequenceElement>,
    ) -> DicomElement {
        let cs: CSRef = vr.get_proper_cs(cs);
        DicomElement {
            tag,
            vr,
            vl,
            ts,
            cs,
            data,
            sq_path,
        }
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

    pub fn get_tag_path(&self) -> TagPath {
        let mut path: Vec<TagPathElement> = self
            .sq_path
            .iter()
            .map(|dse: &SequenceElement| TagPathElement::new(dse.get_seq_tag(), None))
            .collect::<Vec<TagPathElement>>();
        path.push(TagPathElement::new(self.tag, None));
        TagPath::new_from_vec(path)
    }

    /// Returns if this element is a `SQ` or if it should be parsed as though it were a sequence
    pub fn is_seq_like(&self) -> bool {
        self.vr == &vr::SQ || dcmparser_util::is_non_standard_seq(self.tag, self.vr, self.vl)
    }

    /// Returns whether the the size of the value for this element is zero
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Parses this element's data into native/raw value type
    pub fn parse_value(&self) -> Result<RawValue, Error> {
        if self.vr == &vr::AT {
            let attr: Attribute = Attribute::try_from(self)?;
            Ok(RawValue::Attribute(attr))
        } else if self.vr == &vr::UI {
            let uid: String = String::try_from(self)?;
            Ok(RawValue::Uid(uid))
        } else if self.vr == &vr::DS {
            let val: Vec<f64> = Vec::<f64>::try_from(self)?;
            return Ok(RawValue::Doubles(val));
        } else if self.vr == &vr::IS {
            let val: Vec<i32> = Vec::<i32>::try_from(self)?;
            return Ok(RawValue::Integers(val));
        } else if self.vr.is_character_string {
            let strings: Vec<String> = Vec::<String>::try_from(self)?;
            Ok(RawValue::Strings(strings))
        } else if self.vr == &vr::FD
            || self.vr == &vr::OF
            || self.vr == &vr::OD
            || self.vr == &vr::FL
        {
            let doubles: Vec<f64> = match self.vl {
                ValueLength::Explicit(len)
                    if (self.vr == &vr::OD || self.vr == &vr::FL) && len > 0 && len % 8 == 0 =>
                {
                    Vec::<f64>::try_from(self)?
                }
                ValueLength::Explicit(len) if len > 0 && len % 4 == 0 => {
                    Vec::<f32>::try_from(self)?
                        .into_iter()
                        .map(f64::from)
                        .collect::<Vec<f64>>()
                }
                ValueLength::Explicit(1) => vec![f64::from(self.get_data()[0])],
                _ => vec![],
            };
            Ok(RawValue::Doubles(doubles))
        } else if self.vr == &vr::SS {
            let shorts: Vec<i16> = match self.vl {
                ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => {
                    Vec::<i16>::try_from(self)?
                }
                ValueLength::Explicit(1) => vec![i16::from(self.get_data()[0])],
                _ => vec![],
            };
            Ok(RawValue::Shorts(shorts))
        } else if self.vr == &vr::SL {
            let ints: Vec<i32> = match self.vl {
                ValueLength::Explicit(len) if len > 0 && len % 4 == 0 => {
                    Vec::<i32>::try_from(self)?
                }
                ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => {
                    Vec::<i16>::try_from(self)?
                        .into_iter()
                        .map(i32::from)
                        .collect::<Vec<i32>>()
                }
                ValueLength::Explicit(1) => vec![i32::from(self.get_data()[0])],
                _ => vec![],
            };
            Ok(RawValue::Integers(ints))
        } else if self.vr == &vr::UL
            || self.vr == &vr::OL
            || self.vr == &vr::OW
            || self.vr == &vr::US
        {
            let uints: Vec<u32> = match self.vl {
                ValueLength::Explicit(len)
                    if (self.vr == &vr::UL || self.vr == &vr::OL) && len > 0 && len % 4 == 0 =>
                {
                    Vec::<u32>::try_from(self)?
                }
                ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => {
                    Vec::<u16>::try_from(self)?
                        .into_iter()
                        .map(u32::from)
                        .collect::<Vec<u32>>()
                }
                ValueLength::Explicit(1) => vec![u32::from(self.get_data()[0])],
                _ => vec![],
            };
            Ok(RawValue::UnsignedIntegers(uints))
        } else {
            let bytes: Vec<u8> = self.get_data().clone();
            Ok(RawValue::Bytes(bytes))
        }
    }
}

impl TryFrom<&DicomElement> for Attribute {
    type Error = Error;

    /// Parses the value for this element as an attribute (aka a tag)
    /// Associated VRs: AT
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        if value.data.len() < 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse attribute",
            ));
        }

        let mut buf: [u8; 2] = [0; 2];
        buf.copy_from_slice(&value.data[0..2]);
        let first: u32 = if value.ts.is_big_endian() {
            u32::from(u16::from_be_bytes(buf)) << 16
        } else {
            u32::from(u16::from_le_bytes(buf)) << 16
        };

        buf.copy_from_slice(&value.data[2..4]);
        let second: u32 = if value.ts.is_big_endian() {
            u32::from(u16::from_be_bytes(buf))
        } else {
            u32::from(u16::from_le_bytes(buf))
        };
        let result: u32 = first + second;
        Ok(Attribute(result))
    }
}

impl<'me> TryFrom<&DicomElement> for String {
    type Error = Error;

    /// Parses the value of this element as a string using the element's encoding and VR
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        String::try_from(ElementWithVr(value, value.vr))
    }
}

impl<'me> TryFrom<ElementWithVr<'me>> for String {
    type Error = Error;

    /// Parses the value of this element as a string using the element's encoding and the specified
    /// VR. This can be used in the event the parser of the element did not have an appropriate
    /// tag dictionary for resolving implicit VRs.
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: ElementWithVr<'_>) -> Result<Self, Self::Error> {
        let element: &DicomElement = value.0;
        let data: &[u8] = BytesWithoutPadding::from(value).0;
        element
            .cs
            .decode(data, DecoderTrap::Strict)
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
    }
}

impl TryFrom<&DicomElement> for Vec<String> {
    type Error = Error;

    /// Parses the value of this element as a list of strings using the element's encoding and vr.
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        Vec::<String>::try_from(ElementWithVr(value, value.vr))
    }
}

impl<'me> TryFrom<ElementWithVr<'me>> for Vec<String> {
    type Error = Error;

    /// Parses the value of this element as a list of strings using the element's encoding and the
    /// specified VR. This can be used in the event the parser of the element did not have an appropriate
    /// tag dictionary for resolving implicit VRs.
    /// Associated VRs:
    /// All character string VR's -- subsequent interpretation of String is necessary based on VR
    /// AE, AS, CS, DA, DS, DT, IS, LO, LT, PN, SH, ST, TM, UC, UI, UR, UT
    fn try_from(value: ElementWithVr<'_>) -> Result<Self, Self::Error> {
        let element: &DicomElement = value.0;
        let vr: VRRef = value.1;
        let data: &[u8] = BytesWithoutPadding::from(value).0;
        element
            .cs
            .decode(data, DecoderTrap::Strict)
            .map_err(|e: Cow<'static, str>| Error::new(ErrorKind::InvalidData, e.into_owned()))
            .map(|multivalue: String| {
                if !vr.allows_backslash_text_value {
                    multivalue
                        .split(CHARACTER_STRING_SEPARATOR)
                        .map(str::to_owned)
                        .collect::<Vec<String>>()
                } else {
                    let mut vec: Vec<String> = Vec::new();
                    vec.push(multivalue);
                    vec
                }
            })
    }
}

impl<'me> From<ElementWithVr<'me>> for BytesWithoutPadding<'me> {
    /// Returns the value as a slice with the padding character
    /// removed per the specification of whether the VR indicates leading/trailing
    /// padding is significant.
    fn from(value: ElementWithVr<'me>) -> Self {
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
    type Error = Error;

    /// Parses the value for this element as a 32bit floating point
    /// Associated VRs: FL
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        Vec::<f32>::try_from(value)?
            .first()
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to parse as f32"))
    }
}

impl TryFrom<&DicomElement> for Vec<f32> {
    type Error = Error;

    /// Parses the value for this element as a list of 32bit floating point values
    /// Associated VRs: OF
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        if value.vr.is_character_string {
            let parse_results: Vec<Result<f32, Error>> = Vec::<String>::try_from(value)?
                .iter()
                .map(|s| s.parse::<f32>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Vec<Result<f32, Error>>>();

            let mut results: Vec<f32> = Vec::with_capacity(parse_results.len());
            for parse_result in parse_results {
                results.push(parse_result?);
            }
            return Ok(results);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes < F32_SIZE || num_bytes % F32_SIZE != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse as f32(s)",
            ));
        }

        let mut buf: [u8; F32_SIZE] = [0; F32_SIZE];
        let num_bytes: usize = value.data.len();
        let num_f32s: usize = num_bytes / F32_SIZE;
        let mut result: Vec<f32> = Vec::with_capacity(num_f32s);
        for i in 0..num_f32s {
            buf.copy_from_slice(&value.data[i..(i + F32_SIZE)]);
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
    type Error = Error;

    /// Parses the value for this element as a 64bit floating point
    /// Associated VRs: FD
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        Vec::<f64>::try_from(value)?
            .first()
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to parse as f64"))
    }
}

impl TryFrom<&DicomElement> for Vec<f64> {
    type Error = Error;

    /// Parses the value for this element as a list of 64bit floating point values
    /// Associated VRs: OD
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        if value.vr.is_character_string {
            let parse_results: Vec<Result<f64, Error>> = Vec::<String>::try_from(value)?
                .iter()
                .map(|s| s.parse::<f64>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Vec<Result<f64, Error>>>();

            let mut results: Vec<f64> = Vec::with_capacity(parse_results.len());
            for parse_result in parse_results {
                results.push(parse_result?);
            }
            return Ok(results);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes < F64_SIZE || num_bytes % F64_SIZE != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse as f64(s)",
            ));
        }

        let mut buf: [u8; F64_SIZE] = [0; F64_SIZE];
        let num_f64s: usize = num_bytes / F64_SIZE;
        let mut result: Vec<f64> = Vec::with_capacity(num_f64s);
        for i in 0..num_f64s {
            buf.copy_from_slice(&value.data[i..(i + F64_SIZE)]);
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
    type Error = Error;

    /// Parses the value for this element as a signed 16bit integer
    /// Associated VRs: SS
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        Vec::<i16>::try_from(value)?
            .first()
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to parse as i16"))
    }
}

impl TryFrom<&DicomElement> for Vec<i16> {
    type Error = Error;

    /// Parses the value for this element as a list of signed 16bit integer values
    /// Associated VRs: OW
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        if value.vr.is_character_string {
            let parse_results: Vec<Result<i16, Error>> = Vec::<String>::try_from(value)?
                .iter()
                .map(|s| s.parse::<i16>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Vec<Result<i16, Error>>>();

            let mut results: Vec<i16> = Vec::with_capacity(parse_results.len());
            for parse_result in parse_results {
                results.push(parse_result?);
            }
            return Ok(results);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes < I16_SIZE || num_bytes % I16_SIZE != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse as i16(s)",
            ));
        }

        let mut buf: [u8; I16_SIZE] = [0; I16_SIZE];
        let num_i16s: usize = num_bytes / I16_SIZE;
        let mut result: Vec<i16> = Vec::with_capacity(num_i16s);
        // TODO: Verify that we're parsing as 2s complement (not sure Endian should be considered?)
        for i in 0..num_i16s {
            buf.copy_from_slice(&value.data[i..(i + I16_SIZE)]);
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
    type Error = Error;

    /// Parses the value for this element as a signed 32bit integer
    /// Associated VRs: SL
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        Vec::<i32>::try_from(value)?
            .first()
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to parse as i32"))
    }
}

impl TryFrom<&DicomElement> for Vec<i32> {
    type Error = Error;

    /// Parses the value for this element as a list of signed 32bit integer values
    /// Associated VRs: OL
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        if value.vr.is_character_string {
            let parse_results: Vec<Result<i32, Error>> = Vec::<String>::try_from(value)?
                .iter()
                .map(|s| s.parse::<i32>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Vec<Result<i32, Error>>>();

            let mut results: Vec<i32> = Vec::with_capacity(parse_results.len());
            for parse_result in parse_results {
                results.push(parse_result?);
            }
            return Ok(results);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes < I32_SIZE || num_bytes % I32_SIZE != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse as i32(s)",
            ));
        }

        let mut buf: [u8; I32_SIZE] = [0; I32_SIZE];
        let num_i32s: usize = num_bytes / I32_SIZE;
        let mut result: Vec<i32> = Vec::with_capacity(num_i32s);
        for i in 0..num_i32s {
            buf.copy_from_slice(&value.data[i..(i + I32_SIZE)]);
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
    type Error = Error;

    /// Parses the value for this element as an unsigned 32bit integer
    /// Associated VRs: UL
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        Vec::<u32>::try_from(value)?
            .first()
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to parse as u32"))
    }
}

impl TryFrom<&DicomElement> for Vec<u32> {
    type Error = Error;

    /// Parses the value for this element as a list of unsigned 32bit integer values
    /// Associated VRs: UL, OL
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        if value.vr.is_character_string {
            let parse_results: Vec<Result<u32, Error>> = Vec::<String>::try_from(value)?
                .iter()
                .map(|s| s.parse::<u32>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Vec<Result<u32, Error>>>();

            let mut results: Vec<u32> = Vec::with_capacity(parse_results.len());
            for parse_result in parse_results {
                results.push(parse_result?);
            }
            return Ok(results);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes < U32_SIZE || num_bytes % U32_SIZE != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse as u32(s)",
            ));
        }

        let mut buf: [u8; U32_SIZE] = [0; U32_SIZE];
        let num_u32s: usize = num_bytes / U32_SIZE;
        let mut result: Vec<u32> = Vec::with_capacity(num_u32s);
        for i in 0..num_u32s {
            buf.copy_from_slice(&value.data[i..(i + U32_SIZE)]);
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

impl TryFrom<&DicomElement> for u16 {
    type Error = Error;

    /// Parses the value for this element as an unsigned 16bit integer
    /// Associated VRs: US
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        Vec::<u16>::try_from(value)?
            .first()
            .copied()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to parse as u16"))
    }
}

impl TryFrom<&DicomElement> for Vec<u16> {
    type Error = Error;

    /// Parses the value for this element as a list of unsigned 16bit integer values
    /// Associated VRs: US, OW
    fn try_from(value: &DicomElement) -> Result<Self, Self::Error> {
        if value.vr.is_character_string {
            let parse_results: Vec<Result<u16, Error>> = Vec::<String>::try_from(value)?
                .iter()
                .map(|s| s.parse::<u16>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Vec<Result<u16, Error>>>();

            let mut results: Vec<u16> = Vec::with_capacity(parse_results.len());
            for parse_result in parse_results {
                results.push(parse_result?);
            }
            return Ok(results);
        }

        let num_bytes: usize = value.data.len();
        if num_bytes < U16_SIZE || num_bytes % U16_SIZE != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse as u16(s)",
            ));
        }

        let mut buf: [u8; U16_SIZE] = [0; U16_SIZE];
        let num_u16s: usize = num_bytes / U16_SIZE;
        let mut result: Vec<u16> = Vec::with_capacity(num_u16s);
        for i in 0..num_u16s {
            buf.copy_from_slice(&value.data[i..(i + U16_SIZE)]);
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
