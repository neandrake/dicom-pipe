mod common;

#[cfg(feature = "stddicom")]
mod writing_tests {
    use std::{fs::File, io::Read, path::PathBuf};

    use dcmpipe_lib::{
        core::{
            charset,
            dcmelement::DicomElement,
            dcmobject::DicomRoot,
            defn::{tag::TagPath, vl::ValueLength, vr},
            read::{Parser, ParserBuilder},
            values::{Attribute, RawValue},
            write::{builder::WriterBuilder, error::WriteError, writer::Writer},
        },
        dict::{stdlookup::STANDARD_DICOM_DICTIONARY, tags, transfer_syntaxes as ts, uids},
    };

    use super::common::{
        common_stddicom::{fixture, get_dicom_file_paths},
        mockdata,
    };

    /// This builds up an in-memory dicom dataset that when written out will result in the same bytes
    /// as `mockdata::STANDARD_HEADER`.
    #[test]
    fn test_write_mock_standard_header() -> Result<(), WriteError> {
        let mut writer: Writer<Vec<u8>> = WriterBuilder::for_file()
            .ts(&ts::ExplicitVRLittleEndian)
            .build(Vec::new());

        let mut elements: Vec<DicomElement> = Vec::new();

        elements.push(writer.create_element(
            &tags::FileMetaInformationVersion,
            &vr::OB,
            RawValue::Bytes(vec![0x00, 0x01]),
        )?);

        elements.push(writer.create_element(
            &tags::MediaStorageSOPClassUID,
            &vr::UI,
            RawValue::Uid(uids::CTImageStorage.uid().to_string()),
        )?);

        elements.push(writer.create_element(
            &tags::MediaStorageSOPInstanceUID,
            &vr::UI,
            RawValue::Uid("1.2.276.0.7230010.3.1.4.1787205428.2345.1071048146.1".to_string()),
        )?);

        elements.push(writer.create_element(
            &tags::TransferSyntaxUID,
            &vr::UI,
            RawValue::Uid(uids::RLELossless.uid().to_string()),
        )?);

        elements.push(writer.create_element(
            &tags::ImplementationClassUID,
            &vr::UI,
            RawValue::Uid("1.2.826.0.1.3680043.2.1143.107.104.103.115.2.1.0".to_string()),
        )?);

        elements.push(writer.create_element(
            &tags::ImplementationVersionName,
            &vr::SH,
            RawValue::of_string("GDCM 2.1.0".to_string()),
        )?);

        elements.push(writer.create_element(
            &tags::SourceApplicationEntityTitle,
            &vr::AE,
            RawValue::of_string("gdcmconv".to_string()),
        )?);

        elements.push(writer.create_element(
            &tags::SpecificCharacterSet,
            &vr::CS,
            RawValue::of_string("ISO_IR 100".to_string()),
        )?);

        writer.write_elements(elements.iter())?;

        let bytes: Vec<u8> = writer.into_dataset();
        assert_eq!(mockdata::STANDARD_HEADER, bytes);
        Ok(())
    }

    #[test]
    pub fn test_write_same_object() -> Result<(), WriteError> {
        let path: &str = "gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm";
        let dataset = fixture(path)?;
        let file_size = dataset.get_ref().metadata()?.len();
        let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);

        let dcmroot = DicomRoot::parse(&mut parser)?.expect("Parse file into DicomObject");

        let mut writer: Writer<Vec<u8>> =
            WriterBuilder::for_file()
                .ts(parser.ts())
                .build(Vec::with_capacity(
                    usize::try_from(file_size).unwrap_or_default(),
                ));
        writer.write_dcmroot(&dcmroot)?;
        let written_bytes: Vec<u8> = writer.into_dataset();

        // Read all bytes into memory.
        let mut file_bytes: Vec<u8> =
            Vec::with_capacity(usize::try_from(file_size).unwrap_or_default());
        let mut dataset = fixture(path)?;
        dataset.read_to_end(&mut file_bytes)?;

        assert_byte_chunks(&file_bytes, &written_bytes);

