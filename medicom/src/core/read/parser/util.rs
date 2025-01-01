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

use std::io::{ErrorKind, Read};

use crate::core::{
    defn::{
        ts::TSRef,
        vl::ValueLength,
        vr::{VRRef, VR},
    },
    read::{error::ParseError, ParseResult},
};

/// This is a variation of `Read::read_exact` however if zero bytes are read instead of returning
/// an error with `ErrorKind::UnexpectedEof` it will return an error with `ParseError::ExpectedEOF`.
fn read_exact_expect_eof(dataset: &mut impl Read, mut buf: &mut [u8]) -> ParseResult<()> {
    let mut bytes_read: usize = 0;
    while !buf.is_empty() {
        match dataset.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                bytes_read += n;
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
    }
    if buf.is_empty() {
        Ok(())
    } else if bytes_read == 0 {
        Err(ParseError::ExpectedEOF)
    } else {
        Err(ParseError::IOError {
            source: std::io::Error::new(
                ErrorKind::UnexpectedEof,
                format!("failed to fill whole buffer, read {bytes_read} bytes"),
            ),
        })
    }
}

/// Reads a tag attribute from a given dataset
pub(crate) fn read_tag_from_dataset(
    dataset: &mut impl Read,
    big_endian: bool,
) -> ParseResult<(u32, usize)> {
    let mut buf: [u8; 2] = [0; 2];
    let mut bytes_read: usize = 0;

    read_exact_expect_eof(dataset, &mut buf)?;
    bytes_read += buf.len();
    let group_number: u32 = if big_endian {
        u32::from(u16::from_be_bytes(buf)) << 16
    } else {
        u32::from(u16::from_le_bytes(buf)) << 16
    };

    dataset.read_exact(&mut buf)?;
    bytes_read += buf.len();
    let element_number: u32 = if big_endian {
        u32::from(u16::from_be_bytes(buf))
    } else {
        u32::from(u16::from_le_bytes(buf))
    };

    let tag: u32 = group_number + element_number;
    Ok((tag, bytes_read))
}

/// Reads a VR from a given dataset.
pub(crate) fn read_vr_from_dataset(dataset: &mut impl Read) -> ParseResult<(VRRef, usize)> {
    let mut buf: [u8; 2] = [0; 2];
    let mut bytes_read: usize = 0;

    dataset.read_exact(&mut buf)?;
    bytes_read += buf.len();

    let first_char: u8 = buf[0];
    let second_char: u8 = buf[1];

    let code: u16 = (u16::from(first_char) << 8) + u16::from(second_char);
    let vr: VRRef = match VR::from_code(code) {
        Some(found_vr) => {
            if found_vr.has_explicit_2byte_pad {
                dataset.read_exact(&mut buf)?;
                bytes_read += buf.len();
            }
            found_vr
        }
        None => return Err(ParseError::UnknownExplicitVR(code)),
    };

    Ok((vr, bytes_read))
}

/// Reads a Value Length from a given dataset.
/// `dataset` The dataset to read bytes from.
/// `ts` The transfer syntax of the element being read from.
/// `vr` The VR of the current element the value length is being read for.
pub(crate) fn read_value_length_from_dataset(
    dataset: &mut impl Read,
    ts: TSRef,
    vr: VRRef,
) -> ParseResult<(ValueLength, usize)> {
    let mut bytes_read: usize = 0;
    let value_length: u32 = if !ts.explicit_vr() || vr.has_explicit_2byte_pad {
        let mut buf: [u8; 4] = [0; 4];
        dataset.read_exact(&mut buf)?;
        bytes_read += buf.len();
        if ts.big_endian() {
            u32::from_be_bytes(buf)
        } else {
            u32::from_le_bytes(buf)
        }
    } else {
        let mut buf: [u8; 2] = [0; 2];
        dataset.read_exact(&mut buf)?;
        bytes_read += buf.len();
        if ts.big_endian() {
            u32::from(u16::from_be_bytes(buf))
        } else {
            u32::from(u16::from_le_bytes(buf))
        }
    };
    Ok((ValueLength::from(value_length), bytes_read))
}
