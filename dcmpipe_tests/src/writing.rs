use dcmpipe_dict::dict::{tags, transfer_syntaxes as ts, uids};
use dcmpipe_lib::{
    core::{
        dcmelement::{DicomElement, RawValue},
        write::writer::Writer,
    },
    defn::vr,
};

use crate::mockdata;

#[test]
fn test_basic_writing_against_mock() -> Result<(), std::io::Error> {
    let mut writer: Writer<Vec<u8>> = Writer::new(Vec::new());
    let mut elements: Vec<DicomElement> = Vec::new();

    let mut element = DicomElement::new_empty(
        tags::FileMetaInformationVersion.tag,
        &vr::OB,
        &ts::ExplicitVRLittleEndian,
    );
    element
        .encode_value(RawValue::Bytes(vec![0x00, 0x00, 0x00, 0x01]))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    elements.push(element);

    element = DicomElement::new_empty(
        tags::MediaStorageSOPClassUID.tag,
        &vr::UI,
        &ts::ExplicitVRLittleEndian,
    );
    element
        .encode_value(RawValue::Uid(uids::CTImageStorage.get_uid().to_string()))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    elements.push(element);

    element = DicomElement::new_empty(
        tags::MediaStorageSOPInstanceUID.tag,
        &vr::UI,
        &ts::ExplicitVRLittleEndian,
    );
    element
        .encode_value(RawValue::Uid("1.2.276.0.7230010.3.1.4.1787205428.2345.1071048146.1".to_string()))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    elements.push(element);

    element = DicomElement::new_empty(
        tags::TransferSyntaxUID.tag,
        &vr::UI,
        &ts::ExplicitVRLittleEndian,
    );
    element
        .encode_value(RawValue::Uid(uids::RLELossless.get_uid().to_string()))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    elements.push(element);

    writer
        .write_elements(elements.iter())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let bytes: Vec<u8> = writer.into_dataset()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    assert_eq!(bytes, mockdata::STANDARD_HEADER);

    Ok(())
}