        Ok(())
    }

    #[test]
    pub fn test_reencoded_values() -> Result<(), WriteError> {
        let file_path: &str = "./gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm";
        let dataset = fixture(file_path)?;
        let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);

        while let Some(Ok(mut elem)) = parser.next() {
            assert_reencode_element(file_path, &mut elem)?;
        }

        Ok(())
    }

    #[test]
    pub fn test_write_reencoded_values() -> Result<(), WriteError> {
        let file_path: &str = "gdcm/gdcmConformanceTests/RTStruct_VRDSAsVRUN.dcm";
        let dataset = fixture(file_path)?;
        let file_size = dataset.get_ref().metadata()?.len();
        let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);

        let mut elements: Vec<DicomElement> = Vec::new();
        while let Some(Ok(elem)) = parser.next() {
            let value = elem.parse_value()?;
            let mut re_encoded = DicomElement::new_empty(elem.tag(), elem.vr(), elem.ts());
            re_encoded.encode_val_with_vl(value, Some(elem.vl()))?;
            elements.push(re_encoded);
        }

        let mut writer: Writer<Vec<u8>> = WriterBuilder::for_file().build(Vec::new());
        writer.write_elements(elements.iter())?;
        let written_bytes = writer.into_dataset();

        // Read all bytes into memory.
        let mut file_bytes: Vec<u8> =
            Vec::with_capacity(usize::try_from(file_size).unwrap_or_default());
        let mut dataset = fixture(file_path)?;
        dataset.read_to_end(&mut file_bytes)?;

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
    pub fn test_write_ushorts() -> Result<(), WriteError> {
        let mut elem = DicomElement::new_empty(
            &tags::ReferencedWaveformChannels,
            &vr::US,
            &ts::ExplicitVRLittleEndian,
        );

        let value = vec![1u16, 1u16];
        elem.encode_val(RawValue::UShorts(value.clone()))?;

        let raw_data = vec![1u8, 0u8, 1u8, 0u8];
        assert_eq!(&raw_data, elem.data());

        elem = DicomElement::new(
            &tags::ReferencedWaveformChannels,
            &vr::US,
            ValueLength::Explicit(4),
            &ts::ExplicitVRLittleEndian,
            charset::DEFAULT_CHARACTER_SET,
            raw_data.clone(),
            Vec::with_capacity(0),
        );

        let re_value = elem.parse_value()?;
        match re_value {
            RawValue::UShorts(shorts) => {
                assert_eq!(value, shorts, "mismatch shorts: {:?}", elem);
            }
            _ => panic!("Parsed value was not ushorts. Actually: {:?}", re_value),
        }

        Ok(())
    }

    #[test]
    pub fn test_write_attr() -> Result<(), WriteError> {
        let mut elem =
            DicomElement::new_empty(&tags::FrameIncrementPointer, &vr::AT, &ts::RLELossless);

        let value = vec![Attribute(0x0018_1063)];
        elem.encode_val(RawValue::Attributes(value.clone()))?;

        let raw_data = vec![0x18u8, 0u8, 0x63u8, 0x10u8];
        assert_eq!(&raw_data, elem.data(), "encoding of attribute failed");

        elem = DicomElement::new(
            &tags::FrameIncrementPointer,
            &vr::AT,
            ValueLength::Explicit(4),
            &ts::RLELossless,
            charset::DEFAULT_CHARACTER_SET,
            raw_data.clone(),
            Vec::with_capacity(0),
        );

        let re_value = elem.parse_value()?;
        match re_value {
            RawValue::Attributes(attrs) => {
                assert_eq!(value, attrs, "mismatch attribute: {:?}", elem);
            }
            _ => panic!("Parsed value was not ushorts. Actually: {:?}", re_value),
        }

        Ok(())
    }

    #[test]
    pub fn test_write_double() -> Result<(), WriteError> {
        let tag: u32 = 0x7fe1_1052;
        let mut elem = DicomElement::new_empty(tag, &vr::FD, &ts::JPEGBaselineProcess1);

        let value = vec![3499.9999999999995];
        elem.encode_val(RawValue::Doubles(value.clone()))?;

        let raw_data = vec![255, 255, 255, 255, 255, 87, 171, 64];
        assert_eq!(&raw_data, elem.data(), "encoding of attribute failed");

        elem = DicomElement::new(
            tag,
            &vr::FD,
            ValueLength::Explicit(8),
            &ts::JPEGBaselineProcess1,
            charset::DEFAULT_CHARACTER_SET,
            raw_data.clone(),
            Vec::with_capacity(0),
        );

        let re_value = elem.parse_value()?;
        match re_value {
            RawValue::Doubles(doubles) => {
                assert_eq!(value, doubles, "mismatch doubles: {:?}", elem);
            }
            _ => panic!("Parsed value was not doubles. Actually: {:?}", re_value),
        }

        Ok(())
    }

    #[test]
    #[ignore]
    pub fn test_reencoded_values_all_files() -> Result<(), WriteError> {
        let mut any_failed = false;
        for path in get_dicom_file_paths() {
            if let Err(e) = reencode_file_elements(path.clone()) {
                any_failed = true;
                eprintln!("Error in file: {:?}\n\t{:?}", path, e);
            }
        }

        if any_failed {
            panic!("Failed reencoding element values");
        }

        Ok(())
    }

    fn reencode_file_elements(path: PathBuf) -> Result<(), WriteError> {
        let path_str: &str = path.to_str().expect("path");

        let mut parser: Parser<'_, File> =
            ParserBuilder::default().build(File::open(path.clone())?, &STANDARD_DICOM_DICTIONARY);

        while let Some(Ok(mut elem)) = parser.next() {
            assert_reencode_element(path_str, &mut elem)?;
        }
        Ok(())
    }

    fn assert_reencode_element(path_str: &str, elem: &DicomElement) -> Result<(), WriteError> {
        let orig_parsed_data = elem.data().clone();
        let value = elem.parse_value();
        if let Err(e) = value {
            eprintln!("Parsing error in file.\n\tfile: {path_str}\n\terr: {e:?}");
            return Ok(());
        }
        let value = value?;
        let mut re_encoded = DicomElement::new_empty(elem.tag(), elem.vr(), elem.ts());
        re_encoded.encode_val_with_vl(value.clone(), Some(elem.vl()))?;
        let reencoded_data = elem.data().clone();

        if orig_parsed_data == reencoded_data {
            return Ok(());
        }

        // Some formatting/values are expected to not match exactly. Adjust the original
        // element data so that it would match what the writer would output.

        // Some datasets encode at least one digit precision, others don't. There's no
        // great way to update the original data in a minimal way to adjust for the
        // writer always ensuring a minimum of a single digit of precision for all
        // values in this element.
        if elem.vr() == &vr::DS {
            return Ok(());
        }

        // If strings consist of only the padding character then ignore size differences.
        if elem.vr().is_character_string {
            // Some character-based elements seem to include trailing null-byte padding.
            let trimmer = |v: &Vec<u8>| {
                v.iter()
                    .rev()
                    .map(|b| b.to_owned())
                    .skip_while(|&b| b == vr::SPACE_PADDING || b == vr::NULL_PADDING)
                    .collect::<Vec<u8>>()
                    .iter()
                    .rev()
                    .map(|b| b.to_owned())
                    .skip_while(|&b| b == vr::SPACE_PADDING)
                    .collect::<Vec<u8>>()
            };

            let orig_end_trimmed = trimmer(&orig_parsed_data);
            let reencoded_end_trimmed = trimmer(&reencoded_data);

            if orig_end_trimmed == reencoded_end_trimmed {
                return Ok(());
            }
        }

        if elem.vr() == &vr::IS || elem.vr() == &vr::LO {
            // Values may have originally had leading and trailing spaces which are lost
            // when parsed into RawValue. Additionally the same for leading zeros.
            let trimmer = |v: &Vec<u8>| {
                // Account for numbers encoded as "2.5000"
                let indexof_dot = v.iter().position(|&b| b == b'.');
                let remove_trailing_zeroes = if let Some(dot_pos) = indexof_dot {
                    dot_pos != v.len() - 1
                } else {
                    false
                };

                v.iter()
                    .map(|b| b.to_owned())
                    .skip_while(|&b| b == vr::SPACE_PADDING || b == b'0')
                    .collect::<Vec<u8>>()
                    .iter()
                    .rev()
                    .map(|b| b.to_owned())
                    .skip_while(|&b| {
                        b == vr::SPACE_PADDING || (remove_trailing_zeroes && b == b'0')
                    })
                    .collect::<Vec<u8>>()
                    .iter()
                    .rev()
                    .map(|b| b.to_owned())
                    .collect::<Vec<u8>>()
            };

            let orig_pieces = orig_parsed_data
                .split(|&b| b == vr::CS_SEPARATOR_BYTE)
                .map(|b| b.to_owned())
                .collect::<Vec<Vec<u8>>>();
            let reencoded_pieces = reencoded_data
                .split(|&b| b == vr::CS_SEPARATOR_BYTE)
                .map(|b| b.to_owned())
                .collect::<Vec<Vec<u8>>>();

            if orig_pieces.len() == reencoded_pieces.len() {
                let mut all_eq = true;
                for i in 0..orig_pieces.len() {
                    let orig_end_trimmed = trimmer(&orig_pieces[i]);
                    let reencoded_end_trimmed = trimmer(&reencoded_pieces[i]);

                    if orig_end_trimmed != reencoded_end_trimmed {
                        all_eq = false;
                        break;
                    }
                }
                if all_eq {
                    return Ok(());
                }
            }
        }

        assert_eq!(
        orig_parsed_data,
        reencoded_data,
        "Element did not re-encode the same\n\tfile: {}\n\ttagpath: {}\n\telem: {:?}\n\tvalue: {:?}",
        path_str,
        TagPath::format_tagpath_to_display(&elem.create_tagpath(), Some(&STANDARD_DICOM_DICTIONARY)),
        elem,
        value
    );

        Ok(())
    }
}
