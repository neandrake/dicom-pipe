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
            dcmdict::DicomDictionary,
            ts::TSRef,
            vr::{self, VRRef},
        },
        read::Parser,
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{self},
        uids,
    },
};
use image::{ImageBuffer, Rgb};
use std::{
    fmt::Display,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::{app::parse_file, args::ImageArgs, CommandApplication};

#[derive(PartialEq, Eq, Debug)]
enum Modality {
    CR,
    CT,
    MR,
    NM,
    PT,
    SC,
    Unsupported(String),
}

impl Modality {
    fn from_uid(uid: &str) -> Modality {
        if uids::ComputedRadiographyImageStorage.uid() == uid {
            Modality::CR
        } else if uids::CTImageStorage.uid() == uid {
            Modality::CT
        } else if uids::MRImageStorage.uid() == uid {
            Modality::MR
        } else if uids::NuclearMedicineImageStorage.uid() == uid {
            Modality::NM
        } else if uids::PositronEmissionTomographyImageStorage.uid() == uid {
            Modality::PT
        } else if uids::SecondaryCaptureImageStorage.uid() == uid {
            Modality::SC
        } else {
            Modality::Unsupported(uid.to_owned())
        }
    }
}

impl Display for Modality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modality::CR => write!(f, "CR"),
            Modality::CT => write!(f, "CT"),
            Modality::MR => write!(f, "MR"),
            Modality::NM => write!(f, "NM"),
            Modality::PT => write!(f, "PT"),
            Modality::SC => write!(f, "SC"),
            Modality::Unsupported(uid) => write!(f, "{uid}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum PhotoInterp {
    Unsupported(String),
    Rgb,
    Monochrome1,
    Monochrome2,
}

impl PhotoInterp {
    fn is_rgb(&self) -> bool {
        *self == PhotoInterp::Rgb
    }

    fn is_monochrome(&self) -> bool {
        *self == PhotoInterp::Monochrome1 || *self == PhotoInterp::Monochrome2
    }
}

impl From<&str> for PhotoInterp {
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

#[derive(PartialEq, Eq, Debug)]
enum BitsAllocated {
    Unspecified(u16),
    Eight,
    Sixteen,
    ThirtyTwo,
}

impl Display for BitsAllocated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BitsAllocated::Unspecified(val) => write!(f, "Unspecified({val})"),
            BitsAllocated::Eight => write!(f, "Bits(8)"),
            BitsAllocated::Sixteen => write!(f, "Bits(16)"),
            BitsAllocated::ThirtyTwo => write!(f, "Bits(32)"),
        }
    }
}

impl BitsAllocated {
    fn from_val(val: u16) -> Self {
        match val {
            8 => BitsAllocated::Eight,
            16 => BitsAllocated::Sixteen,
            32 => BitsAllocated::ThirtyTwo,
            other => BitsAllocated::Unspecified(other),
        }
    }

    fn val(&self) -> u16 {
        match self {
            Self::Unspecified(val) => *val,
            Self::Eight => 8,
            Self::Sixteen => 16,
            Self::ThirtyTwo => 32,
        }
    }
}

struct PixelDataInfo {
    big_endian: bool,
    modality: Modality,
    vr: VRRef,
    samples_per_pixel: u16,
    photo_interp: Option<PhotoInterp>,
    planar_config: u16,
    cols: u16,
    rows: u16,
    pixel_padding_val: Option<u16>,
    bits_alloc: BitsAllocated,
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
            modality: Modality::Unsupported(String::new()),
            vr: &vr::OB,
            samples_per_pixel: 0,
            photo_interp: None,
            planar_config: 0,
            cols: 0,
            rows: 0,
            pixel_padding_val: None,
            bits_alloc: BitsAllocated::Unspecified(0),
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
            .field("modality", &self.modality)
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
    fn is_signed(&self) -> bool {
        self.pixel_rep != 0
    }

    /// Loads the pixel data bytes into a `ByteBuffer`. This will mutate self to swap ownership of
    /// self.pd_bytes so as not to duplicate the bytes in memory. Because of the ownership swap,
    /// calling this function again after the first time will return an empty `ByteBuffer`.
    fn bytes(&mut self) -> Result<ByteBuffer> {
        let bytes = std::mem::replace(&mut self.pd_bytes, Vec::with_capacity(0));
        let mut bb = ByteBuffer::from_vec(bytes);
        bb.set_endian(if self.big_endian {
            BigEndian
        } else {
            LittleEndian
        });
        Ok(bb)
    }

