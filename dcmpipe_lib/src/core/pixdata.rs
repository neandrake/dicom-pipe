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

use std::io::Read;

use thiserror::Error;

use crate::{
    core::{
        dcmelement::DicomElement,
        defn::vr::{self, VRRef},
        read::{ParseError, Parser},
        RawValue,
    },
    dict::tags,
};

#[derive(Error, Debug)]
pub enum PixelDataError {
    #[error("No Pixel Data bytes found")]
    MissingPixelData,

    #[error("Invalid dimensions: {0}x{1}")]
    InvalidSize(u16, u16),

    #[error("Invalid VR: {0:?}")]
    InvalidVR(VRRef),

    #[error("Invalid Bits Allocated: {0}")]
    InvalidBitsAlloc(u16),

    #[error("Invalid Photometric Interpretation and Samples per Pixel combo: {0:?}, {1}")]
    InvalidPhotoInterpSamples(PhotoInterp, u16),

    #[error("Error parsing DICOM")]
    ParseError {
        #[from]
        source: ParseError,
    },

    #[error("Error interpreting bytes")]
    BytesError {
        #[from]
        source: std::array::TryFromSliceError,
    },
}

/// Supported values of Photometric Interpretation.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PhotoInterp {
    Unsupported(String),
    Rgb,
    Monochrome1,
    Monochrome2,
}

impl PhotoInterp {
    /// Whether this `PhotoInterp` is `RGB`.
    #[must_use]
    pub fn is_rgb(&self) -> bool {
        *self == PhotoInterp::Rgb
    }

    /// Whether this `PhotoInterp` is one of the supported monochrome values.
    #[must_use]
    pub fn is_monochrome(&self) -> bool {
        *self == PhotoInterp::Monochrome1 || *self == PhotoInterp::Monochrome2
    }
}

impl From<&str> for PhotoInterp {
    /// Parse Photometric Interpretation from its DICOM element value.
    #[must_use]
    fn from(value: &str) -> Self {
        if value == "RGB" {
            Self::Rgb
        } else if value == "MONOCHROME1" {
            Self::Monochrome1
        } else if value == "MONOCHROME2" {
            Self::Monochrome2
        } else {
            Self::Unsupported(value.to_owned())
        }
    }
}

/// Supported values of Bits Allocated.
#[derive(PartialEq, Eq)]
pub enum BitsAlloc {
    Unsupported(u16),
    Eight,
    Sixteen,
    ThirtyTwo,
}

impl std::fmt::Display for BitsAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::fmt::Debug for BitsAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(other) => write!(f, "BitsAlloc(Unsupported:{other})"),
            Self::Eight => write!(f, "BitsAlloc(8)"),
            Self::Sixteen => write!(f, "BitsAlloc(16)"),
            Self::ThirtyTwo => write!(f, "BitsAlloc(32)"),
        }
    }
}

impl BitsAlloc {
    /// Parse Bits Allocated from its DICOM value.
    #[must_use]
    pub fn from_val(val: u16) -> Self {
        match val {
            8 => BitsAlloc::Eight,
            16 => BitsAlloc::Sixteen,
            32 => BitsAlloc::ThirtyTwo,
            other => BitsAlloc::Unsupported(other),
        }
    }

    #[must_use]
    pub fn val(&self) -> u16 {
        match self {
            Self::Unsupported(val) => *val,
            Self::Eight => 8,
            Self::Sixteen => 16,
            Self::ThirtyTwo => 32,
        }
    }
}

/// Parsed tag values relevant to interpreting Pixel Data.
pub struct PixelDataInfo {
    big_endian: bool,
    vr: VRRef,
    samples_per_pixel: u16,
    photo_interp: Option<PhotoInterp>,
    planar_config: u16,
    cols: u16,
    rows: u16,
    pixel_padding_val: Option<u16>,
    bits_alloc: BitsAlloc,
    bits_stored: u16,
    high_bit: u16,
    slope: f64,
    pixel_rep: u16,
    intercept: f64,
    unit: String,
    window_centers: Vec<f32>,
    window_widths: Vec<f32>,
    window_labels: Vec<String>,
    pd_bytes: Vec<u8>,
}

