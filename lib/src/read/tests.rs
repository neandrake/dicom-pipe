#![cfg(test)]

extern crate byteorder;
extern crate walkdir;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use core::dict::dicom_elements as tags;
use core::tag::TagRef;
use core::vr::DEFAULT_CHARACTER_SET;

use read::dcmdataset::DicomDataSetContainer;
use read::dcmstream::{DicomStream, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH};
use read::mock::MockDicomStream;
use read::tagstop::TagStop;

use std::fs::File;
use std::io::Seek;
use std::path::Path;

use self::walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "../fixtures/dataset1";

#[test]
fn test_good_preamble() {
    let mut test_good_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble();
    test_good_stream.read_file_preamble()
        .expect("Unable to read file preamble");
    test_good_stream.read_dicom_prefix()
        .expect("Unable to read DICOM prefix");
    let is_dcm: bool = is_standard_preamble(&test_good_stream);
    assert_eq!(is_dcm, true);
}

#[test]
fn test_nonzero_preamble() {
    let mut test_size_stream: DicomStream<MockDicomStream> =
        MockDicomStream::nonzero_preamble();
    test_size_stream.read_file_preamble()
        .expect("Unable to read file preamble");
    test_size_stream.read_dicom_prefix()
        .expect("Unable to read DICOM prefix");
    let is_dcm: bool = is_standard_preamble(&test_size_stream);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "Invalid DICOM Prefix")]
fn test_bad_dicom_prefix() {
    let mut test_bad_stream: DicomStream<MockDicomStream> =
        MockDicomStream::invalid_dicom_prefix();
    test_bad_stream.read_file_preamble()
        .expect("Unable to read file preamble");
    test_bad_stream.read_dicom_prefix()
        .expect("This should fail to read a valid prefix");
    let is_dcm: bool = is_standard_preamble(&test_bad_stream);
    assert_eq!(is_dcm, false);
}

#[test]
#[should_panic(expected = "failed to fill whole buffer")]
fn test_failure_to_read_preamble() {
    let mut test_stream: DicomStream<MockDicomStream> =
        MockDicomStream::standard_dicom_preamble_diff_startpos_and_short_stream();
    test_stream.read_file_preamble()
        .expect("This should fail to read preamble due to not enough data");
    let start_pos: u64 = test_stream.get_stream().position();
    assert!(start_pos != 0);
}

#[test]	// slow
fn test_parse_known_dicom_files() {
    let mut dstream: DicomStream<File> = get_first_file_stream();
    
    dstream.read_file_meta()
        .expect("Unable to read FileMetaInformation");

    let is_dcm = is_standard_preamble(&dstream);
    assert!(is_dcm);

    // Ability to read dicom elements after FileMetaInformation
    // means that we interpret the transfer syntax properly, as
    // the fixtures are implicit VR (FMI is encoded as explicit)

    let read_until_before_tag: TagRef = &tags::PixelData;
    dstream.read_until(TagStop::BeforeTag(read_until_before_tag.tag))
        .expect("Error reading elements");
    
    // read_until() should have stopped just prior to reading PixelData
    let next_tag: u32 = if dstream.get_ts().big_endian {
        dstream.read_next_tag::<BigEndian>()
    } else {
        dstream.read_next_tag::<LittleEndian>()
    }.expect("Unable to read next tag");
    assert_eq!(next_tag, read_until_before_tag.tag);

    // subsequent call to read_next_tag() should not advance reading elements
    let next_tag: u32 = if dstream.get_ts().big_endian {
        dstream.read_next_tag::<BigEndian>()
    } else {
        dstream.read_next_tag::<LittleEndian>()
    }.expect("Unable to read next tag");
    assert_eq!(next_tag, read_until_before_tag.tag);

    // read_until() being given the same tag to read until should not
    // read any elements
    dstream.read_until_on_each(
        TagStop::BeforeTag(read_until_before_tag.tag),
        |_ds: &mut DicomStream<File>, _element_tag: u32| {
            panic!("Should not read additional elements");
        })
        .expect("Error reading elements");
    
    // repeated calls to retrieve the same tag value should return the same value
    {
        let specific_charset: &String = dstream.get_string(tags::SpecificCharacterSet.tag, DEFAULT_CHARACTER_SET)
            .expect("Should have read a charset");
        assert_eq!("ISO_IR 100", *specific_charset);
    }
    {
        let specific_charset: &String = dstream.get_string(tags::SpecificCharacterSet.tag, DEFAULT_CHARACTER_SET)
            .expect("Should have read a charset");
        assert_eq!("ISO_IR 100", *specific_charset);
    }
}

fn get_first_file_stream() -> DicomStream<File> {
    let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
        .min_depth(1)
        .max_depth(1);

    for entry_res in dirwalker.into_iter() {
        let entry: DirEntry = entry_res.unwrap();
        let path: &Path = entry.path();

        let dstream: DicomStream<File> = DicomStream::new_from_path(path)
            .expect(&format!("Unable to read file: {:?}", path));
        
        return dstream;
    }

    panic!("No DICOM files found");
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'
fn is_standard_preamble<StreamType>(stream: &DicomStream<StreamType>) -> bool
    where StreamType: ReadBytesExt + Seek {
    for i in 0..FILE_PREAMBLE_LENGTH {
        if stream.get_file_preamble()[i] != 0 {
            return false;
        }
    }
    for i in 0..DICOM_PREFIX_LENGTH {
        if stream.get_dicom_prefix()[i] != DICOM_PREFIX[i] {
            return false;
        }
    }
    true
}