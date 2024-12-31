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
        pdinfo::PixelDataSliceInfo, pdslice::PixelDataSlice, pixel_i16::PixelI16,
        pixel_u16::PixelU16, pixel_u8::PixelU8,
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

        let pixdata_info = PixelDataSliceInfo::process_dcm_parser(parser)?;
        let pixdata_buffer = pixdata_info.load_pixel_data()?;
        dbg!(&pixdata_buffer);

        match pixdata_buffer {
            PixelDataSlice::U8(pdslice) => {
                let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                    ImageBuffer::new(pdslice.info().cols().into(), pdslice.info().rows().into());
                for PixelU8 { x, y, r, g, b } in pdslice.pixel_iter() {
                    image.put_pixel(u32::try_from(x)?, u32::try_from(y)?, Rgb([r, g, b]));
                }
                image.save(output_path_buf)?;
            }
            PixelDataSlice::U16(pdslice) => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pdslice.info().cols().into(), pdslice.info().rows().into());
                for PixelU16 { x, y, r, g, b } in pdslice.pixel_iter() {
                    image.put_pixel(u32::try_from(x)?, u32::try_from(y)?, Rgb([r, g, b]));
                }
                image.save(output_path_buf)?;
            }
            PixelDataSlice::I16(pdslice) => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pdslice.info().cols().into(), pdslice.info().rows().into());
                for PixelU16 { x, y, r, g, b } in
                    // The "image" crate does not support i16 pixel values.
                    pdslice.pixel_iter().map(|PixelI16 { x, y, r, g, b }| {
                            let r = PixelDataSlice::shift_i16(r);
                            let g = PixelDataSlice::shift_i16(g);
                            let b = PixelDataSlice::shift_i16(b);
                            PixelU16 { x, y, r, g, b }
                        })
                {
                    image.put_pixel(u32::try_from(x)?, u32::try_from(y)?, Rgb([r, g, b]));
                }
                image.save(output_path_buf)?;
            }
            other => {
                return Err(anyhow!("Unsupported PixelData: {other:?}"));
            }
        }

        Ok(())
    }
}
