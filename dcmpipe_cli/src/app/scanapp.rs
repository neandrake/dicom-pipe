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

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use walkdir::WalkDir;

use dcmpipe_lib::core::read::{Parser, ParserBuilder};
use dcmpipe_lib::dict::stdlookup::STANDARD_DICOM_DICTIONARY;

use crate::{app::CommandApplication, args::ScanArgs};

enum ScanResult {
    Success,
    NotDicom,
    InvalidData(Box<dyn std::error::Error>),
}

pub struct ScanApp {
    args: ScanArgs,
}

impl ScanApp {
    pub fn new(args: ScanArgs) -> ScanApp {
        ScanApp { args }
    }

    fn get_files(folder: &PathBuf) -> impl Iterator<Item = PathBuf> {
        WalkDir::new(folder)
            .into_iter()
            .map(|entry_res| entry_res.expect("walkdir entry").path().to_path_buf())
            .filter(|path: &PathBuf| path.is_file())
    }

    fn parse_all_element_values(parser: Parser<'_, BufReader<File>>) -> ScanResult {
        let mut is_first_elem: bool = true;
        for elem_result in parser {
            match elem_result {
                Ok(elem) => {
                    match elem.parse_value() {
                        Ok(_) => {}
                        Err(e) => return ScanResult::InvalidData(Box::new(e)),
                    };
                }
                Err(e) => {
                    if is_first_elem {
                        return ScanResult::NotDicom;
                    }
                    return ScanResult::InvalidData(Box::new(e));
                }
            };
            is_first_elem = false;
        }

        ScanResult::Success
    }
}

impl CommandApplication for ScanApp {
    fn run(&mut self) -> Result<()> {
        let parser_builder: ParserBuilder = ParserBuilder::default();

        for path in Self::get_files(&self.args.folder) {
            let file = File::open(path.clone())?;
            let dataset = BufReader::new(file);
            let parser = parser_builder.build(dataset, &STANDARD_DICOM_DICTIONARY);

            let relative_path: &str = path
                .strip_prefix(&self.args.folder)?
                .to_str()
                .expect("relative path");

            match Self::parse_all_element_values(parser) {
                ScanResult::Success | ScanResult::NotDicom => {}
                ScanResult::InvalidData(e) => println!("Failure Parsing: {relative_path}\n\t{e}"),
            };
        }

        Ok(())
    }
}