impl Default for PixelDataInfo {
    fn default() -> Self {
        Self {
            big_endian: false,
            vr: &vr::OB,
            samples_per_pixel: 0,
            photo_interp: None,
            planar_config: 0,
            cols: 0,
            rows: 0,
            pixel_padding_val: None,
            bits_alloc: BitsAlloc::Unsupported(0),
            bits_stored: 0,
            high_bit: 0,
            pixel_rep: 0,
            slope: 0.0,
            intercept: 0.0,
            unit: String::new(),
            window_centers: Vec::with_capacity(0),
            window_widths: Vec::with_capacity(0),
            window_labels: Vec::with_capacity(0),
            pd_bytes: Vec::with_capacity(0),
        }
    }
}

impl std::fmt::Debug for PixelDataInfo {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataInfo")
            .field("big_endian", &self.big_endian)
            .field("vr", &self.vr)
            .field("samples_per_pixel", &self.samples_per_pixel)
            .field("photo_interp", &self.photo_interp)
            .field("planar_config", &self.planar_config)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .field("pixel_padding_val", &self.pixel_padding_val)
            .field("bits_alloc", &self.bits_alloc)
            .field("bits_stored", &self.bits_stored)
            .field("high_bit", &self.high_bit)
            .field("pixel_rep", &self.pixel_rep)
            .field("slope", &self.slope)
            .field("intercept", &self.intercept)
            .field("unit", &self.unit)
            .field("window_centers", &self.window_centers)
            .field("window_widths", &self.window_widths)
            .field("window_labels", &self.window_labels)
            .field("pd_bytes", &self.pd_bytes.len())
            .finish()
    }
}

impl PixelDataInfo {
    #[must_use]
    pub fn big_endian(&self) -> bool {
        self.big_endian
    }

    #[must_use]
    pub fn vr(&self) -> VRRef {
        self.vr
    }

    #[must_use]
    pub fn samples_per_pixel(&self) -> u16 {
        self.samples_per_pixel
    }

    #[must_use]
    pub fn photo_interp(&self) -> Option<&PhotoInterp> {
        self.photo_interp.as_ref()
    }

    #[must_use]
    pub fn planar_config(&self) -> u16 {
        self.planar_config
    }

    #[must_use]
    pub fn cols(&self) -> u16 {
        self.cols
    }

    #[must_use]
    pub fn rows(&self) -> u16 {
        self.rows
    }

    #[must_use]
    pub fn pixel_padding_val(&self) -> Option<u16> {
        self.pixel_padding_val
    }

    #[must_use]
    pub fn bits_alloc(&self) -> &BitsAlloc {
        &self.bits_alloc
    }

    #[must_use]
    pub fn bits_stored(&self) -> u16 {
        self.bits_stored
    }

    #[must_use]
    pub fn high_bit(&self) -> u16 {
        self.high_bit
    }

    #[must_use]
    pub fn slope(&self) -> f64 {
        self.slope
    }

    #[must_use]
    pub fn pixel_rep(&self) -> u16 {
        self.pixel_rep
    }

