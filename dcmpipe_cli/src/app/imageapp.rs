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
        pdbuf::{PixelDataBuffer, PixelU16, PixelU32, PixelU8},
        pdinfo::PixelDataInfo,
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

        let pixdata_buffer = PixelDataInfo::process_dcm_parser(parser)?.load_pixel_data()?;
        dbg!(&pixdata_buffer);

        match pixdata_buffer {
            PixelDataBuffer::U8(pdbuf) => {
                let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                    ImageBuffer::new(pdbuf.info().cols().into(), pdbuf.info().rows().into());
                for PixelU8 { x, y, r, g, b } in pdbuf.pixel_iter() {
                    image.put_pixel(x, y, Rgb([r, g, b]));
                }
                image.save(output_path_buf)?;
            }
            PixelDataBuffer::U16(pdbuf) => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pdbuf.info().cols().into(), pdbuf.info().rows().into());
                for PixelU16 { x, y, r, g, b } in pdbuf.pixel_iter() {
                    image.put_pixel(x, y, Rgb([r, g, b]));
                }
                image.save(output_path_buf)?;
            }
            PixelDataBuffer::U32(pdbuf) => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pdbuf.info().cols().into(), pdbuf.info().rows().into());
                for PixelU32 { x, y, r, g, b } in pdbuf.pixel_iter() {
                    image.put_pixel(x, y, Rgb([r as u16, g as u16, b as u16]));
                }
                image.save(output_path_buf)?;
            }
        }

        Ok(())
    }
}
