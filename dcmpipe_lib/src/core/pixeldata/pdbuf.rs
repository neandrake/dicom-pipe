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

use crate::core::pixeldata::{pdinfo::PixelDataInfo, PhotoInterp, PixelDataError};

pub struct PixelU8 {
    pub x: usize,
    pub y: usize,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct PixelDataBufferU8 {
    info: PixelDataInfo,
    buffer: Vec<u8>,
    min: u8,
    max: u8,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataBufferU8 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataBufferU8")
            .field("info", &self.info)
            .field("buffer_len", &self.buffer.len())
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl PixelDataBufferU8 {
    pub fn new(info: PixelDataInfo, buffer: Vec<u8>, min: u8, max: u8) -> Self {
        let stride = if info.planar_config() == 0 {
            1
        } else {
            buffer.len() / info.samples_per_pixel() as usize
        };
        let interp_as_rgb =
            info.photo_interp().is_some_and(PhotoInterp::is_rgb) && info.samples_per_pixel() == 3;

        Self {
            info,
            buffer,
            min,
            max,
            stride,
            interp_as_rgb,
        }
    }

    pub fn info(&self) -> &PixelDataInfo {
        &self.info
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn normalize(&self, val: u8) -> u8 {
        let range: f32 = (self.max - self.min) as f32;
        let valz: f32 = (val - self.min) as f32;
        let val = (valz / range * (u8::MAX as f32)) as u8;
        if self
            .info()
            .photo_interp()
            .is_some_and(|pi| *pi == PhotoInterp::Monochrome1)
        {
            !val
        } else {
            val
        }
    }

    pub fn get_pixel(&self, src_byte_index: usize) -> Result<PixelU8, PixelDataError> {
        if src_byte_index >= self.buffer().len()
            || (self.info().planar_config() == 0
                && src_byte_index % self.info().samples_per_pixel() as usize != 0)
            || (self.info().planar_config() != 0
                && src_byte_index >= self.buffer().len() / self.info().samples_per_pixel() as usize)
        {
            return Err(PixelDataError::InvalidPixelSource(src_byte_index));
        }

        let mut dst_pixel_index: usize = src_byte_index;
        if self.interp_as_rgb && self.info().planar_config() == 0 {
            dst_pixel_index /= self.info().samples_per_pixel() as usize;
        }

        let x = dst_pixel_index % (self.info().cols() as usize);
        let y = dst_pixel_index / (self.info().cols() as usize);

        let stride = self.stride();
        let (r, g, b) = if self.interp_as_rgb {
            let r = self.buffer()[src_byte_index];
            let g = self.buffer()[src_byte_index + stride];
            let b = self.buffer()[src_byte_index + stride * 2];
            (r, g, b)
        } else {
            let val = self.normalize(self.buffer()[src_byte_index]);
            (val, val, val)
        };

        Ok(PixelU8 { x, y, r, g, b })
    }

    pub fn pixel_iter(&self) -> PixelDataBufferU8Iter {
        PixelDataBufferU8Iter {
            pdbuf: self,
            src_byte_index: 0,
        }
    }
}

pub struct PixelDataBufferU8Iter<'buf> {
    pdbuf: &'buf PixelDataBufferU8,
    src_byte_index: usize,
}

impl Iterator for PixelDataBufferU8Iter<'_> {
    type Item = PixelU8;

    fn next(&mut self) -> Option<Self::Item> {
        let pixel = self.pdbuf.get_pixel(self.src_byte_index);

        if self.pdbuf.interp_as_rgb && self.pdbuf.info().planar_config() == 0 {
            self.src_byte_index += self.pdbuf.info().samples_per_pixel() as usize;
        } else {
            // If planar config indicates that all R's are stored followed by all G's then all
            // B's, then next R pixel is the next element.
            self.src_byte_index += 1;
        }

        pixel.ok()
    }
}

#[derive(Debug)]
pub struct PixelU16 {
    pub x: usize,
    pub y: usize,
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

pub struct PixelDataBufferU16 {
    info: PixelDataInfo,
    buffer: Vec<u16>,
    min: u16,
    max: u16,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataBufferU16 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataBufferU16")
            .field("info", &self.info)
            .field("buffer_len", &self.buffer.len())
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl PixelDataBufferU16 {
    pub fn new(info: PixelDataInfo, buffer: Vec<u16>, min: u16, max: u16) -> Self {
        let stride = if info.planar_config() == 0 {
            1
        } else {
            buffer.len() / info.samples_per_pixel() as usize
        };
        let interp_as_rgb =
            info.photo_interp().is_some_and(PhotoInterp::is_rgb) && info.samples_per_pixel() == 3;

        Self {
            info,
            buffer,
            min,
            max,
            stride,
            interp_as_rgb,
        }
    }

    pub fn info(&self) -> &PixelDataInfo {
        &self.info
    }

    pub fn buffer(&self) -> &[u16] {
        &self.buffer
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn normalize(&self, val: u16) -> u16 {
        let range: f32 = (self.max - self.min) as f32;
        let valz: f32 = (val - self.min) as f32;
        let val = (valz / range * (u16::MAX as f32)) as u16;
        if self
            .info()
            .photo_interp()
            .is_some_and(|pi| *pi == PhotoInterp::Monochrome1)
        {
            !val
        } else {
            val
        }
    }

    pub fn get_pixel(&self, src_byte_index: usize) -> Result<PixelU16, PixelDataError> {
        if src_byte_index >= self.buffer().len()
            || (self.info().planar_config() == 0
                && src_byte_index % self.info().samples_per_pixel() as usize != 0)
            || (self.info().planar_config() != 0
                && src_byte_index >= self.buffer().len() / self.info().samples_per_pixel() as usize)
        {
            return Err(PixelDataError::InvalidPixelSource(src_byte_index));
        }

        let mut dst_pixel_index: usize = src_byte_index;
        if self.interp_as_rgb && self.info().planar_config() == 0 {
            dst_pixel_index /= self.info().samples_per_pixel() as usize;
        }

        let x = dst_pixel_index % (self.info().cols() as usize);
        let y = dst_pixel_index / (self.info().cols() as usize);

        let stride = self.stride();
        let (r, g, b) = if self.interp_as_rgb {
            let r = self.buffer()[src_byte_index];
            let g = self.buffer()[src_byte_index + stride];
            let b = self.buffer()[src_byte_index + stride * 2];
            (r, g, b)
        } else {
            let val = self.normalize(self.buffer()[src_byte_index]);
            (val, val, val)
        };

        Ok(PixelU16 { x, y, r, g, b })
    }

    pub fn pixel_iter(&self) -> PixelDataBufferU16Iter {
        PixelDataBufferU16Iter {
            pdbuf: self,
            src_byte_index: 0,
        }
    }
}

pub struct PixelDataBufferU16Iter<'buf> {
    pdbuf: &'buf PixelDataBufferU16,
    src_byte_index: usize,
}

impl Iterator for PixelDataBufferU16Iter<'_> {
    type Item = PixelU16;

