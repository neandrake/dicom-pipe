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
    pdinfo::{PixelDataSliceInfo, I16_SIZE, U16_SIZE},
    pdslice::PixelDataSlice,
    PhotoInterp, PixelDataError,
};

#[derive(Debug)]
pub struct PixelU16 {
    pub x: usize,
    pub y: usize,
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

pub struct PixelDataSliceU16 {
    info: PixelDataSliceInfo,
    buffer: Vec<u16>,
    min: u16,
    max: u16,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataSliceU16 {
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

impl PixelDataSliceU16 {
    pub fn from_mono_16bit(pdinfo: PixelDataSliceInfo) -> Result<Self, PixelDataError> {
        let len = Into::<usize>::into(pdinfo.cols()) * Into::<usize>::into(pdinfo.rows());
        let mut in_pos: usize = 0;
        let mut buffer: Vec<u16> = Vec::with_capacity(len * pdinfo.samples_per_pixel() as usize);
        for _i in 0..len {
            for _j in 0..pdinfo.samples_per_pixel() {
                let val = if pdinfo.big_endian() {
                    if pdinfo.is_signed() {
                        let val = PixelDataSlice::shift_i16(i16::from_be_bytes(
                            pdinfo.bytes()[in_pos..in_pos + I16_SIZE].try_into()?,
                        ));
                        in_pos += I16_SIZE;
                        val
                    } else {
                        let val = u16::from_be_bytes(
                            pdinfo.bytes()[in_pos..in_pos + U16_SIZE].try_into()?,
                        );
                        in_pos += U16_SIZE;
                        val
                    }
                } else if pdinfo.is_signed() {
                    let val = PixelDataSlice::shift_i16(i16::from_le_bytes(
                        pdinfo.bytes()[in_pos..in_pos + I16_SIZE].try_into()?,
                    ));
                    in_pos += I16_SIZE;
                    val
                } else {
                    let val =
                        u16::from_le_bytes(pdinfo.bytes()[in_pos..in_pos + U16_SIZE].try_into()?);
                    in_pos += U16_SIZE;
                    val
                };
                buffer.push(val);
            }
        }
        Ok(Self::new(pdinfo, buffer, u16::MIN, u16::MAX))
    }

    pub fn new(info: PixelDataSliceInfo, buffer: Vec<u16>, min: u16, max: u16) -> Self {
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

    pub fn info(&self) -> &PixelDataSliceInfo {
        &self.info
    }

    pub fn buffer(&self) -> &[u16] {
        &self.buffer
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn normalize(&self, val: u16) -> u16 {
        let range: f32 = self.max as f32 - self.min as f32;
        let normalized: f32 = (val as f32 - self.min as f32) / range;
        let range: f32 = u16::MAX as f32 - u16::MIN as f32;
        let mut denormalized: f32 = normalized * range + u16::MIN as f32;
        if let Some(slope) = self.info().slope() {
            if let Some(intercept) = self.info().intercept() {
                denormalized = (denormalized as f64 * slope + intercept) as f32;
            }
        }
        let denormalized = denormalized as u16;
        if self
            .info()
            .photo_interp()
            .is_some_and(|pi| *pi == PhotoInterp::Monochrome1)
        {
            !denormalized
        } else {
            denormalized
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<PixelU16, PixelDataError> {
        let cols = self.info().cols() as usize;
        let rows = self.info().rows() as usize;

        let src_byte_index = x * rows + y;
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

        let x = dst_pixel_index % cols;
        let y = dst_pixel_index / cols;

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

    pub fn pixel_iter(&self) -> SlicePixelU16Iter {
        SlicePixelU16Iter {
            slice: self,
            src_byte_index: 0,
        }
    }
}

pub struct SlicePixelU16Iter<'buf> {
    slice: &'buf PixelDataSliceU16,
    src_byte_index: usize,
}

impl Iterator for SlicePixelU16Iter<'_> {
    type Item = PixelU16;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.src_byte_index / self.slice.info().cols() as usize;
        let y = self.src_byte_index % self.slice.info().cols() as usize;
        let pixel = self.slice.get_pixel(x, y);

        if self.slice.interp_as_rgb && self.slice.info().planar_config() == 0 {
            self.src_byte_index += self.slice.info().samples_per_pixel() as usize;
        } else {
            // If planar config indicates that all R's are stored followed by all G's then all
            // B's, then next R pixel is the next element.
            self.src_byte_index += 1;
        }

        pixel.ok()
    }
}
