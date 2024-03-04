// The integreation tests in /tests folder are treated as individual crates, which results in the
// rust compiler not detecting that this common crate is used, throwing up a bunch of warnings.
#![allow(unused)]

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use dcmpipe_lib::{
    core::{
        dcmobject::{DicomObject, DicomRoot},
        defn::{constants::lookup::MINIMAL_DICOM_DICTIONARY, dcmdict::DicomDictionary},
        read::{ParseResult, Parser, ParserBuilder},
        DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
    },
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
};

use walkdir::WalkDir;

pub mod mock;
pub mod mockdata;

/// The path to the fixtures, relative to the `dcmpipe_lib` crate which is the working directory
/// when tests are run.
const FIXTURES_PATH: &'static str = "./tests/fixtures";

/// Gets the fixture file of the given file path, relative to the fixtures directory.
pub fn fixture(path: &str) -> Result<File, std::io::Error> {
    File::open(Path::new(FIXTURES_PATH).join(path))
}

/// Parses the given file into a `DicomObject`. The fixture file path should be relative to the
/// fixtures directory.
pub fn parse_file(path: &str, with_std: bool) -> ParseResult<DicomRoot<'_>> {
    let dict: &dyn DicomDictionary = if with_std {
        &STANDARD_DICOM_DICTIONARY
    } else {
        &MINIMAL_DICOM_DICTIONARY
    };

    let mut parser: Parser<'_, File> = ParserBuilder::default()
        .dictionary(dict)
        .build(fixture(path)?);
    let dcmroot: DicomRoot<'_> = DicomRoot::parse(&mut parser)?.unwrap();
    parse_all_dcmroot_values(&dcmroot)?;
    Ok(dcmroot)
}

/// Parses through all dicom files in the `fixtures` folder. The `use_std_dict` argument specifies
/// whether the standard dicom dictionary should be reigstered with the parser.
pub fn parse_all_dicom_files(with_std: bool) -> ParseResult<usize> {
    let dict: &dyn DicomDictionary = if with_std {
        &STANDARD_DICOM_DICTIONARY
    } else {
        &MINIMAL_DICOM_DICTIONARY
    };

    let mut num_failed: usize = 0;
    for path in get_dicom_file_paths() {
        let path_str: &str = path.to_str().expect("path");

        let parser: Parser<'_, File> = ParserBuilder::default()
            .dictionary(dict)
            .build(File::open(path.clone())?);

        if parse_all_element_values(parser, path_str).is_err() {
            num_failed += 1;
        }
    }

    Ok(num_failed)
}

/// Gets the paths to all dicom files within the `fixtures` directory.
/// See the `readme.md` in this project for information on obtaining test fixtures.
pub fn get_dicom_file_paths() -> impl Iterator<Item = PathBuf> {
    let fixtures_path: &Path = Path::new(FIXTURES_PATH);
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
pub fn is_standard_dcm_file<R: Read>(parser: &Parser<'_, R>) -> bool {
    match parser.file_preamble() {
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

    match parser.dicom_prefix() {
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

pub fn parse_all_dcmroot_values(dcmroot: &DicomRoot<'_>) -> ParseResult<()> {
    // This should always do nothing as the root should never have items.
    for dcmobj in dcmroot.iter_items() {
        parse_all_dcmobj_values(dcmobj)?;
    }
    for (_tag, dcmobj) in dcmroot.iter_child_nodes() {
        parse_all_dcmobj_values(dcmobj)?;
    }
    Ok(())
}

fn parse_all_dcmobj_values(dcmobj: &DicomObject) -> ParseResult<()> {
    // Parse current element value before moving on to items/children.
    dcmobj.element().parse_value()?;
    for item_dcmobj in dcmobj.iter_items() {
        parse_all_dcmobj_values(item_dcmobj)?;
    }
    for (_tag, child_dcmobj) in dcmobj.iter_child_nodes() {
        parse_all_dcmobj_values(child_dcmobj)?;
    }
    Ok(())
}

pub fn parse_all_element_values(parser: Parser<'_, File>, path_str: &str) -> ParseResult<()> {
    for elem_result in parser {
        match elem_result {
            Ok(elem) => {
                match elem.parse_value() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        eprintln!(
                            "Error parsing DICOM Element:\n\tfile: {}\n\terr: {}",
                            path_str, e
                        );
                        Err(e)
                    }
                }?;
                Ok(())
            }
            Err(e) => {
                eprintln!("Error parsing DICOM:\n\tfile: {}\n\terr: {}", path_str, e);
                Err(e)
            }
        }?;
    }

    Ok(())
}