    #[must_use]
    pub fn intercept(&self) -> f64 {
        self.intercept
    }

    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }

    #[must_use]
    pub fn window_centers(&self) -> &[f32] {
        &self.window_centers
    }

    #[must_use]
    pub fn window_widths(&self) -> &[f32] {
        &self.window_widths
    }

    #[must_use]
    pub fn window_labels(&self) -> &[String] {
        &self.window_labels
    }
    /// Whether the byte values in Pixel Data are signed or unsigned values.
    #[must_use]
    pub fn is_signed(&self) -> bool {
        self.pixel_rep != 0
    }

    /// After all relevant elements have been parsed, this will validate the result of this
    /// structure.
    ///
    /// # Errors
    /// - This function returns errors in the validation of values parsed from DICOM elements via
    ///   `PixelDataInfo::process_dcm_parser`.
    pub fn validate(&mut self) -> Result<(), PixelDataError> {
        if self.pd_bytes.is_empty() {
            return Err(PixelDataError::MissingPixelData);
        }

        if self.cols == 0 || self.rows == 0 {
            return Err(PixelDataError::InvalidSize(self.cols, self.rows));
        }

        if self.vr != &vr::OB && self.vr != &vr::OW {
            return Err(PixelDataError::InvalidVR(self.vr));
        };

        if let BitsAlloc::Unsupported(val) = self.bits_alloc {
            return Err(PixelDataError::InvalidBitsAlloc(val));
        }

        // BitsStored will generally be the same value as BitsAllocated.
        if self.bits_stored > self.bits_alloc.val() || self.bits_stored == 0 {
            self.bits_stored = self.bits_alloc.val();
        }
        // HighBit will generally be (BitsStored - 1).
        if self.high_bit > self.bits_alloc.val() - 1 || self.high_bit < self.bits_stored - 1 {
            self.high_bit = self.bits_stored - 1;
        }

        if let Some(pi) = &self.photo_interp {
            if (pi.is_rgb() && self.samples_per_pixel != 3)
                || (pi.is_monochrome() && self.samples_per_pixel != 1)
            {
                // RGB must use 3 Samples Per Pixel.
                // MONOCHROME1/2 must use 1 Sample Per Pixel.
                return Err(PixelDataError::InvalidPhotoInterpSamples(
                    pi.clone(),
                    self.samples_per_pixel,
                ));
            }
        }

        Ok(())
    }

    /// Loads the pixel data bytes into `PixelDataBuffer`.
    ///
    /// # Errors
    /// - If BitsAllocated is unsupported (though this should already be caught by
    ///   `PixelDataInfo::validate()`.
    /// - Reading byte/word values from the Pixel Data bytes.
    pub fn load_pixel_data(mut self) -> Result<PixelDataBuffer, PixelDataError> {
        self.validate()?;
        let mut in_pos: usize = 0;
        let len = Into::<usize>::into(self.cols) * Into::<usize>::into(self.rows);
        match self.bits_alloc {
            BitsAlloc::Unsupported(val) => Err(PixelDataError::InvalidBitsAlloc(val)),
            BitsAlloc::Eight => {
                let mut buffer: Vec<u8> = Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: u8 = u8::MAX;
                let mut max: u8 = u8::MIN;
                let pixel_pad_val = self
                    .pixel_padding_val
                    .and_then(|pad_val| TryInto::<u8>::try_into(pad_val).ok());
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let val = if self.is_signed() {
                            PixelDataBuffer::shift_i8(self.pd_bytes[in_pos] as i8)
                        } else {
                            self.pd_bytes[in_pos]
                        };
                        in_pos += 1;
                        buffer.push(val);
                        if pixel_pad_val.is_none_or(|pad_val| val != pad_val) {
                            if val < min {
                                min = val;
                            }
                            if val > max {
                                max = val;
                            }
                        }
                    }
                }
                let stride = if self.planar_config() == 0 {
                    1
                } else {
                    buffer.len() / self.samples_per_pixel() as usize
                };
                Ok(PixelDataBuffer::U8(PixelDataBufferU8 {
                    info: self,
                    buffer,
                    min,
                    max,
                    stride,
                    src_byte_index: 0,
                    dst_pixel_index: 0,
                }))
            }
            BitsAlloc::Sixteen => {
                let mut buffer: Vec<u16> =
                    Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: u16 = u16::MAX;
                let mut max: u16 = u16::MIN;
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let val = if self.is_signed() {
                            let val = if self.big_endian {
                                i16::from_be_bytes(self.pd_bytes[in_pos..in_pos + 2].try_into()?)
                            } else {
                                i16::from_le_bytes(self.pd_bytes[in_pos..in_pos + 2].try_into()?)
                            };
                            PixelDataBuffer::shift_i16(val)
                        } else if self.big_endian {
                            u16::from_be_bytes(self.pd_bytes[in_pos..in_pos + 2].try_into()?)
                        } else {
                            u16::from_le_bytes(self.pd_bytes[in_pos..in_pos + 2].try_into()?)
                        };
                        in_pos += 2;
                        buffer.push(val);
                        if self.pixel_padding_val.is_none_or(|pad_val| val != pad_val) {
                            if val < min {
                                min = val;
                            }
                            if val > max {
                                max = val;
                            }
                        }
                    }
                }
                let stride = if self.planar_config() == 0 {
                    1
                } else {
                    buffer.len() / self.samples_per_pixel() as usize
                };
                Ok(PixelDataBuffer::U16(PixelDataBufferU16 {
                    info: self,
                    buffer,
                    min,
                    max,
                    stride,
                    src_byte_index: 0,
                    dst_pixel_index: 0,
                }))
            }
            BitsAlloc::ThirtyTwo => {
                let mut buffer: Vec<u32> =
                    Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: u32 = u32::MAX;
                let mut max: u32 = u32::MIN;
                let pixel_pad_val = self.pixel_padding_val.map(Into::<u32>::into);
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let val = if self.is_signed() {
                            let val = if self.big_endian {
                                i32::from_be_bytes(self.pd_bytes[in_pos..in_pos + 4].try_into()?)
                            } else {
                                i32::from_le_bytes(self.pd_bytes[in_pos..in_pos + 4].try_into()?)
                            };
                            PixelDataBuffer::shift_i32(val)
                        } else if self.big_endian {
                            u32::from_be_bytes(self.pd_bytes[in_pos..in_pos + 4].try_into()?)
                        } else {
                            u32::from_le_bytes(self.pd_bytes[in_pos..in_pos + 4].try_into()?)
                        };
                        in_pos += 4;
                        buffer.push(val);
                        if pixel_pad_val.is_none_or(|pad_val| val != pad_val) {
                            if val < min {
                                min = val;
                            }
                            if val > max {
                                max = val;
                            }
                        }
                    }
                }
                let stride = if self.planar_config() == 0 {
                    1
                } else {
                    buffer.len() / self.samples_per_pixel() as usize
                };
                Ok(PixelDataBuffer::U32(PixelDataBufferU32 {
                    info: self,
                    buffer,
                    min,
                    max,
                    stride,
                    src_byte_index: 0,
                    dst_pixel_index: 0,
                }))
            }
        }
    }

    /// Processes a DICOM SOP via a `Parser` into a `PixelDataInfo`.
    ///
    /// # Errors
    /// - I/O errors parsing values out of DICOM elements.
    pub fn process_dcm_parser<R: Read>(
        parser: Parser<'_, R>,
    ) -> Result<PixelDataInfo, PixelDataError> {
        let mut pixdata_info: PixelDataInfo = PixelDataInfo {
            big_endian: parser.ts().big_endian(),
            ..Default::default()
        };
        for elem in parser {
            let mut elem = elem?;
            Self::process_element(&mut pixdata_info, &elem)?;
            if elem.is_pixel_data() || elem.is_within_pixel_data() {
                Self::process_pixdata_element(&mut pixdata_info, &mut elem);
            }
        }
        Ok(pixdata_info)
    }

    /// Process relevant DICOM elements into the `PixelDataInfo` structure.
    ///
    /// # Errors
    /// - I/O errors parsing values out of DICOM elements.
    fn process_element(
        pixdata_info: &mut PixelDataInfo,
        elem: &DicomElement,
    ) -> Result<(), PixelDataError> {
        // The order of the tag checks here are the order they will appear in a DICOM protocol.
        if elem.tag() == tags::SamplesperPixel.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.samples_per_pixel = val;
            }
        } else if elem.tag() == tags::PhotometricInterpretation.tag() {
            if let Some(val) = elem.parse_value()?.string() {
                pixdata_info.photo_interp = Some(Into::<PhotoInterp>::into(val.as_str()));
            }
        } else if elem.tag() == tags::PlanarConfiguration.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.planar_config = val;
            }
        } else if elem.tag() == tags::Rows.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.rows = val;
            }
        } else if elem.tag() == tags::Columns.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.cols = val;
            }
        } else if elem.tag() == tags::BitsAllocated.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.bits_alloc = BitsAlloc::from_val(val);
            }
        } else if elem.tag() == tags::BitsStored.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.bits_stored = val;
            }
        } else if elem.tag() == tags::HighBit.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.high_bit = val;
            }
        } else if elem.tag() == tags::PixelRepresentation.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.pixel_rep = val;
            }
        } else if elem.tag() == tags::PixelPaddingValue.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.pixel_padding_val = Some(val);
            }
        } else if elem.tag() == tags::WindowCenter.tag() {
            if let RawValue::Floats(vals) = elem.parse_value()? {
                pixdata_info.window_centers = vals;
            }
        } else if elem.tag() == tags::WindowWidth.tag() {
            if let RawValue::Floats(vals) = elem.parse_value()? {
                pixdata_info.window_widths = vals;
            }
        } else if elem.tag() == tags::RescaleIntercept.tag() {
            if let Some(val) = elem.parse_value()?.double() {
                pixdata_info.intercept = val;
            }
        } else if elem.tag() == tags::RescaleSlope.tag() {
            if let Some(val) = elem.parse_value()?.double() {
                pixdata_info.slope = val;
            }
        } else if elem.tag() == tags::RescaleType.tag() || elem.tag() == tags::Units.tag() {
            if let Some(val) = elem.parse_value()?.string() {
                // Only use Units if RescaleType wasn't present.
                if pixdata_info.unit.is_empty() {
                    pixdata_info.unit = val.to_owned();
                }
            }
        } else if elem.tag() == tags::WindowCenter_and_WidthExplanation.tag() {
            if let RawValue::Strings(vals) = elem.parse_value()? {
                pixdata_info.window_labels = vals;
            }
        }

        Ok(())
    }

    /// Process the relevant PixelData element/fragments by copying the data/bytes into the
    /// `PixelDataInfo::pd_bytes` field, replacing the element's data/bytes with an empty vec.
    fn process_pixdata_element(pixdata_info: &mut PixelDataInfo, elem: &mut DicomElement) {
        // Transfer ownership of the fragment's bytes to a local to copy into pixdata_info.pd_bytes.
        let data = std::mem::replace(elem.mut_data(), Vec::with_capacity(0));
        pixdata_info.pd_bytes.extend_from_slice(&data);
    }
}

