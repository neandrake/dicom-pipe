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
        defn::{dcmdict::DicomDictionary, ts::TSRef, vr},
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
    path::{Path, PathBuf},
};

use crate::{app::parse_file, args::ImageArgs, CommandApplication};

#[derive(PartialEq, Eq, Debug)]
enum Modality {
    CT,
    MR,
    NM,
    PT,
    Unsupported(String),
}

impl Modality {
    fn from_uid(uid: &str) -> Modality {
        if uids::CTImageStorage.uid() == uid {
            Modality::CT
        } else if uids::MRImageStorage.uid() == uid {
            Modality::MR
        } else if uids::NuclearMedicineImageStorage.uid() == uid {
            Modality::NM
        } else if uids::PositronEmissionTomographyImageStorage.uid() == uid {
            Modality::PT
        } else {
            Modality::Unsupported(uid.to_owned())
        }
    }
}

impl Display for Modality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modality::CT => write!(f, "CT"),
            Modality::MR => write!(f, "MR"),
            Modality::NM => write!(f, "NM"),
            Modality::PT => write!(f, "PT"),
            Modality::Unsupported(modality) => write!(f, "{modality}"),
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

struct DcmImageInfo {
    modality: Modality,
    bytes_in_word: u8,
    signed: bool,
    rows: u16,
    cols: u16,
    pixel_padding_val: Option<u16>,
    bits_allocated: BitsAllocated,
    bits_stored: u16,
    high_bit: u16,
    slope: f64,
    intercept: f64,
    unit: String,
    window_centers: Vec<f32>,
    window_widths: Vec<f32>,
    window_labels: Vec<String>,
}

impl Default for DcmImageInfo {
    fn default() -> Self {
        Self {
            modality: Modality::Unsupported(String::new()),
            bytes_in_word: 0,
            signed: true,
            rows: 0,
            cols: 0,
            pixel_padding_val: None,
            bits_allocated: BitsAllocated::Unspecified(0),
            bits_stored: 0,
            high_bit: 0,
            slope: 0.0,
            intercept: 0.0,
            unit: String::new(),
            window_centers: Vec::new(),
            window_widths: Vec::new(),
            window_labels: Vec::new(),
        }
    }
}

