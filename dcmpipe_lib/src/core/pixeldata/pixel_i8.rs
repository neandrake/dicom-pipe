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
pub struct PixelI8 {
    pub x: usize,
    pub y: usize,
    pub r: i8,
    pub g: i8,
    pub b: i8,
}

pub struct PixelDataSliceI8 {
    info: PixelDataInfo,
    buffer: Vec<i8>,
    min: i8,
    max: i8,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataSliceI8 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataBufferI8")
            .field("info", &self.info)
            .field("buffer_len", &self.buffer.len())
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl PixelDataSliceI8 {
    pub fn new(info: PixelDataInfo, buffer: Vec<i8>, min: i8, max: i8) -> Self {
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

    pub fn buffer(&self) -> &[i8] {
        &self.buffer
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn normalize(&self, val: i8) -> i8 {
        let range: f32 = self.max as f32 - self.min as f32;
        let normalized: f32 = (val as f32 - self.min as f32) / range;
        let range: f32 = i8::MAX as f32 - i8::MIN as f32;
        let mut denormalized: f32 = normalized * range + i8::MIN as f32;
        if let Some(slope) = self.info().slope() {
            if let Some(intercept) = self.info().intercept() {
                denormalized = (denormalized as f64 * slope + intercept) as f32;
            }
        }
        let denormalized = denormalized as i8;
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

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<PixelI8, PixelDataError> {
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

        Ok(PixelI8 { x, y, r, g, b })
    }

    pub fn pixel_iter(&self) -> SlicePixelI8Iter {
        SlicePixelI8Iter {
            slice: self,
            src_byte_index: 0,
        }
    }
}

pub struct SlicePixelI8Iter<'buf> {
    slice: &'buf PixelDataSliceI8,
    src_byte_index: usize,
}

impl Iterator for SlicePixelI8Iter<'_> {
    type Item = PixelI8;

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
