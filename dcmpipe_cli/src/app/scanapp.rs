use std::fs::File;
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

    fn get_files(&self) -> impl Iterator<Item = PathBuf> {
        WalkDir::new(&self.args.folder)
            .into_iter()
            .map(|entry_res| entry_res.expect("walkdir entry").path().to_path_buf())
            .filter(|path: &PathBuf| path.is_file())
    }

    fn parse_all_element_values(&self, parser: Parser<'_, File>) -> ScanResult {
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

        for path in self.get_files() {
            let file: File = File::open(path.clone())?;
            let parser: Parser<'_, File> = parser_builder.build(file, &STANDARD_DICOM_DICTIONARY);

            let relative_path: &str = path
                .strip_prefix(&self.args.folder)?
                .to_str()
                .expect("relative path");

            match self.parse_all_element_values(parser) {
                ScanResult::Success => {}  /*println!("Valid DICOM: {}", path_str),*/
                ScanResult::NotDicom => {} /*println!("Not DICOM: {}", relative_path),*/
                ScanResult::InvalidData(e) => {
                    println!("Failure Parsing: {}\n\t{}", relative_path, e)
                }
            };
        }

        Ok(())
    }
}
