use std::{fs::File, io::BufReader, path::Path};

use anyhow::{anyhow, Result};
use dcmpipe_lib::{
    core::read::{Parser, ParserBuilder},
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

fn parse_file(path: &Path, allow_partial_object: bool) -> Result<Parser<'_, BufReader<File>>> {
    if !path.is_file() {
        return Err(anyhow!("invalid file: {}", path.display()));
    }

    let file = File::open(path)?;
    let dataset = BufReader::new(file);
    let mut parser = ParserBuilder::default()
        .allow_partial_object(allow_partial_object)
        .build(dataset, &STANDARD_DICOM_DICTIONARY);

    let mut peeker = parser.by_ref().peekable();

    let first = peeker.peek();
    if let Some(Err(_)) = first {
        return Err(anyhow!("file is not dicom: {}", path.display()));
    } else if first.is_none() {
        return Err(anyhow!("file is empty: {}", path.display()));
    }

    Ok(parser)
}
