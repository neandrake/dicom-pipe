use crate::defn::constants::tags;
use crate::defn::ts::TSRef;
use crate::defn::vl;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef, VR};
use std::io::{Error, Read};

/// If the tag isn't Item and VR isn't SQ but ValueLength is Undefined then element should be
/// considered a private-tag sequence whose contents are encoded as IVRLE.
pub fn should_parse_as_seq(tag: u32, vr: VRRef, vl: ValueLength) -> bool {
    tag != tags::ITEM
        && (vr == &vr::UN || vr == &vr::OB || vr == &vr::OW)
        && vl == ValueLength::UndefinedLength
}

/// Reads a tag attribute from a given stream
pub fn read_tag_from_stream(stream: &mut impl Read, ts: TSRef) -> Result<u32, Error> {
    let mut buf: [u8; 2] = [0; 2];

    stream.read_exact(&mut buf)?;
    let group_number: u32 = if ts.is_big_endian() {
        u32::from(u16::from_be_bytes(buf)) << 16
    } else {
        u32::from(u16::from_le_bytes(buf)) << 16
    };

    stream.read_exact(&mut buf)?;
    let element_number: u32 = if ts.is_big_endian() {
        u32::from(u16::from_be_bytes(buf))
    } else {
        u32::from(u16::from_le_bytes(buf))
    };

    let tag: u32 = group_number + element_number;
    Ok(tag)
}

/// Reads a VR from a given stream
pub fn read_vr_from_stream(stream: &mut impl Read) -> Result<Option<VRRef>, Error> {
    let mut buf: [u8; 2] = [0; 2];
    stream.read_exact(&mut buf)?;
    let first_char: u8 = buf[0];
    let second_char: u8 = buf[1];

    let code: u16 = (u16::from(first_char) << 8) + u16::from(second_char);
    let vr: VRRef = match VR::from_code(code) {
        Some(found_vr) => {
            if found_vr.has_explicit_2byte_pad {
                stream.read_exact(&mut buf)?;
            }
            found_vr
        }
        None => return Ok(None),
    };

    Ok(Some(vr))
}

/// Reads a Value Length from a given stream
pub fn read_value_length_from_stream(
    stream: &mut impl Read,
    read_4bytes: bool,
    big_endian: bool,
) -> Result<ValueLength, Error> {
    let value_length: u32 = if read_4bytes {
        let mut buf: [u8; 4] = [0; 4];
        stream.read_exact(&mut buf)?;

        if big_endian {
            u32::from_be_bytes(buf)
        } else {
            u32::from_le_bytes(buf)
        }
    } else {
        let mut buf: [u8; 2] = [0; 2];
        stream.read_exact(&mut buf)?;

        if big_endian {
            u32::from(u16::from_be_bytes(buf))
        } else {
            u32::from(u16::from_le_bytes(buf))
        }
    };
    Ok(vl::from_value_length(value_length))
}
