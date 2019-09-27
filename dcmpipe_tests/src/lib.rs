extern crate encoding;
extern crate walkdir;

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_lib::core::dcmelement::{Attribute, DicomElement};
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::dcmparser::{
    Parser, ParserBuilder, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};
use dcmpipe_lib::core::dcmparser_util::parse_into_object;
use dcmpipe_lib::defn::tag::Tag;
use dcmpipe_lib::defn::vl::ValueLength;
use dcmpipe_lib::defn::vr;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[cfg(test)]
mod charsets;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod parsing;

/// Parses the given file into a `DicomObject`
pub fn parse_file(path: &str, with_std: bool) -> Result<DicomRoot, Error> {
    let file: File = File::open(path)?;
    let mut parser: ParserBuilder<File> = ParserBuilder::new(file);
    if with_std {
        parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let mut parser: Parser<File> = parser.build();
    let dcmroot: DicomRoot = parse_into_object(&mut parser)?;
    parse_all_dcmroot_values(&dcmroot)?;
    Ok(dcmroot)
}

/// Parses through all dicom files in the `fixtures` folder. The `use_std_dict` argument specifies
/// whether the standard dicom dictionary should be reigstered with the parser.
pub fn parse_all_dicom_files(with_std: bool) -> Result<usize, Error> {
    let mut num_failed: usize = 0;
    for path in get_dicom_file_paths() {
        let path_str: &str = path.to_str().expect("path");
        let file: File = File::open(path.clone())?;
        let mut parser: ParserBuilder<File> = ParserBuilder::new(file);
        if with_std {
            parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
        }
        let parser: Parser<File> = parser.build();

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
pub fn is_standard_dcm_file<DatasetType>(parser: &Parser<DatasetType>) -> bool
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

pub fn parse_all_dcmroot_values(dcmroot: &DicomRoot) -> Result<(), Error> {
    for (_tag, dcmobj) in dcmroot.iter_child_nodes() {
        parse_all_dcmobj_values(dcmobj)?;
    }
    Ok(())
}

fn parse_all_dcmobj_values(dcmobj: &DicomObject) -> Result<(), Error> {
    parse_element_value(dcmobj.as_element())?;
    for (_tag, child_dcmobj) in dcmobj.iter_child_nodes() {
        parse_all_dcmobj_values(child_dcmobj)?;
    }
    Ok(())
}

pub fn parse_all_element_values(parser: Parser<File>, path_str: &str) -> Result<(), Error> {
    for elem_result in parser {
        match elem_result {
            Ok(elem) => {
                match parse_element_value(&elem) {
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

pub fn parse_element_value(elem: &DicomElement) -> Result<(), Error> {
    if elem.vr == &vr::AT {
        Attribute::try_from(elem)?;
    } else if elem.vr == &vr::FD || elem.vr == &vr::OF || elem.vr == &vr::OD || elem.vr == &vr::FL {
        match elem.vl {
            ValueLength::Explicit(len)
                if (elem.vr == &vr::OD || elem.vr == &vr::FL) && len > 0 && len % 8 == 0 =>
            {
                Vec::<f64>::try_from(elem)?
            }
            ValueLength::Explicit(len) if len > 0 && len % 4 == 0 => Vec::<f32>::try_from(elem)?
                .into_iter()
                .map(f64::from)
                .collect::<Vec<f64>>(),
            ValueLength::Explicit(1) => vec![f64::from(elem.get_data()[0])],
            _ => vec![],
        };
    } else if elem.vr == &vr::SS {
        match elem.vl {
            ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => Vec::<i16>::try_from(elem)?,
            ValueLength::Explicit(1) => vec![i16::from(elem.get_data()[0])],
            _ => vec![],
        };
    } else if elem.vr == &vr::SL {
        match elem.vl {
            ValueLength::Explicit(len) if len > 0 && len % 4 == 0 => Vec::<i32>::try_from(elem)?,
            ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => Vec::<i16>::try_from(elem)?
                .into_iter()
                .map(i32::from)
                .collect::<Vec<i32>>(),
            ValueLength::Explicit(1) => vec![i32::from(elem.get_data()[0])],
            _ => vec![],
        };
    } else if elem.vr == &vr::UI {
        String::try_from(elem)?;
    } else if elem.vr == &vr::UL || elem.vr == &vr::OL || elem.vr == &vr::OW || elem.vr == &vr::US {
        match elem.vl {
            ValueLength::Explicit(len)
                if (elem.vr == &vr::UL || elem.vr == &vr::OL) && len > 0 && len % 4 == 0 =>
            {
                Vec::<u32>::try_from(elem)?
            }
            ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => Vec::<u16>::try_from(elem)?
                .into_iter()
                .map(u32::from)
                .collect::<Vec<u32>>(),
            ValueLength::Explicit(1) => vec![u32::from(elem.get_data()[0])],
            _ => vec![],
        };
    } else if elem.vr.is_character_string {
        Vec::<String>::try_from(elem)?;
    }

    Ok(())
}
