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

//! The image command extracts pixel data and encodes it as a standard image format.

use anyhow::{anyhow, Result};
use dcmpipe_lib::core::{
    defn::ts::TSRef,
    pixeldata::{
        pdbuf::PixelDataBuffer, pdinfo::PixelDataInfo, pixel_i16::PixelI16, pixel_i32::PixelI32,
        pixel_i8::PixelI8, pixel_u16::PixelU16, pixel_u32::PixelU32, pixel_u8::PixelU8,
    },
};
use image::{ImageBuffer, Rgb};
use std::path::{Path, PathBuf};

use crate::{app::parse_file, args::ImageArgs, CommandApplication};

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

        let pixdata_info = PixelDataInfo::process_dcm_parser(parser)?;
        dbg!(&pixdata_info);

        let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
            ImageBuffer::new(pixdata_info.cols().into(), pixdata_info.rows().into());

        let pixdata_buffer = pixdata_info.load_pixel_data()?;
        dbg!(&pixdata_buffer);

        match pixdata_buffer {
            PixelDataBuffer::I8(pdbuf) => {
                for PixelU16 { x, y, r, g, b } in
                    pdbuf.pixel_iter().map(|PixelI8 { x, y, r, g, b }| {
                        let r = PixelDataBuffer::shift_i8(r);
                        let g = PixelDataBuffer::shift_i8(g);
                        let b = PixelDataBuffer::shift_i8(b);
                        PixelU16 {
                            x,
                            y,
                            r: ((r as f32 / u8::MAX as f32) * u16::MAX as f32) as u16,
                            g: ((g as f32 / u8::MAX as f32) * u16::MAX as f32) as u16,
                            b: ((b as f32 / u8::MAX as f32) * u16::MAX as f32) as u16,
                        }
                    })
                {
                    image.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                }
            }
            PixelDataBuffer::U8(pdbuf) => {
                for PixelU16 { x, y, r, g, b } in
                    pdbuf
                        .pixel_iter()
                        .map(|PixelU8 { x, y, r, g, b }| PixelU16 {
                            x,
                            y,
                            r: ((r as f32 / u8::MAX as f32) * u16::MAX as f32) as u16,
                            g: ((g as f32 / u8::MAX as f32) * u16::MAX as f32) as u16,
                            b: ((b as f32 / u8::MAX as f32) * u16::MAX as f32) as u16,
                        })
                {
                    image.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                }
            }
            PixelDataBuffer::I16(pdbuf) => {
                for PixelU16 { x, y, r, g, b } in
                    pdbuf.pixel_iter().map(|PixelI16 { x, y, r, g, b }| {
                        let r = PixelDataBuffer::shift_i16(r);
                        let g = PixelDataBuffer::shift_i16(g);
                        let b = PixelDataBuffer::shift_i16(b);
                        PixelU16 { x, y, r, g, b }
                    })
                {
                    image.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                }
            }
            PixelDataBuffer::U16(pdbuf) => {
                for PixelU16 { x, y, r, g, b } in pdbuf.pixel_iter() {
                    image.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                }
            }
            PixelDataBuffer::I32(pdbuf) => {
                for PixelU16 { x, y, r, g, b } in
                    pdbuf.pixel_iter().map(|PixelI32 { x, y, r, g, b }| {
                        let r = PixelDataBuffer::shift_i32(r);
                        let g = PixelDataBuffer::shift_i32(g);
                        let b = PixelDataBuffer::shift_i32(b);
                        PixelU16 {
                            x,
                            y,
                            r: ((r as f32 / u32::MAX as f32) * u16::MAX as f32) as u16,
                            g: ((g as f32 / u32::MAX as f32) * u16::MAX as f32) as u16,
                            b: ((b as f32 / u32::MAX as f32) * u16::MAX as f32) as u16,
                        }
                    })
                {
                    image.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                }
            }
            PixelDataBuffer::U32(pdbuf) => {
                for PixelU16 { x, y, r, g, b } in
                    pdbuf
                        .pixel_iter()
                        .map(|PixelU32 { x, y, r, g, b }| PixelU16 {
                            x,
                            y,
                            r: ((r as f32 / u32::MAX as f32) * u16::MAX as f32) as u16,
                            g: ((g as f32 / u32::MAX as f32) * u16::MAX as f32) as u16,
                            b: ((b as f32 / u32::MAX as f32) * u16::MAX as f32) as u16,
                        })
                {
                    image.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                }
            }
        }
        image.save(output_path_buf)?;

        Ok(())
    }
}