    fn next(&mut self) -> Option<Self::Item> {
        let pixel = self.pdbuf.get_pixel(self.src_byte_index);

        if self.pdbuf.interp_as_rgb && self.pdbuf.info().planar_config() == 0 {
            self.src_byte_index += self.pdbuf.info().samples_per_pixel() as usize;
        } else {
            // If planar config indicates that all R's are stored followed by all G's then all
            // B's, then next R pixel is the next element.
            self.src_byte_index += 1;
        }

        pixel.ok()
    }
}

pub struct PixelU32 {
    pub x: usize,
    pub y: usize,
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub struct PixelDataBufferU32 {
    info: PixelDataInfo,
    buffer: Vec<u32>,
    min: u32,
    max: u32,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataBufferU32 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataBufferU32")
            .field("info", &self.info)
            .field("buffer_len", &self.buffer.len())
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl PixelDataBufferU32 {
    pub fn new(info: PixelDataInfo, buffer: Vec<u32>, min: u32, max: u32) -> Self {
        let stride = if info.planar_config() == 0 {
            1
        } else {
            buffer.len() / info.samples_per_pixel() as usize
        };
        let interp_as_rgb =
            info.photo_interp().is_some_and(PhotoInterp::is_rgb) && info.samples_per_pixel() == 3;

        Self {
            info,
            buffer,
            min,
            max,
            stride,
            interp_as_rgb,
        }
    }

    pub fn info(&self) -> &PixelDataInfo {
        &self.info
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn normalize(&self, val: u32) -> u32 {
        let range: f32 = (self.max - self.min) as f32;
        let valz: f32 = (val - self.min) as f32;
        let val = (valz / range * (u32::MAX as f32)) as u32;
        if self
            .info()
            .photo_interp()
            .is_some_and(|pi| *pi == PhotoInterp::Monochrome1)
        {
            !val
        } else {
            val
        }
    }

    pub fn get_pixel(&self, src_byte_index: usize) -> Result<PixelU32, PixelDataError> {
        if src_byte_index >= self.buffer().len()
            || (self.info().planar_config() == 0
                && src_byte_index % self.info().samples_per_pixel() as usize != 0)
            || (self.info().planar_config() != 0
                && src_byte_index >= self.buffer().len() / self.info().samples_per_pixel() as usize)
        {
            return Err(PixelDataError::InvalidPixelSource(src_byte_index));
        }

        let mut dst_pixel_index: usize = src_byte_index;
        if self.interp_as_rgb && self.info().planar_config() == 0 {
            dst_pixel_index /= self.info().samples_per_pixel() as usize;
        }

        let x = dst_pixel_index % (self.info().cols() as usize);
        let y = dst_pixel_index / (self.info().cols() as usize);

        let stride = self.stride();
        let (r, g, b) = if self.interp_as_rgb {
            let r = self.buffer()[src_byte_index];
            let g = self.buffer()[src_byte_index + stride];
            let b = self.buffer()[src_byte_index + stride * 2];
            (r, g, b)
        } else {
            let val = self.normalize(self.buffer()[src_byte_index]);
            (val, val, val)
        };

        Ok(PixelU32 { x, y, r, g, b })
    }

    pub fn pixel_iter(&self) -> PixelDataBufferU32Iter {
        PixelDataBufferU32Iter {
            pdbuf: self,
            src_byte_index: 0,
        }
    }
}

pub struct PixelDataBufferU32Iter<'buf> {
    pdbuf: &'buf PixelDataBufferU32,
    src_byte_index: usize,
}

impl Iterator for PixelDataBufferU32Iter<'_> {
    type Item = PixelU32;

    fn next(&mut self) -> Option<Self::Item> {
        let pixel = self.pdbuf.get_pixel(self.src_byte_index);

        if self.pdbuf.interp_as_rgb && self.pdbuf.info().planar_config() == 0 {
            self.src_byte_index += self.pdbuf.info().samples_per_pixel() as usize;
        } else {
            // If planar config indicates that all R's are stored followed by all G's then all
            // B's, then next R pixel is the next element.
            self.src_byte_index += 1;
        }

        pixel.ok()
    }
}

/// Container for the raw pixel values parsed from the DICOM binary data.
#[derive(Debug)]
pub enum PixelDataBuffer {
    U8(PixelDataBufferU8),
    U16(PixelDataBufferU16),
    U32(PixelDataBufferU32),
}

impl PixelDataBuffer {
    /// Shift an i8 value into u8 space, so i8::MIN -> u8::MIN.
    pub(crate) fn shift_i8(val: i8) -> u8 {
        ((val as i16).saturating_add(1) + (i8::MAX as i16)) as u8
    }

    /// Shift an i16 value into u16 space, so i16::MIN -> u16::MIN.
    pub(crate) fn shift_i16(val: i16) -> u16 {
        ((val as i32).saturating_add(1) + (i16::MAX as i32)) as u16
    }

    /// Shift an i32 value into u32 space, so i32::MIN -> u32::MIN.
    pub(crate) fn shift_i32(val: i32) -> u32 {
        ((val as i64).saturating_add(1) + (i32::MAX as i64)) as u32
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
