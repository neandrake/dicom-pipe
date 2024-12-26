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

//! The image command extracts pixel data and encodes it as a standard iamge format.

use anyhow::{anyhow, Result};
use bytebuffer::{ByteBuffer, Endian::BigEndian, Endian::LittleEndian};
use dcmpipe_lib::{
    core::{
        dcmelement::DicomElement,
        defn::{
            ts::TSRef,
            vr::{self, VRRef},
        },
        read::Parser,
        RawValue,
    },
    dict::tags::{self},
};
use image::{ImageBuffer, Rgb};
use std::{
    io::Read,
    path::{Path, PathBuf},
};

use crate::{app::parse_file, args::ImageArgs, CommandApplication};

/// Supported values of Photometric Interpretation.
#[derive(PartialEq, Eq, Debug)]
enum PhotoInterp {
    Unsupported(String),
    Rgb,
    Monochrome1,
    Monochrome2,
}

impl PhotoInterp {
    /// Whether this `PhotoInterp` is `RGB`.
    #[must_use]
    fn is_rgb(&self) -> bool {
        *self == PhotoInterp::Rgb
    }

    /// Whether this `PhotoInterp` is one of the supported monochrome values.
    #[must_use]
    fn is_monochrome(&self) -> bool {
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
enum BitsAlloc {
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
    fn from_val(val: u16) -> Self {
        match val {
            8 => BitsAlloc::Eight,
            16 => BitsAlloc::Sixteen,
            32 => BitsAlloc::ThirtyTwo,
            other => BitsAlloc::Unsupported(other),
        }
    }

    #[must_use]
    fn val(&self) -> u16 {
        match self {
            Self::Unsupported(val) => *val,
            Self::Eight => 8,
            Self::Sixteen => 16,
            Self::ThirtyTwo => 32,
        }
    }
}

/// Parsed tag values relevant to interpreting Pixel Data.
struct PixelDataInfo {
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
    /// Whether the byte values in Pixel Data are signed or unsigned values.
    #[must_use]
    fn is_signed(&self) -> bool {
        self.pixel_rep != 0
    }

    /// Loads the pixel data bytes into a `ByteBuffer`. This will mutate self to swap ownership of
    /// self.pd_bytes so as not to duplicate the bytes in memory. Because of the ownership swap,
    /// calling this function again after the first time will return an empty `ByteBuffer`.
    fn bytes(&mut self) -> ByteBuffer {
        let bytes = std::mem::replace(&mut self.pd_bytes, Vec::with_capacity(0));
        let mut bb = ByteBuffer::from_vec(bytes);
        bb.set_endian(if self.big_endian {
            BigEndian
        } else {
            LittleEndian
        });
        bb
    }

    /// After all relevant elements have been parsed, this will validate the result of this
    /// structure.
    ///
    /// # Errors
    /// - This function returns errors in the validation of values parsed from DICOM elements via
    ///   `PixelDataInfo::process_dcm_parser`.
    fn validate(&mut self) -> Result<()> {
        if self.pd_bytes.is_empty() {
            return Err(anyhow!("Missing PixelData"));
        }

        if self.cols == 0 || self.rows == 0 {
            return Err(anyhow!("Invalid Columns/Rows: {}x{}", self.cols, self.rows));
        }

        if self.vr != &vr::OB && self.vr != &vr::OW {
            return Err(anyhow!("Unsupported PixelData VR: {:?}", self.vr));
        };

        if let BitsAlloc::Unsupported(val) = self.bits_alloc {
            return Err(anyhow!("Unsupported BitsAllocated: {val}"));
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
            if pi.is_rgb() && self.samples_per_pixel != 3 {
                // RGB must use 3 Samples Per Pixel.
                return Err(anyhow!(
                    "RGB image does not have 3 samples-per-pixel: {}",
                    self.samples_per_pixel
                ));
            } else if pi.is_monochrome() && self.samples_per_pixel != 1 {
                // MONOCHROME1/2 must use 1 Sample Per Pixel.
                return Err(anyhow!(
                    "MONOCHROME image does not have 1 samples-per-pixel: {}",
                    self.samples_per_pixel
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
    fn load_pixel_data(&mut self) -> Result<PixelDataBuffer> {
        let mut bytes = self.bytes();
        let len = Into::<usize>::into(self.cols) * Into::<usize>::into(self.rows);
        match self.bits_alloc {
            BitsAlloc::Unsupported(_) => {
                Err(anyhow!("Unsupported BitsAllocated: {}", self.bits_alloc))
            }
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
                            PixelDataBuffer::shift_i8(bytes.read_i8()?)
                        } else {
                            bytes.read_u8()?
                        };
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
                Ok(PixelDataBuffer::U8 { buffer, min, max })
            }
            BitsAlloc::Sixteen => {
                let mut buffer: Vec<u16> =
                    Vec::with_capacity(len * self.samples_per_pixel as usize);
                let mut min: u16 = u16::MAX;
                let mut max: u16 = u16::MIN;
                for _i in 0..len {
                    for _j in 0..self.samples_per_pixel {
                        let val = if self.is_signed() {
                            PixelDataBuffer::shift_i16(bytes.read_i16()?)
                        } else {
                            bytes.read_u16()?
                        };
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
                Ok(PixelDataBuffer::U16 { buffer, min, max })
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
                            PixelDataBuffer::shift_i32(bytes.read_i32()?)
                        } else {
                            bytes.read_u32()?
                        };
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
                Ok(PixelDataBuffer::U32 { buffer, min, max })
            }
        }
    }

    /// Processes a DICOM SOP via a `Parser` into a `PixelDataInfo`.
    ///
    /// # Errors
    /// - I/O errors parsing values out of DICOM elements.
    fn process_dcm_parser<R: Read>(parser: Parser<'_, R>) -> Result<PixelDataInfo> {
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
    fn process_element(pixdata_info: &mut PixelDataInfo, elem: &DicomElement) -> Result<()> {
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

/// Container for the raw pixel values parsed from the DICOM binary data.
enum PixelDataBuffer {
    U8 {
        buffer: Vec<u8>,
        min: u8,
        max: u8,
    },
    U16 {
        buffer: Vec<u16>,
        min: u16,
        max: u16,
    },
    U32 {
        buffer: Vec<u32>,
        min: u32,
        max: u32,
    },
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

impl std::fmt::Debug for PixelDataBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Default Debug implementation but don't print all bytes, just the length.
        match self {
            Self::U8 { buffer, min, max } => f
                .debug_struct("U8")
                .field("buffer_size", &(buffer.len()))
                .field("min", min)
                .field("max", max)
                .finish(),
            Self::U16 { buffer, min, max } => f
                .debug_struct("U16")
                .field("buffer_size", &(buffer.len()))
                .field("min", min)
                .field("max", max)
                .finish(),
            Self::U32 { buffer, min, max } => f
                .debug_struct("U32")
                .field("buffer_size", &(buffer.len()))
                .field("min", min)
                .field("max", max)
                .finish(),
        }
    }
}

pub struct ImageApp {
    args: ImageArgs,
}

impl ImageApp {
    pub fn new(args: ImageArgs) -> ImageApp {
        ImageApp { args }
    }

    fn is_jpeg(ts: TSRef) -> bool {
        ts.uid().name().contains("JPEG")
    }
}

impl CommandApplication for ImageApp {
    fn run(&mut self) -> Result<()> {
        let path_buf: PathBuf = self.args.file.clone();
        let output_path_buf = self.args.output.clone();
        let path: &Path = path_buf.as_path();
        let parser = parse_file(path, true)?;

        if ImageApp::is_jpeg(parser.ts()) {
            return Err(anyhow!(
                "Unsupported TransferSyntax: {}",
                parser.ts().uid().name()
            ));
        }

        let mut pixdata_info: PixelDataInfo = PixelDataInfo::process_dcm_parser(parser)?;
        dbg!(&pixdata_info);
        pixdata_info.validate()?;

        let pixdata_buffer = pixdata_info.load_pixel_data()?;
        dbg!(&pixdata_buffer);

        match pixdata_buffer {
            PixelDataBuffer::U8 { buffer, min, max } => {
                let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                    ImageBuffer::new(pixdata_info.cols.into(), pixdata_info.rows.into());
                let range = max - min;
                let stride = if pixdata_info.planar_config == 0 {
                    1
                } else {
                    buffer.len() / pixdata_info.samples_per_pixel as usize
                };
                let mut src_byte_index = 0;
                let mut dst_pixel_index: usize = 0;
                while src_byte_index < buffer.len() {
                    let x = (dst_pixel_index as u32) % (pixdata_info.cols as u32);
                    let y = (dst_pixel_index as u32) / (pixdata_info.cols as u32);
                    if pixdata_info
                        .photo_interp
                        .as_ref()
                        .is_some_and(PhotoInterp::is_rgb)
                        && pixdata_info.samples_per_pixel == 3
                    {
                        let r = buffer[src_byte_index];
                        let g = buffer[src_byte_index + stride];
                        let b = buffer[src_byte_index + stride * 2];
                        image.put_pixel(x, y, Rgb([r, g, b]));
                        if pixdata_info.planar_config == 0 {
                            src_byte_index += pixdata_info.samples_per_pixel as usize;
                        } else {
                            src_byte_index += 1;
                        }
                    } else if pixdata_info
                        .photo_interp
                        .as_ref()
                        .is_some_and(PhotoInterp::is_monochrome)
                        && pixdata_info.samples_per_pixel == 1
                    {
                        let val = buffer[src_byte_index];
                        let val = ((val as f32 / range as f32) * u8::MAX as f32) as u8;
                        image.put_pixel(x, y, Rgb([val, val, val]));
                        src_byte_index += 1;
                    }
                    dst_pixel_index += 1;
                }
                image.save(output_path_buf)?;
            }
            PixelDataBuffer::U16 { buffer, min, max } => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pixdata_info.cols.into(), pixdata_info.rows.into());
                let range = max - min;
                let stride = if pixdata_info.planar_config == 0 {
                    1
                } else {
                    buffer.len() / pixdata_info.samples_per_pixel as usize
                };
                let mut src_byte_index = 0;
                let mut dst_pixel_index = 0;
                while src_byte_index < buffer.len() {
                    let x = (dst_pixel_index as u32) % (pixdata_info.cols as u32);
                    let y = (dst_pixel_index as u32) / (pixdata_info.cols as u32);
                    if pixdata_info
                        .photo_interp
                        .as_ref()
                        .is_some_and(PhotoInterp::is_rgb)
                        && pixdata_info.samples_per_pixel == 3
                        && pixdata_info.planar_config == 0
                    {
                        let r = buffer[src_byte_index];
                        let g = buffer[src_byte_index + stride];
                        let b = buffer[src_byte_index + stride * 2];
                        image.put_pixel(x, y, Rgb([r, g, b]));
                        if pixdata_info.planar_config == 0 {
                            src_byte_index += pixdata_info.samples_per_pixel as usize;
                        } else {
                            src_byte_index += 1;
                        }
                    } else if pixdata_info
                        .photo_interp
                        .as_ref()
                        .is_some_and(PhotoInterp::is_monochrome)
                        && pixdata_info.samples_per_pixel == 1
                    {
                        let val = buffer[src_byte_index];
                        let val = ((val as f32 / range as f32) * u16::MAX as f32) as u16;
                        image.put_pixel(x, y, Rgb([val, val, val]));
                        src_byte_index += 1;
                    }
                    dst_pixel_index += 1;
                }
                image.save(output_path_buf)?;
            }
            PixelDataBuffer::U32 { buffer, min, max } => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pixdata_info.cols.into(), pixdata_info.rows.into());
                let range = max - min;
                let stride = if pixdata_info.planar_config == 0 {
                    1
                } else {
                    buffer.len() / pixdata_info.samples_per_pixel as usize
                };
                let mut src_byte_index = 0;
                let mut dst_pixel_index = 0;
                while src_byte_index < buffer.len() {
                    let x = (dst_pixel_index as u32) % (pixdata_info.cols as u32);
                    let y = (dst_pixel_index as u32) / (pixdata_info.cols as u32);
                    if pixdata_info
                        .photo_interp
                        .as_ref()
                        .is_some_and(PhotoInterp::is_rgb)
                        && pixdata_info.samples_per_pixel == 3
                        && pixdata_info.planar_config == 0
                    {
                        let r = buffer[src_byte_index] as u16;
                        let g = buffer[src_byte_index + stride] as u16;
                        let b = buffer[src_byte_index + stride * 2] as u16;
                        image.put_pixel(x, y, Rgb([r, g, b]));
                        if pixdata_info.planar_config == 0 {
                            src_byte_index += pixdata_info.samples_per_pixel as usize;
                        } else {
                            src_byte_index += 1;
                        }
                    } else if pixdata_info
                        .photo_interp
                        .as_ref()
                        .is_some_and(PhotoInterp::is_monochrome)
                        && pixdata_info.samples_per_pixel == 1
                    {
                        let val = buffer[src_byte_index];
                        let val = ((val as f32 / range as f32) * u32::MAX as f32) as u16;
                        image.put_pixel(x, y, Rgb([val, val, val]));
                        src_byte_index += 1;
                    }
                    dst_pixel_index += 1;
                }
                image.save(output_path_buf)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::imageapp::PixelDataBuffer;

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