    /// After all relevant elements have been parsed, this will validate the result of this
    /// structure.
    fn validate(&mut self) -> Result<()> {
        if self.pd_bytes.is_empty() {
            return Err(anyhow!("Missing PixelData"));
        }

        if let Modality::Unsupported(ref uid) = self.modality {
            return Err(anyhow!(
                "Unsupported Modality: {uid} {}",
                STANDARD_DICOM_DICTIONARY
                    .get_uid_by_uid(uid)
                    .map(|uid| uid.name())
                    .unwrap_or("unknown")
            ));
        }

        if self.cols == 0 || self.rows == 0 {
            return Err(anyhow!("Invalid Columns/Rows: {}x{}", self.cols, self.rows));
        }

        if self.vr != &vr::OB && self.vr != &vr::OW {
            return Err(anyhow!("Unsupported PixelData VR"));
        };

        if let BitsAllocated::Unspecified(val) = self.bits_alloc {
            return Err(anyhow!("Invalid BitsAllocated: {val}"));
        }
        if self.bits_stored > self.bits_alloc.val() || self.bits_stored == 0 {
            self.bits_stored = self.bits_alloc.val();
        }
        if self.high_bit > self.bits_alloc.val() - 1 || self.high_bit < self.bits_stored - 1 {
            self.high_bit = self.bits_stored - 1;
        }

        if let Some(pi) = &self.photo_interp {
            if pi.is_rgb() && self.samples_per_pixel != 3 {
                return Err(anyhow!(
                    "RGB image does not have 3 samples-per-pixel: {}",
                    self.samples_per_pixel
                ));
            } else if pi.is_monochrome() && self.samples_per_pixel != 1 {
                return Err(anyhow!(
                    "MONOCHROME image does not have 1 samples-per-pixel: {}",
                    self.samples_per_pixel
                ));
            }
        }

        Ok(())
    }

    /// Loads the pixel data bytes into `PixelDataBuffer`.
    fn load_pixel_data(&mut self) -> Result<PixelDataBuffer> {
        let mut bytes = self.bytes()?;
        let len = Into::<usize>::into(self.cols) * Into::<usize>::into(self.rows);
        match self.bits_alloc {
            BitsAllocated::Unspecified(_) => {
                Err(anyhow!("Unsupported BitsAllocated: {}", self.bits_alloc))
            }
            BitsAllocated::Eight => {
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
            BitsAllocated::Sixteen => {
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
            BitsAllocated::ThirtyTwo => {
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

    fn process_dcm_parser(parser: Parser<'_, BufReader<File>>) -> Result<PixelDataInfo> {
        let mut pixdata_info: PixelDataInfo = PixelDataInfo {
            big_endian: parser.ts().big_endian(),
            ..Default::default()
        };
        for elem in parser {
            Self::process_element(&mut pixdata_info, elem?)?;
        }
        Ok(pixdata_info)
    }

    /// Process relevant DICOM elements into this structure.
    fn process_element(pixdata_info: &mut PixelDataInfo, mut elem: DicomElement) -> Result<()> {
        // The order of the tag checks here are the order they will appear in a DICOM protocol.
        if elem.tag() == tags::SOPClassUID.tag() {
            if let Some(val) = elem.parse_value()?.string() {
                pixdata_info.modality = Modality::from_uid(val);
            }
        } else if elem.tag() == tags::SamplesperPixel.tag() {
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
                pixdata_info.bits_alloc = BitsAllocated::from_val(val);
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
        } else if elem.is_pixel_data() {
            // Transfer ownership of the PixelData bytes to a local to copy into pixdata_info.pd_bytes.
            // If PixelData is fragmented then this slice will be empty.
            let data = std::mem::replace(elem.mut_data(), Vec::with_capacity(0));
            pixdata_info.pd_bytes.extend_from_slice(&data);
        } else if elem.is_within_pixel_data() {
            // Transfer ownership of the fragment's bytes to a local to copy into pixdata_info.pd_bytes.
            let data = std::mem::replace(elem.mut_data(), Vec::with_capacity(0));
            pixdata_info.pd_bytes.extend_from_slice(&data);
        }

        Ok(())
    }
}

#[allow(dead_code)]
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
    /// Shift an i8 value into u8 space.
    fn shift_i8(val: i8) -> u8 {
        ((val as i16) + (i8::MAX as i16)) as u8
    }

    /// Shift an i16 value into u16 space.
    fn shift_i16(val: i16) -> u16 {
        ((val as i32) + (i16::MAX as i32)) as u16
    }

    /// Shift an i32 value into u32 space.
    fn shift_i32(val: i32) -> u32 {
        ((val as i64) + (i32::MAX as i64)) as u32
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
