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
pub struct PixelI16 {
    pub x: usize,
    pub y: usize,
    pub r: i16,
    pub g: i16,
    pub b: i16,
}

pub struct PixelDataBufferI16 {
    info: PixelDataInfo,
    buffer: Vec<i16>,
    min: i16,
    max: i16,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataBufferI16 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataBufferI16")
            .field("info", &self.info)
            .field("buffer_len", &self.buffer.len())
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl PixelDataBufferI16 {
    pub fn new(info: PixelDataInfo, buffer: Vec<i16>, min: i16, max: i16) -> Self {
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

    pub fn buffer(&self) -> &[i16] {
        &self.buffer
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn normalize(&self, val: i16) -> i16 {
        let range: f32 = (self.max - self.min) as f32;
        let valz: f32 = (val - self.min) as f32;
        let val = (valz / range * (i16::MAX as f32)) as i16;
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

    pub fn get_pixel(&self, src_byte_index: usize) -> Result<PixelI16, PixelDataError> {
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

        Ok(PixelI16 { x, y, r, g, b })
    }

    pub fn pixel_iter(&self) -> PixelDataBufferI16Iter {
        PixelDataBufferI16Iter {
            pdbuf: self,
            src_byte_index: 0,
        }
    }
}

pub struct PixelDataBufferI16Iter<'buf> {
    pdbuf: &'buf PixelDataBufferI16,
    src_byte_index: usize,
}

impl Iterator for PixelDataBufferI16Iter<'_> {
    type Item = PixelI16;

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
