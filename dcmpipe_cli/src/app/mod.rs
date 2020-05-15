use anyhow::{anyhow, Result};

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmparser::{ParseError, Parser, ParserBuilder};

use std::fs::File;
use std::iter::Peekable;
use std::path::Path;

pub(crate) mod archiveapp;
pub(crate) mod editapp;
pub(crate) mod indexapp;
pub(crate) mod printapp;
pub(crate) mod scanapp;

pub(crate) trait CommandApplication {
    fn run(&mut self) -> Result<()>;
}

fn parse_file(path: &Path) -> Result<Parser<'_, File>> {
    if !path.is_file() {
        return Err(anyhow!("invalid file: {}", path.display()));
    }

    let file: File = File::open(path)?;
    let mut parser: Parser<'_, File> = ParserBuilder::default()
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

    let mut peeker: Peekable<&mut Parser<'_, File>> = parser.by_ref().peekable();

    let first: Option<&Result<DicomElement, ParseError>> = peeker.peek();
    if let Some(Err(_)) = first {
        return Err(anyhow!("file is not dicom: {}", path.display()));
    } else if first.is_none() {
        return Err(anyhow!("file is empty: {}", path.display()));
    }

    Ok(parser)
}
