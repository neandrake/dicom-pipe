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

use crate::{
    core::{
        dcmelement::DicomElement,
        defn::vr::{self, VRRef},
        pixeldata::{
            pdbuf::PixelDataBuffer, pixel_i16::PixelDataBufferI16, pixel_i32::PixelDataBufferI32,
            pixel_i8::PixelDataBufferI8, pixel_u16::PixelDataBufferU16,
            pixel_u32::PixelDataBufferU32, pixel_u8::PixelDataBufferU8, BitsAlloc, PhotoInterp,
            PixelDataError,
        },
        read::Parser,
        RawValue,
    },
    dict::tags,
};

const I8_SIZE: usize = size_of::<i8>();
const I16_SIZE: usize = size_of::<i16>();
const I32_SIZE: usize = size_of::<i32>();
const U8_SIZE: usize = size_of::<u8>();
const U16_SIZE: usize = size_of::<u16>();
const U32_SIZE: usize = size_of::<u32>();

/// Parsed tag values relevant to interpreting Pixel Data, including the raw `PixelData` bytes.
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
    pixel_rep: u16,
    slope: Option<f64>,
    intercept: Option<f64>,
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
            slope: None,
            intercept: None,
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
    pub fn pixel_rep(&self) -> u16 {
        self.pixel_rep
    }

    #[must_use]
    pub fn slope(&self) -> Option<f64> {
        self.slope
    }

    #[must_use]
    pub fn intercept(&self) -> Option<f64> {
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
        let is_signed = self.is_signed();
        let mut in_pos: usize = 0;
        let len = Into::<usize>::into(self.cols) * Into::<usize>::into(self.rows);
        match (self.bits_alloc, is_signed) {
            (BitsAlloc::Unsupported(val), _) => Err(PixelDataError::InvalidBitsAlloc(val)),
            (BitsAlloc::Eight, true) => {
                let mut buffer: Vec<i8> = Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: i8 = i8::MAX;
                let mut max: i8 = i8::MIN;
                let pixel_pad_val = self
                    .pixel_padding_val
                    .and_then(|pad_val| TryInto::<i8>::try_into(pad_val).ok());
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let mut val = self.pd_bytes[in_pos] as i8;
                        if let Some(slope) = self.slope {
                            if let Some(intercept) = self.intercept {
                                val = (val as f64 * slope + intercept) as i8;
                            }
                        }
                        in_pos += I8_SIZE;
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
                Ok(PixelDataBuffer::I8(PixelDataBufferI8::new(
                    self, buffer, min, max,
                )))
            }
            (BitsAlloc::Eight, false) => {
                let mut buffer: Vec<u8> = Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: u8 = u8::MAX;
                let mut max: u8 = u8::MIN;
                let pixel_pad_val = self
                    .pixel_padding_val
                    .and_then(|pad_val| TryInto::<u8>::try_into(pad_val).ok());
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let mut val = self.pd_bytes[in_pos];
                        if let Some(slope) = self.slope {
                            if let Some(intercept) = self.intercept {
                                val = (val as f64 * slope + intercept) as u8;
                            }
                        }
                        in_pos += U8_SIZE;
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
                Ok(PixelDataBuffer::U8(PixelDataBufferU8::new(
                    self, buffer, min, max,
                )))
            }
            (BitsAlloc::Sixteen, true) => {
                let mut buffer: Vec<i16> =
                    Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: i16 = i16::MAX;
                let mut max: i16 = i16::MIN;
                let pixel_pad_val = self
                    .pixel_padding_val
                    .and_then(|pad_val| TryInto::<i16>::try_into(pad_val).ok());
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let mut val = if self.big_endian {
                            i16::from_be_bytes(self.pd_bytes[in_pos..in_pos + I16_SIZE].try_into()?)
                        } else {
                            i16::from_le_bytes(self.pd_bytes[in_pos..in_pos + I16_SIZE].try_into()?)
                        };
                        if let Some(slope) = self.slope {
                            if let Some(intercept) = self.intercept {
                                val = (val as f64 * slope + intercept) as i16;
                            }
                        }
                        in_pos += I16_SIZE;
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
                Ok(PixelDataBuffer::I16(PixelDataBufferI16::new(
                    self, buffer, min, max,
                )))
            }
            (BitsAlloc::Sixteen, false) => {
                let mut buffer: Vec<u16> =
                    Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: u16 = u16::MAX;
                let mut max: u16 = u16::MIN;
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let val = if self.big_endian {
                            let mut val = u16::from_be_bytes(
                                self.pd_bytes[in_pos..in_pos + U16_SIZE].try_into()?,
                            );
                            if let Some(slope) = self.slope {
                                if let Some(intercept) = self.intercept {
                                    val = (val as f64 * slope + intercept) as u16;
                                }
                            }
                            val
                        } else {
                            let mut val = u16::from_le_bytes(
                                self.pd_bytes[in_pos..in_pos + U16_SIZE].try_into()?,
                            );
                            if let Some(slope) = self.slope {
                                if let Some(intercept) = self.intercept {
                                    val = (val as f64 * slope + intercept) as u16;
                                }
                            }
                            val
                        };
                        in_pos += U16_SIZE;
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
                Ok(PixelDataBuffer::U16(PixelDataBufferU16::new(
                    self, buffer, min, max,
                )))
            }
            (BitsAlloc::ThirtyTwo, true) => {
                let mut buffer: Vec<i32> =
                    Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: i32 = i32::MAX;
                let mut max: i32 = i32::MIN;
                let pixel_pad_val = self.pixel_padding_val.map(Into::<i32>::into);
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let mut val = if self.big_endian {
                            i32::from_be_bytes(self.pd_bytes[in_pos..in_pos + I32_SIZE].try_into()?)
                        } else {
                            i32::from_le_bytes(self.pd_bytes[in_pos..in_pos + I32_SIZE].try_into()?)
                        };
                        if let Some(slope) = self.slope {
                            if let Some(intercept) = self.intercept {
                                val = (val as f64 * slope + intercept) as i32;
                            }
                        }
                        in_pos += I32_SIZE;
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
                Ok(PixelDataBuffer::I32(PixelDataBufferI32::new(
                    self, buffer, min, max,
                )))
            }
            (BitsAlloc::ThirtyTwo, false) => {
                let mut buffer: Vec<u32> =
                    Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: u32 = u32::MAX;
                let mut max: u32 = u32::MIN;
                let pixel_pad_val = self.pixel_padding_val.map(Into::<u32>::into);
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let val = if self.big_endian {
                            let mut val = u32::from_be_bytes(
                                self.pd_bytes[in_pos..in_pos + U32_SIZE].try_into()?,
                            );
                            if let Some(slope) = self.slope {
                                if let Some(intercept) = self.intercept {
                                    val = (val as f64 * slope + intercept) as u32;
                                }
                            }
                            val
                        } else {
                            let mut val = u32::from_le_bytes(
                                self.pd_bytes[in_pos..in_pos + U32_SIZE].try_into()?,
                            );
                            if let Some(slope) = self.slope {
                                if let Some(intercept) = self.intercept {
                                    val = (val as f64 * slope + intercept) as u32;
                                }
                            }
                            val
                        };
                        in_pos += U32_SIZE;
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
                Ok(PixelDataBuffer::U32(PixelDataBufferU32::new(
                    self, buffer, min, max,
                )))
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
                pixdata_info.intercept = Some(val);
            }
        } else if elem.tag() == tags::RescaleSlope.tag() {
            if let Some(val) = elem.parse_value()?.double() {
                pixdata_info.slope = Some(val);
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
