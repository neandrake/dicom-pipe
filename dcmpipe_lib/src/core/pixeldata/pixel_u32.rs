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

#[derive(Debug)]
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
        let range: f32 = self.max as f32 - self.min as f32;
        let normalized: f32 = (val as f32 - self.min as f32) / range;
        let range: f32 = u32::MAX as f32 - u32::MIN as f32;
        let mut denormalized: f32 = normalized * range + (u32::MIN as f32);
        if let Some(slope) = self.info().slope() {
            if let Some(intercept) = self.info().intercept() {
                denormalized = (denormalized as f64 * slope + intercept) as f32;
            }
        }
        let denormalized = denormalized as u32;
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
