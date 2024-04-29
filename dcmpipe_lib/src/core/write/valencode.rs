//! This module contains implementations for encoding values for a DICOM element's value field
//! bytes, based on the element's value representation and transfer syntax.

use std::iter::once;

use crate::{
    core::{
        dcmelement::DicomElement,
        read::{valdecode::U32_SIZE, ParseError, ParseResult},
        values::{Attribute, RawValue},
    },
    defn::vr::{CS_SEPARATOR, CS_SEPARATOR_BYTE},
};

/// Encodes a RawValue into the binary data for the given element, based on the element's currently
/// set Value Representation, Character Set, and Transfer Syntax.
pub fn encode_value(elem: &DicomElement, value: RawValue) -> ParseResult<Vec<u8>> {
    let mut bytes: Vec<u8> = match value {
        RawValue::Attribute(attrs) => {
            let num_attrs = attrs.len();
            let mut bytes: Vec<u8> = vec![0u8; U32_SIZE * num_attrs];
            for (i, attr) in attrs.iter().enumerate() {
                let Attribute(attr) = attr;
                let group_number: u16 = ((attr >> 16) & 0xFFFF) as u16;
                let elem_number: u16 = (attr & 0xFFFF) as u16;
                let idx = i * U32_SIZE;
                if elem.get_ts().is_big_endian() {
                    bytes[idx..(idx + 2)].copy_from_slice(&group_number.to_be_bytes());
                    bytes[(idx + 2)..(idx + 4)].copy_from_slice(&elem_number.to_be_bytes());
                } else {
                    bytes[idx..(idx + 2)].copy_from_slice(&group_number.to_le_bytes());
                    bytes[(idx + 2)..(idx + 4)].copy_from_slice(&elem_number.to_le_bytes());
                }
            }
            bytes
        }
        RawValue::Uid(uid) => elem
            .get_cs()
            .encode(&uid)
            .map_err(|e| ParseError::CharsetError { source: e })?,
        RawValue::Strings(strings) => {
            type MaybeBytes = Vec<ParseResult<Vec<u8>>>;
            let (values, errs): (MaybeBytes, MaybeBytes) = strings
                .iter()
                .map(|s| {
                    elem.get_cs()
                        .encode(s)
                        // Add the separator after each encoded value. Below the last separator
                        // will be popped off.
                        .map(|mut v| {
                            v.push(CS_SEPARATOR as u8);
                            v
                        })
                        .map_err(|e| ParseError::CharsetError { source: e })
                })
                .partition(ParseResult::is_ok);

            if let Some(Err(e)) = errs.into_iter().last() {
                return Err(e);
            }

            // Flatten the bytes for all strings.
            let mut bytes: Vec<u8> = values
                .into_iter()
                .flat_map(ParseResult::unwrap)
                .collect::<Vec<u8>>();
            // Remove last separator.
            bytes.pop();
            bytes
        }
        RawValue::Shorts(shorts) => {
            if elem.get_vr().is_character_string {
                // This should only be the case for a VR of IS.
                let mut encoded = shorts
                    .into_iter()
                    // In theory this should use the default character set, but this
                    // relies on i16::to_string only using ascii which falls under that.
                    .map(|short: i16| short.to_string().into_bytes())
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
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
                        if elem.get_ts().is_big_endian() {
                            short.to_be_bytes()
                        } else {
                            short.to_le_bytes()
                        }
                    })
                    .collect::<Vec<u8>>()
            }
        }
        RawValue::UnsignedShorts(ushorts) => {
            if elem.get_vr().is_character_string {
                // This should only be the case for a VR of IS.
                let mut encoded = ushorts
                    .into_iter()
                    // In theory this should use the default character set, but this
                    // relies on u16::to_string only using ascii which falls under that.
                    .map(|ushort: u16| ushort.to_string().into_bytes())
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
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
                        if elem.get_ts().is_big_endian() {
                            ushort.to_be_bytes()
                        } else {
                            ushort.to_le_bytes()
                        }
                    })
                    .collect::<Vec<u8>>()
            }
        }
        RawValue::Integers(ints) => {
            if elem.get_vr().is_character_string {
                // This should only be the case for a VR of IS.
                let mut encoded = ints
                    .into_iter()
                    // In theory this should use the default character set, but this
                    // relies on i32::to_string only using ascii which falls under that.
                    .map(|int: i32| int.to_string().into_bytes())
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                    .collect::<Vec<u8>>();
                if !encoded.is_empty() {
                    encoded.remove(encoded.len() - 1);
                }
                encoded
            } else {
                // This should only be the case for a VR of SL.
                ints.into_iter()
                    .flat_map(|int: i32| {
                        if elem.get_ts().is_big_endian() {
                            int.to_be_bytes()
                        } else {
                            int.to_le_bytes()
                        }
                    })
                    .collect::<Vec<u8>>()
            }
        }
        RawValue::UnsignedIntegers(uints) => {
            if elem.get_vr().is_character_string {
                // XXX: This shouldn't happen. Unsigned integers should only ever be encoded
                // as binary.
                let mut encoded = uints
                    .into_iter()
                    // In theory this should use the default character set, but this
                    // relies on i16::to_string only using ascii which falls under that.
                    .map(|uint: u32| uint.to_string().into_bytes())
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
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
                        if elem.get_ts().is_big_endian() {
                            uint.to_be_bytes()
                        } else {
                            uint.to_le_bytes()
                        }
                    })
                    .collect::<Vec<u8>>()
            }
        }
        RawValue::Longs(longs) => {
            if elem.get_vr().is_character_string {
                // This should only be the case for a VR of IS.
                let mut encoded = longs
                    .into_iter()
                    // In theory this should use the default character set, but this
                    // relies on i32::to_string only using ascii which falls under that.
                    .map(|long: i64| long.to_string().into_bytes())
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
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
                        if elem.get_ts().is_big_endian() {
                            long.to_be_bytes()
                        } else {
                            long.to_le_bytes()
                        }
                    })
                    .collect::<Vec<u8>>()
            }
        }
        RawValue::UnsignedLongs(ulongs) => {
            if elem.get_vr().is_character_string {
                // XXX: This shouldn't happen. Unsigned integers should only ever be encoded
                // as binary.
                let mut encoded = ulongs
                    .into_iter()
                    // In theory this should use the default character set, but this
                    // relies on i16::to_string only using ascii which falls under that.
                    .map(|ulong: u64| ulong.to_string().into_bytes())
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
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
                        if elem.get_ts().is_big_endian() {
                            ulong.to_be_bytes()
                        } else {
                            ulong.to_le_bytes()
                        }
                    })
                    .collect::<Vec<u8>>()
            }
        }
        RawValue::Floats(floats) => {
            if elem.get_vr().is_character_string {
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
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
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
                        if elem.get_ts().is_big_endian() {
                            float.to_be_bytes()
                        } else {
                            float.to_le_bytes()
                        }
                    })
                    .collect::<Vec<u8>>()
            }
        }
        RawValue::Doubles(doubles) => {
            if elem.get_vr().is_character_string {
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
                    .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
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
                        if elem.get_ts().is_big_endian() {
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
                if elem.get_ts().is_big_endian() {
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
                if elem.get_ts().is_big_endian() {
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
                if elem.get_ts().is_big_endian() {
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
        bytes.push(elem.get_vr().padding);
    }

    Ok(bytes)
}