pub struct PixelU8 {
    pub x: u32,
    pub y: u32,
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
    src_byte_index: usize,
    dst_pixel_index: usize,
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
    pub fn info(&self) -> &PixelDataInfo {
        &self.info
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn range(&self) -> u8 {
        self.max - self.min
    }
}

impl Iterator for PixelDataBufferU8 {
    type Item = PixelU8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.src_byte_index >= self.buffer().len() {
            return None;
        }
        let x = (self.dst_pixel_index as u32) % (self.info().cols() as u32);
        let y = (self.dst_pixel_index as u32) / (self.info().cols() as u32);
        self.dst_pixel_index += 1;
        if self.info().photo_interp().is_some_and(PhotoInterp::is_rgb)
            && self.info().samples_per_pixel() == 3
        {
            let r = self.buffer()[self.src_byte_index];
            let g = self.buffer()[self.src_byte_index + self.stride];
            let b = self.buffer()[self.src_byte_index + self.stride * 2];
            if self.info().planar_config() == 0 {
                self.src_byte_index += self.info().samples_per_pixel() as usize;
            } else {
                self.src_byte_index += 1;
            }
            Some(PixelU8 { x, y, r, g, b })
        } else if self
            .info()
            .photo_interp()
            .is_some_and(PhotoInterp::is_monochrome)
            && self.info().samples_per_pixel() == 1
        {
            let val = self.buffer()[self.src_byte_index];
            let val = ((val as f32 / self.range() as f32) * u8::MAX as f32) as u8;
            let r = val;
            let g = val;
            let b = val;
            self.src_byte_index += 1;
            Some(PixelU8 { x, y, r, g, b })
        } else {
            None
        }
    }
}

