use std::{fs::File, iter::Peekable, path::Path};

use anyhow::{anyhow, Result};
use dcmpipe_lib::{
    core::{
        dcmelement::DicomElement,
        read::{ParseError, Parser, ParserBuilder},
    },
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
};

pub(crate) mod archiveapp;
pub(crate) mod browseapp;
#[cfg(feature = "index")]
pub(crate) mod indexapp;
pub(crate) mod printapp;
pub(crate) mod scanapp;
pub(crate) mod scpapp;

pub(crate) trait CommandApplication {
    fn run(&mut self) -> Result<()>;
}

fn parse_file(path: &Path, allow_partial_object: bool) -> Result<Parser<'_, File>> {
    if !path.is_file() {
        return Err(anyhow!("invalid file: {}", path.display()));
    }

    let file: File = File::open(path)?;
    let mut parser: Parser<'_, File> = ParserBuilder::default()
        .allow_partial_object(allow_partial_object)
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
