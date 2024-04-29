use dcmpipe_dict::dict::{tags, transfer_syntaxes as ts, uids};
use dcmpipe_lib::{
    core::{
        dcmelement::{DicomElement, RawValue},
        write::{error::WriteError, writer::Writer},
    },
    defn::vr,
};

use crate::mockdata;

/// This builds up an in-memory dicom dataset that when written out will result in the same bytes
/// as `mockdata::STANDARD_HEADER`.
#[test]
fn test_write_mock_standard_header() -> Result<(), WriteError> {
    let mut writer: Writer<Vec<u8>> = Writer::new(Vec::new());
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
    assert_eq!(bytes, mockdata::STANDARD_HEADER);
    Ok(())
}