pub struct PixelU16 {
    pub x: u32,
    pub y: u32,
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
    src_byte_index: usize,
    dst_pixel_index: usize,
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
    pub fn info(&self) -> &PixelDataInfo {
        &self.info
    }

    pub fn buffer(&self) -> &[u16] {
        &self.buffer
    }

    pub fn range(&self) -> u16 {
        self.max - self.min
    }
}

impl Iterator for PixelDataBufferU16 {
    type Item = PixelU16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.src_byte_index >= self.buffer().len() {
            return None;
        }
        let x = (self.dst_pixel_index as u32) % (self.info().cols() as u32);
        let y = (self.dst_pixel_index as u32) / (self.info().cols() as u32);
        self.dst_pixel_index += 1;
        if self.info().photo_interp().is_some_and(PhotoInterp::is_rgb)
            && self.info().samples_per_pixel() == 3
        {
            let r = self.buffer()[self.src_byte_index];
            let g = self.buffer()[self.src_byte_index + self.stride];
            let b = self.buffer()[self.src_byte_index + self.stride * 2];
            if self.info().planar_config() == 0 {
                self.src_byte_index += self.info().samples_per_pixel() as usize;
            } else {
                self.src_byte_index += 1;
            }
            Some(PixelU16 { x, y, r, g, b })
        } else if self
            .info()
            .photo_interp()
            .is_some_and(PhotoInterp::is_monochrome)
            && self.info().samples_per_pixel() == 1
        {
            let val = self.buffer()[self.src_byte_index];
            let val = ((val as f32 / self.range() as f32) * u16::MAX as f32) as u16;
            let r = val;
            let g = val;
            let b = val;
            self.src_byte_index += 1;
            Some(PixelU16 { x, y, r, g, b })
        } else {
            None
        }
    }
}