impl DcmImageInfo {
    /// Parses relevant DICOM element values into this structure.
    fn parse_elem(&mut self, elem: &DicomElement) -> Result<()> {
        // The order of the tag checks here are the order they will appear in a DICOM protocol.
        if elem.tag() == tags::SOPClassUID.tag() {
            if let Some(val) = elem.parse_value()?.string() {
                self.modality = Modality::from_uid(val);
            }
        } else if elem.tag() == tags::Rows.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                self.rows = val;
            }
        } else if elem.tag() == tags::Columns.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                self.cols = val;
            }
        } else if elem.tag() == tags::BitsAllocated.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                self.bits_allocated = BitsAllocated::from_val(val);
            }
        } else if elem.tag() == tags::BitsStored.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                self.bits_stored = val;
            }
        } else if elem.tag() == tags::HighBit.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                self.high_bit = val;
            }
        } else if elem.tag() == tags::PixelRepresentation.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                self.signed = val != 0;
            }
        } else if elem.tag() == tags::PixelPaddingValue.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                self.pixel_padding_val = Some(val);
            }
        } else if elem.tag() == tags::WindowCenter.tag() {
            if let RawValue::Floats(vals) = elem.parse_value()? {
                self.window_centers = vals;
            }
        } else if elem.tag() == tags::WindowWidth.tag() {
            if let RawValue::Floats(vals) = elem.parse_value()? {
                self.window_widths = vals;
            }
        } else if elem.tag() == tags::RescaleIntercept.tag() {
            if let Some(val) = elem.parse_value()?.double() {
                self.intercept = val;
            }
        } else if elem.tag() == tags::RescaleSlope.tag() {
            if let Some(val) = elem.parse_value()?.double() {
                self.slope = val;
            }
        } else if elem.tag() == tags::RescaleType.tag() || elem.tag() == tags::Units.tag() {
            if let Some(val) = elem.parse_value()?.string() {
                // Only use Units if RescaleType wasn't present.
                if self.unit.is_empty() {
                    self.unit = val.to_owned();
                }
            }
        } else if elem.tag() == tags::WindowCenter_and_WidthExplanation.tag() {
            if let RawValue::Strings(vals) = elem.parse_value()? {
                self.window_labels = vals;
            }
        }

        Ok(())
    }

    /// After all relevant elements have been parsed, this will validate the result of this
    /// structure.
    fn validate(&mut self, pixel_data: &DicomElement) -> Result<()> {
        if let Modality::Unsupported(ref uid) = self.modality {
            return Err(anyhow!(
                "Unsupported Modality: {uid} {}",
                STANDARD_DICOM_DICTIONARY
                    .get_uid_by_uid(uid)
                    .map(|uid| uid.name())
                    .unwrap_or("unknown")
            ));
        }

        if self.rows == 0 || self.cols == 0 {
            return Err(anyhow!("Invalid Rows/Columns: {}x{}", self.rows, self.cols));
        }

        self.bytes_in_word = if pixel_data.vr() == &vr::OW {
            2
        } else if pixel_data.vr() == &vr::OB {
            1
        } else {
            return Err(anyhow!("Unsupported VR"));
        };

        if let BitsAllocated::Unspecified(val) = self.bits_allocated {
            return Err(anyhow!("Invalid BitsAllocated: {val}"));
        }
        if self.bits_stored > self.bits_allocated.val() || self.bits_stored == 0 {
            self.bits_stored = self.bits_allocated.val();
        }
        if self.high_bit > self.bits_allocated.val() - 1 || self.high_bit < self.bits_stored - 1 {
            self.high_bit = self.bits_stored - 1;
        }

        Ok(())
    }
}

#[allow(dead_code)]
enum DcmImageBuffer {
    I8 {
        buffer: Vec<i8>,
        min: i8,
        max: i8,
    },
    U8 {
        buffer: Vec<u8>,
        min: u8,
        max: u8,
    },
    I16 {
        buffer: Vec<i16>,
        min: i16,
        max: i16,
    },
    U16 {
        buffer: Vec<u16>,
        min: u16,
        max: u16,
    },
    I32 {
        buffer: Vec<i32>,
        min: i32,
        max: i32,
    },
    U32 {
        buffer: Vec<u32>,
        min: u32,
        max: u32,
    },
}

impl std::fmt::Debug for DcmImageBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8 { buffer, min, max } => f
                .debug_struct("I8")
                .field("buffer_size", &(buffer.len()))
                .field("min", min)
                .field("max", max)
                .finish(),
            Self::U8 { buffer, min, max } => f
                .debug_struct("U8")
                .field("buffer_size", &(buffer.len()))
                .field("min", min)
                .field("max", max)
                .finish(),
            Self::I16 { buffer, min, max } => f
                .debug_struct("I16")
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
            Self::I32 { buffer, min, max } => f
                .debug_struct("I32")
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

