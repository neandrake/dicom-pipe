/*
   Copyright 2024 Christopher Speck

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

use crate::core::pixeldata::{
    pixel_i16::PixelDataBufferI16, pixel_i32::PixelDataBufferI32, pixel_i8::PixelDataBufferI8,
    pixel_u16::PixelDataBufferU16, pixel_u32::PixelDataBufferU32, pixel_u8::PixelDataBufferU8,
};

/// Container for the raw pixel values parsed from the DICOM binary data.
#[derive(Debug)]
pub enum PixelDataBuffer {
    I8(PixelDataBufferI8),
    U8(PixelDataBufferU8),
    I16(PixelDataBufferI16),
    U16(PixelDataBufferU16),
    I32(PixelDataBufferI32),
    U32(PixelDataBufferU32),
}

impl PixelDataBuffer {
    /// Shift an i8 value into u8 space, so i8::MIN -> u8::MIN.
    pub fn shift_i8(val: i8) -> u8 {
        ((val as i16).saturating_add(1) + (i8::MAX as i16)) as u8
    }

    pub fn unshift_u8(val: u8) -> i8 {
        ((val as i16).saturating_sub(1) - (i8::MAX as i16)) as i8
    }

    /// Shift an i16 value into u16 space, so i16::MIN -> u16::MIN.
    pub fn shift_i16(val: i16) -> u16 {
        ((val as i32).saturating_add(1) + (i16::MAX as i32)) as u16
    }

    pub fn unshift_u16(val: u16) -> i16 {
        ((val as i32).saturating_sub(1) - (i16::MAX as i32)) as i16
    }

    /// Shift an i32 value into u32 space, so i32::MIN -> u32::MIN.
    pub fn shift_i32(val: i32) -> u32 {
        ((val as i64).saturating_add(1) + (i32::MAX as i64)) as u32
    }

    pub fn ushift_u32(val: u32) -> i32 {
        ((val as i64).saturating_sub(1) - (i32::MAX as i64)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::PixelDataBuffer;

    #[test]
    pub fn test_shift_i8() {
        assert_eq!(0u8, PixelDataBuffer::shift_i8(i8::MIN));
        assert_eq!(1u8, PixelDataBuffer::shift_i8(i8::MIN + 1));
        assert_eq!(127u8, PixelDataBuffer::shift_i8(-1));
        assert_eq!(128u8, PixelDataBuffer::shift_i8(0));
        assert_eq!(129u8, PixelDataBuffer::shift_i8(1));
        assert_eq!(254u8, PixelDataBuffer::shift_i8(i8::MAX - 1));
        assert_eq!(255u8, PixelDataBuffer::shift_i8(i8::MAX));
    }

    #[test]
    pub fn test_shift_i16() {
        assert_eq!(0u16, PixelDataBuffer::shift_i16(i16::MIN));
        assert_eq!(1u16, PixelDataBuffer::shift_i16(i16::MIN + 1));
        assert_eq!(32767u16, PixelDataBuffer::shift_i16(-1));
        assert_eq!(32768u16, PixelDataBuffer::shift_i16(0));
        assert_eq!(32769u16, PixelDataBuffer::shift_i16(1));
        assert_eq!(65534u16, PixelDataBuffer::shift_i16(i16::MAX - 1));
        assert_eq!(65535u16, PixelDataBuffer::shift_i16(i16::MAX));
    }

    #[test]
    pub fn test_shift_i32() {
        assert_eq!(0u32, PixelDataBuffer::shift_i32(i32::MIN));
        assert_eq!(1u32, PixelDataBuffer::shift_i32(i32::MIN + 1));
        assert_eq!(2147483647u32, PixelDataBuffer::shift_i32(-1));
        assert_eq!(2147483648u32, PixelDataBuffer::shift_i32(0));
        assert_eq!(2147483649u32, PixelDataBuffer::shift_i32(1));
        assert_eq!(4294967294u32, PixelDataBuffer::shift_i32(i32::MAX - 1));
        assert_eq!(4294967295u32, PixelDataBuffer::shift_i32(i32::MAX));
    }
}
