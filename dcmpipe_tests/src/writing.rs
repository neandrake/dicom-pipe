use std::{fs::File, io::Read};

use dcmpipe_dict::dict::{
    stdlookup::STANDARD_DICOM_DICTIONARY, tags, transfer_syntaxes as ts, uids,
};
use dcmpipe_lib::{
    core::{
        dcmelement::{DicomElement, RawValue},
        dcmobject::DicomRoot,
        read::{Parser, ParserBuilder},
        write::{error::WriteError, writer::Writer},
    },
    defn::vr,
};

use crate::mockdata;

/// This builds up an in-memory dicom dataset that when written out will result in the same bytes
/// as `mockdata::STANDARD_HEADER`.
#[test]
fn test_write_mock_standard_header() -> Result<(), WriteError> {
    let mut writer: Writer<Vec<u8>> = Writer::to_file(Vec::new());
    writer.set_ts(&ts::ExplicitVRLittleEndian);

    let mut elements: Vec<DicomElement> = Vec::new();

    elements.push(writer.create_element(
        tags::FileMetaInformationVersion.tag,
        &vr::OB,
        RawValue::Bytes(vec![0x00, 0x01]),
    )?);

    elements.push(writer.create_element(
        tags::MediaStorageSOPClassUID.tag,
        &vr::UI,
        RawValue::Uid(uids::CTImageStorage.get_uid().to_string()),
    )?);

    elements.push(writer.create_element(
        tags::MediaStorageSOPInstanceUID.tag,
        &vr::UI,
        RawValue::Uid("1.2.276.0.7230010.3.1.4.1787205428.2345.1071048146.1".to_string()),
    )?);

    elements.push(writer.create_element(
        tags::TransferSyntaxUID.tag,
        &vr::UI,
        RawValue::Uid(uids::RLELossless.get_uid().to_string()),
    )?);

    elements.push(writer.create_element(
        tags::ImplementationClassUID.tag,
        &vr::UI,
        RawValue::Uid("1.2.826.0.1.3680043.2.1143.107.104.103.115.2.1.0".to_string()),
    )?);

    elements.push(writer.create_element(
        tags::ImplementationVersionName.tag,
        &vr::SH,
        RawValue::Strings(vec!["GDCM 2.1.0".to_string()]),
    )?);

    elements.push(writer.create_element(
        tags::SourceApplicationEntityTitle.tag,
        &vr::AE,
        RawValue::Strings(vec!["gdcmconv".to_string()]),
    )?);

    elements.push(writer.create_element(
        tags::SpecificCharacterSet.tag,
        &vr::CS,
        RawValue::Strings(vec!["ISO_IR 100".to_string()]),
    )?);

    writer.write_elements(elements.iter())?;

    let bytes: Vec<u8> = writer.into_dataset()?;
    assert_eq!(mockdata::STANDARD_HEADER, bytes);
    Ok(())
}

#[test]
pub fn test_write_same_object() -> Result<(), WriteError> {
    let file_path: &str = "./fixtures/gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm";
    let file: File = File::open(file_path)?;
    let file_size = file.metadata()?.len();
    let mut parser = ParserBuilder::default()
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

    let dcmroot = DicomRoot::parse(&mut parser)?.expect("Parse file into DicomObject");

    let mut writer: Writer<Vec<u8>> = Writer::to_file(Vec::new());
    writer.set_ts(parser.get_ts());
    writer.write_dcmroot(&dcmroot)?;
    let written_bytes: Vec<u8> = writer.into_dataset()?;

    // Read all bytes into memory.
    let mut file_bytes: Vec<u8> = Vec::with_capacity(file_size as usize);
    let mut file: File = File::open(file_path)?;
    file.read_to_end(&mut file_bytes)?;

    assert_byte_chunks(&file_bytes, &written_bytes);

    Ok(())
}

#[test]
pub fn test_reencoded_values() -> Result<(), WriteError> {
    let file_path: &str = "./fixtures/gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm";
    let file: File = File::open(file_path)?;
    let mut parser = ParserBuilder::default()
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

    while let Some(Ok(mut elem)) = parser.next() {
        let data = elem.get_data().clone();
        let value = elem.parse_value()?;
        elem.encode_value(value, Some(elem.get_vl()))?;
        let reencoded_data = elem.get_data().clone();

        assert_eq!(
            data, reencoded_data,
            "Element did not re-encode the same: {:?}",
            elem
        );
    }

    Ok(())
}

#[test]
pub fn test_write_reencoded_values() -> Result<(), WriteError> {
    let file_path: &str = "./fixtures/gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm";
    let file: File = File::open(file_path)?;
    let file_size = file.metadata()?.len();
    let mut parser = ParserBuilder::default()
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

    let mut elements: Vec<DicomElement> = Vec::new();
    while let Some(Ok(mut elem)) = parser.next() {
        let value = elem.parse_value()?;
        elem.encode_value(value, Some(elem.get_vl()))?;
        elements.push(elem);
    }

    let mut writer: Writer<Vec<u8>> = Writer::to_file(Vec::new());
    writer.write_elements(elements.iter())?;
    let written_bytes = writer.into_dataset()?;

    // Read all bytes into memory.
    let mut file_bytes: Vec<u8> = Vec::with_capacity(file_size as usize);
    let mut file: File = File::open(file_path)?;
    file.read_to_end(&mut file_bytes)?;

    assert_byte_chunks(&file_bytes, &written_bytes);

    Ok(())
}

fn assert_byte_chunks(file_bytes: &Vec<u8>, written_bytes: &Vec<u8>) {
    let chunk_size = 0x1000;
    let written_chunks = written_bytes
        .chunks(chunk_size)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<u8>>>();
    let file_chunks = file_bytes
        .chunks(chunk_size)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<u8>>>();

    for i in 0..written_chunks.len() {
        assert_eq!(
            file_chunks.get(i).unwrap(),
            written_chunks.get(i).unwrap(),
            "chunk mismatch: {}",
            i
        );
    }
}

#[test]
#[ignore]
pub fn test_reencoded_values_all_files() -> Result<(), WriteError> {
    for path in crate::get_dicom_file_paths() {
        let path_str: &str = path.to_str().expect("path");

        let mut parser: Parser<'_, File> = ParserBuilder::default()
            .dictionary(&STANDARD_DICOM_DICTIONARY)
            .build(File::open(path.clone())?);

        while let Some(Ok(mut elem)) = parser.next() {
            let orig_parsed_data = elem.get_data().clone();
            let value = elem.parse_value()?;
            elem.encode_value(value, Some(elem.get_vl()))?;
            let reencoded_data = elem.get_data().clone();

            assert_eq!(
                orig_parsed_data, reencoded_data,
                "Element did not re-encode the same. {} : {:?}",
                path_str, elem
            );
        }
    }

    Ok(())
}