impl DcmImageBuffer {
    fn parse(image_data: &DcmImageInfo, bytes: &mut ByteBuffer) -> Result<Self> {
        match image_data.bits_allocated {
            BitsAllocated::Unspecified(_) => Err(anyhow!(
                "Unsupported BitsAllocated: {}",
                image_data.bits_allocated
            )),
            BitsAllocated::Eight => {
                if image_data.signed {
                    let mut buffer: Vec<i8> = Vec::with_capacity(bytes.len());
                    let mut min: i8 = i8::MAX;
                    let mut max: i8 = i8::MIN;
                    let pixel_pad_val = image_data
                        .pixel_padding_val
                        .and_then(|pad_val| TryInto::<i8>::try_into(pad_val).ok());
                    for _i in 0..buffer.capacity() {
                        let val = bytes.read_i8()?;
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
                    if bytes.endian() == BigEndian && image_data.bytes_in_word == 2 {
                        let mut i = 0;
                        while i < buffer.len() - 1 {
                            buffer.swap(i, i + 1);
                            i += 2;
                        }
                    }
                    Ok(DcmImageBuffer::I8 { buffer, min, max })
                } else {
                    let mut buffer: Vec<u8> = Vec::with_capacity(bytes.len());
                    let mut min: u8 = u8::MAX;
                    let mut max: u8 = u8::MIN;
                    let pixel_pad_val = image_data
                        .pixel_padding_val
                        .and_then(|pad_val| TryInto::<u8>::try_into(pad_val).ok());
                    for _i in 0..buffer.capacity() {
                        let val = bytes.read_u8()?;
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
                    if bytes.endian() == BigEndian && image_data.bytes_in_word == 2 {
                        let mut i = 0;
                        while i < buffer.len() - 1 {
                            buffer.swap(i, i + 1);
                            i += 2;
                        }
                    }
                    Ok(DcmImageBuffer::U8 { buffer, min, max })
                }
            }
            BitsAllocated::Sixteen => {
                if image_data.signed {
                    let mut buffer: Vec<i16> = Vec::with_capacity(bytes.len() / 2);
                    let mut min: i16 = i16::MAX;
                    let mut max: i16 = i16::MIN;
                    let pixel_pad_val = image_data
                        .pixel_padding_val
                        .and_then(|pad_val| TryInto::<i16>::try_into(pad_val).ok());
                    for _i in 0..buffer.capacity() {
                        let val = bytes.read_i16()?;
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
                    Ok(DcmImageBuffer::I16 { buffer, min, max })
                } else {
                    let mut buffer: Vec<u16> = Vec::with_capacity(bytes.len() / 2);
                    let mut min: u16 = u16::MAX;
                    let mut max: u16 = u16::MIN;
                    for _i in 0..buffer.capacity() {
                        let val = bytes.read_u16()?;
                        buffer.push(val);
                        if image_data
                            .pixel_padding_val
                            .is_none_or(|pad_val| val != pad_val)
                        {
                            if val < min {
                                min = val;
                            }
                            if val > max {
                                max = val;
                            }
                        }
                    }
                    Ok(DcmImageBuffer::U16 { buffer, min, max })
                }
            }
            BitsAllocated::ThirtyTwo => {
                if image_data.signed {
                    let mut buffer: Vec<i32> = Vec::with_capacity(bytes.len() / 4);
                    let mut min: i32 = i32::MAX;
                    let mut max: i32 = i32::MIN;
                    let pixel_pad_val = image_data.pixel_padding_val.map(Into::<i32>::into);
                    for _i in 0..buffer.capacity() {
                        let val = bytes.read_i32()?;
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
                    Ok(DcmImageBuffer::I32 { buffer, min, max })
                } else {
                    let mut buffer: Vec<u32> = Vec::with_capacity(bytes.len() / 4);
                    let mut min: u32 = u32::MAX;
                    let mut max: u32 = u32::MIN;
                    let pixel_pad_val = image_data.pixel_padding_val.map(Into::<u32>::into);
                    for _i in 0..buffer.capacity() {
                        let val = bytes.read_u32()?;
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
                    Ok(DcmImageBuffer::U32 { buffer, min, max })
                }
            }
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

        let big_endian = parser.ts().big_endian();

        let mut image_info: DcmImageInfo = DcmImageInfo::default();
        let mut pixel_data: Option<DicomElement> = None;
        let mut in_pixel_data: bool = false;
        let mut fragmented_data: Vec<u8> = Vec::with_capacity(0);

        for elem in parser {
            let elem: DicomElement = elem?;

            if elem.is_pixel_data() {
                pixel_data = Some(elem);
                in_pixel_data = true;
            } else if in_pixel_data {
                fragmented_data.extend_from_slice(elem.data());
            } else {
                image_info.parse_elem(&elem)?;
            }
        }

        let Some(pixel_data) = pixel_data else {
            return Err(anyhow!("Missing PixelData"));
        };

        image_info.validate(&pixel_data)?;

        let mut bytes = if !fragmented_data.is_empty() {
            ByteBuffer::from_vec(fragmented_data)
        } else {
            ByteBuffer::from_bytes(pixel_data.data())
        };
        bytes.set_endian(if big_endian { BigEndian } else { LittleEndian });

        let dcm_image_buffer = DcmImageBuffer::parse(&image_info, &mut bytes)?;

        match dcm_image_buffer {
            DcmImageBuffer::I8 {
                buffer,
                min,
                max,
            } => {
                let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                    ImageBuffer::new(image_info.rows.into(), image_info.cols.into());
                let range = max - min;
                for (i, val) in buffer.iter().enumerate() {
                    let x = (i as u32) % (image_info.rows as u32);
                    let y = (i as u32) / (image_info.rows as u32);
                    let val = ((*val as f32 / range as f32) * u8::MAX as f32) as u8;
                    image.put_pixel(x, y, Rgb([val, val, val]));
                }
                image.save(output_path_buf)?;
            }
            DcmImageBuffer::U8 {
                buffer,
                min,
                max,
            } => {
                let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                    ImageBuffer::new(image_info.rows.into(), image_info.cols.into());
                let range = max - min;
                for (i, val) in buffer.iter().enumerate() {
                    let x = (i as u32) % (image_info.rows as u32);
                    let y = (i as u32) / (image_info.rows as u32);
                    let val = ((*val as f32 / range as f32) * u8::MAX as f32) as u8;
                    image.put_pixel(x, y, Rgb([val, val, val]));
                }
                image.save(output_path_buf)?;
            }
            DcmImageBuffer::I16 {
                buffer,
                min,
                max,
            } => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(image_info.rows.into(), image_info.cols.into());
                let range = max - min;
                for (i, val) in buffer.iter().enumerate() {
                    let x = (i as u32) % (image_info.rows as u32);
                    let y = (i as u32) / (image_info.rows as u32);
                    let val = ((*val as f32 / range as f32) * u16::MAX as f32) as u16;
                    image.put_pixel(x, y, Rgb([val, val, val]));
                }
                image.save(output_path_buf)?;
            }
            DcmImageBuffer::U16 {
                buffer,
                min,
                max,
            } => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(image_info.rows.into(), image_info.cols.into());
                let range = max - min;
                for (i, val) in buffer.iter().enumerate() {
                    let x = (i as u32) % (image_info.rows as u32);
                    let y = (i as u32) / (image_info.rows as u32);
                    let val = ((*val as f32 / range as f32) * u16::MAX as f32) as u16;
                    image.put_pixel(x, y, Rgb([val, val, val]));
                }
                image.save(output_path_buf)?;
            }
            DcmImageBuffer::I32 {
                buffer,
                min,
                max,
            } => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(image_info.rows.into(), image_info.cols.into());
                let range = max - min;
                for (i, val) in buffer.iter().enumerate() {
                    let x = (i as u32) % (image_info.rows as u32);
                    let y = (i as u32) / (image_info.rows as u32);
                    let val = ((*val as f32 / range as f32) * u16::MAX as f32) as u16;
                    image.put_pixel(x, y, Rgb([val, val, val]));
                }
                image.save(output_path_buf)?;
            }
            DcmImageBuffer::U32 {
                buffer,
                min,
                max,
            } => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(image_info.rows.into(), image_info.cols.into());
                let range = max - min;
                for (i, val) in buffer.iter().enumerate() {
                    let x = (i as u32) % (image_info.rows as u32);
                    let y = (i as u32) / (image_info.rows as u32);
                    let val = ((*val as f32 / range as f32) * u16::MAX as f32) as u16;
                    image.put_pixel(x, y, Rgb([val, val, val]));
                }
                image.save(output_path_buf)?;
            }
        }

        Ok(())
    }
}
