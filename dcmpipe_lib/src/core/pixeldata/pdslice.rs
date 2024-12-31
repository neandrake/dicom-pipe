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
    pixel_i16::PixelDataSliceI16, pixel_i32::PixelDataSliceI32, pixel_i8::PixelDataSliceI8,
    pixel_u16::PixelDataSliceU16, pixel_u32::PixelDataSliceU32, pixel_u8::PixelDataSliceU8,
};

/// Container for the raw pixel values parsed from the DICOM binary data.
#[derive(Debug)]
pub enum PixelDataSlice {
    I8(PixelDataSliceI8),
    U8(PixelDataSliceU8),
    I16(PixelDataSliceI16),
    U16(PixelDataSliceU16),
    I32(PixelDataSliceI32),
    U32(PixelDataSliceU32),
}

impl PixelDataSlice {
    /// Shift an i8 value into u8 space, so i8::MIN -> u8::MIN.
    pub fn shift_i8(val: i8) -> u8 {
        ((val as i16).saturating_add(1) + (i8::MAX as i16)) as u8
    }

    /// Shift an i16 value into u16 space, so i16::MIN -> u16::MIN.
    pub fn shift_i16(val: i16) -> u16 {
        ((val as i32).saturating_add(1) + (i16::MAX as i32)) as u16
    }

    /// Shift an i32 value into u32 space, so i32::MIN -> u32::MIN.
    pub fn shift_i32(val: i32) -> u32 {
        ((val as i64).saturating_add(1) + (i32::MAX as i64)) as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::core::pixeldata::pdslice::PixelDataSlice;

    #[test]
    pub fn test_shift_i8() {
        assert_eq!(0u8, PixelDataSlice::shift_i8(i8::MIN));
        assert_eq!(1u8, PixelDataSlice::shift_i8(i8::MIN + 1));
        assert_eq!(127u8, PixelDataSlice::shift_i8(-1));
        assert_eq!(128u8, PixelDataSlice::shift_i8(0));
        assert_eq!(129u8, PixelDataSlice::shift_i8(1));
        assert_eq!(254u8, PixelDataSlice::shift_i8(i8::MAX - 1));
        assert_eq!(255u8, PixelDataSlice::shift_i8(i8::MAX));
    }

    #[test]
    pub fn test_shift_i16() {
        assert_eq!(0u16, PixelDataSlice::shift_i16(i16::MIN));
        assert_eq!(1u16, PixelDataSlice::shift_i16(i16::MIN + 1));
        assert_eq!(32767u16, PixelDataSlice::shift_i16(-1));
        assert_eq!(32768u16, PixelDataSlice::shift_i16(0));
        assert_eq!(32769u16, PixelDataSlice::shift_i16(1));
        assert_eq!(65534u16, PixelDataSlice::shift_i16(i16::MAX - 1));
        assert_eq!(65535u16, PixelDataSlice::shift_i16(i16::MAX));
    }

    #[test]
    pub fn test_shift_i32() {
        assert_eq!(0u32, PixelDataSlice::shift_i32(i32::MIN));
        assert_eq!(1u32, PixelDataSlice::shift_i32(i32::MIN + 1));
        assert_eq!(2147483647u32, PixelDataSlice::shift_i32(-1));
        assert_eq!(2147483648u32, PixelDataSlice::shift_i32(0));
        assert_eq!(2147483649u32, PixelDataSlice::shift_i32(1));
        assert_eq!(4294967294u32, PixelDataSlice::shift_i32(i32::MAX - 1));
        assert_eq!(4294967295u32, PixelDataSlice::shift_i32(i32::MAX));
    }
}
