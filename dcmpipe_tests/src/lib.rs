use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::read::{Parser, ParserBuilder, Result};
use dcmpipe_lib::core::{DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH};

use dcmpipe_lib::core::read::stop::ParseStop;
use dcmpipe_lib::core::read::util::parse_into_object;
use dcmpipe_lib::defn::tag::Tag;

#[cfg(test)]
mod charsets;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod parsing;

/// Parses the given file into a `DicomObject`
pub fn parse_file(path: &str, with_std: bool) -> Result<DicomRoot<'_>> {
    parse_file_with_parsestop(path, with_std, ParseStop::EndOfDataset)
}

/// Parses the given file into a `DicomObject` using the given parsestop
pub fn parse_file_with_parsestop(
    path: &str,
    with_std: bool,
    stop: ParseStop,
) -> Result<DicomRoot<'_>> {
    let file: File = File::open(path)?;

    let mut parser_builder: ParserBuilder<'_> = ParserBuilder::default().stop(stop);
    if with_std {
        parser_builder = parser_builder.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let parser: Parser<'_, File> = parser_builder.build(file);
    parse_all_element_values(parser, path)?;

    // TODO: There's a difference between parsing all values directly from parser vs. read in
    //       from the dicom object?

    let file: File = File::open(path)?;
    let mut parser: Parser<'_, File> = parser_builder.build(file);
    let dcmroot: DicomRoot<'_> = parse_into_object(&mut parser)?.unwrap();
    parse_all_dcmroot_values(&dcmroot)?;
    Ok(dcmroot)
}

pub fn parse_and_print_file(path: &str, with_std: bool) -> Result<()> {
    let file: File = File::open(path)?;
    let mut parser: ParserBuilder<'_> = ParserBuilder::default();
    if with_std {
        parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let mut parser: Parser<'_, File> = parser.build(file);
    while let Some(Ok(_elem)) = parser.next() {}

    Ok(())
}

/// Parses through all dicom files in the `fixtures` folder. The `use_std_dict` argument specifies
/// whether the standard dicom dictionary should be reigstered with the parser.
pub fn parse_all_dicom_files(with_std: bool) -> Result<usize> {
    let mut num_failed: usize = 0;
    for path in get_dicom_file_paths() {
        let path_str: &str = path.to_str().expect("path");
        let file: File = File::open(path.clone())?;
        let mut parser: ParserBuilder<'_> = ParserBuilder::default();
        if with_std {
            parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
        }
        let parser: Parser<'_, File> = parser.build(file);

        if parse_all_element_values(parser, path_str).is_err() {
            num_failed += 1;
        }
    }

    Ok(num_failed)
}

/// Gets the paths to all dicom files within the `fixtures` directory.
/// See the `readme.md` in this project for information on obtaining test fixtures.
pub fn get_dicom_file_paths() -> impl Iterator<Item = PathBuf> {
    let fixtures_path: &Path = Path::new("./fixtures");
    assert!(
        fixtures_path.is_dir(),
        "The fixtures are missing and need downloaded"
    );

    WalkDir::new(fixtures_path)
        .into_iter()
        .map(|entry_res| entry_res.expect("walkdir entry").path().to_path_buf())
        .filter(|path: &PathBuf| {
            if !path.is_file() {
                return false;
            }

            let filename = path
                .file_name()
                .expect("Should be able to get filename")
                .to_str()
                .expect("Should be able to stringify filename");

            // Only attempt to parse .DCM files or files without any extension
            if filename.contains('.') && !filename.ends_with(".dcm") {
                false
            } else {
                !filename.eq("README")
            }
        })
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'.
/// DICOM files do not need to abide by this format to be valid, but it's standard.
pub fn is_standard_dcm_file<DatasetType>(parser: &Parser<'_, DatasetType>) -> bool
where
    DatasetType: Read,
{
    match parser.get_file_preamble() {
        Some(file_preamble) => {
            for b in file_preamble.iter().take(FILE_PREAMBLE_LENGTH) {
                if *b != 0 {
                    eprintln!("BAD PREAMBLE??");
                    return false;
                }
            }
        }
        None => {}
    }

    match parser.get_dicom_prefix() {
        Some(prefix) => {
            for i in 0..DICOM_PREFIX_LENGTH {
                if prefix[i] != DICOM_PREFIX[i] {
                    return false;
                }
            }
        }
        None => {}
    }
    true
}

pub fn parse_all_dcmroot_values(dcmroot: &DicomRoot<'_>) -> Result<()> {
    for (_tag, dcmobj) in dcmroot.iter_child_nodes() {
        parse_all_dcmobj_values(dcmobj)?;
    }
    Ok(())
}

fn parse_all_dcmobj_values(dcmobj: &DicomObject) -> Result<()> {
    dcmobj.get_element().parse_value()?;
    for (_tag, child_dcmobj) in dcmobj.iter_child_nodes() {
        parse_all_dcmobj_values(child_dcmobj)?;
    }
    Ok(())
}

pub fn parse_all_element_values(parser: Parser<'_, File>, path_str: &str) -> Result<()> {
    for elem_result in parser {
        match elem_result {
            Ok(elem) => {
                match elem.parse_value() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let sq_path: String = elem
                            .get_sequence_path()
                            .iter()
                            .map(|sq| sq.get_seq_tag())
                            .chain(vec![elem.tag])
                            .map(Tag::format_tag_to_display)
                            .collect::<Vec<String>>()
                            .join(".");
                        eprintln!(
                            "Error parsing DICOM Element:\n\t{}\n\t{}\n\t{}",
                            path_str, sq_path, e
                        );
                        Err(e)
                    }
                }?;
                Ok(())
            }
            Err(e) => {
                eprintln!("Error parsing DICOM:\n\t{}\n\t{}", path_str, e);
                Err(e)
            }
        }?;
    }

    Ok(())
}