pub struct PixelU32 {
    pub x: u32,
    pub y: u32,
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
    src_byte_index: usize,
    dst_pixel_index: usize,
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
    pub fn info(&self) -> &PixelDataInfo {
        &self.info
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn range(&self) -> u32 {
        self.max - self.min
    }
}

impl Iterator for PixelDataBufferU32 {
    type Item = PixelU32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.src_byte_index >= self.buffer().len() {
            return None;
        }
        let x = (self.dst_pixel_index as u32) % (self.info().cols() as u32);
        let y = (self.dst_pixel_index as u32) / (self.info().cols() as u32);
        self.dst_pixel_index += 1;
        if self.info().photo_interp().is_some_and(PhotoInterp::is_rgb)
            && self.info().samples_per_pixel() == 3
        {
            let r = self.buffer()[self.src_byte_index];
            let g = self.buffer()[self.src_byte_index + self.stride];
            let b = self.buffer()[self.src_byte_index + self.stride * 2];
            if self.info().planar_config() == 0 {
                self.src_byte_index += self.info().samples_per_pixel() as usize;
            } else {
                self.src_byte_index += 1;
            }
            Some(PixelU32 { x, y, r, g, b })
        } else if self
            .info()
            .photo_interp()
            .is_some_and(PhotoInterp::is_monochrome)
            && self.info().samples_per_pixel() == 1
        {
            let val = self.buffer()[self.src_byte_index];
            let val = ((val as f32 / self.range() as f32) * u32::MAX as f32) as u32;
            let r = val;
            let g = val;
            let b = val;
            self.src_byte_index += 1;
            Some(PixelU32 { x, y, r, g, b })
        } else {
            None
        }
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
    fn shift_i8(val: i8) -> u8 {
        ((val as i16).saturating_add(1) + (i8::MAX as i16)) as u8
    }

    /// Shift an i16 value into u16 space, so i16::MIN -> u16::MIN.
    fn shift_i16(val: i16) -> u16 {
        ((val as i32).saturating_add(1) + (i16::MAX as i32)) as u16
    }

    /// Shift an i32 value into u32 space, so i32::MIN -> u32::MIN.
    fn shift_i32(val: i32) -> u32 {
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
