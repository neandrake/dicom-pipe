use std::io::Write;

use crate::core::charset::{CSRef, DEFAULT_CHARACTER_SET};
use crate::core::dcmelement::{DicomElement, RawValue};
use crate::core::dcmsqelem::SequenceElement;
use crate::core::read::ParseError;
use crate::core::write::ds::dataset::Dataset;
use crate::core::write::error::WriteError;
use crate::core::{DICOM_PREFIX, FILE_PREAMBLE_LENGTH};
use crate::defn::constants::{tags, ts};
use crate::defn::ts::TSRef;
use crate::defn::vl::{ValueLength, UNDEFINED_LENGTH};
use crate::defn::vr::{self, VRRef};

pub type Result<T> = core::result::Result<T, WriteError>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WriteState {
    Preamble,
    GroupLength,
    FileMeta,
    Element,
}

pub struct Writer<DatasetType: Write> {
    pub(crate) dataset: Dataset<DatasetType>,

    pub(crate) state: WriteState,

    pub(crate) bytes_written: u64,

    pub(crate) ts: TSRef,

    pub(crate) cs: CSRef,

    pub(crate) current_path: Vec<SequenceElement>,

    /// The file preamble to write to the dataset. Not all datasets may have a preamble.
    /// If a file preamble is specified then the `"DICM"` prefix will be written immediately
    /// after the file preamble is written.
    pub(crate) file_preamble: Option<[u8; FILE_PREAMBLE_LENGTH]>,
}

impl<DatasetType: Write> Writer<DatasetType> {
    pub fn new(dataset: DatasetType) -> Writer<DatasetType> {
        Writer {
            dataset: Dataset::new(dataset, 8 * 1024),
            state: WriteState::Preamble,
            bytes_written: 0,
            ts: &ts::ExplicitVRLittleEndian,
            cs: DEFAULT_CHARACTER_SET,
            current_path: Vec::new(),
            file_preamble: Some([0u8; FILE_PREAMBLE_LENGTH]),
        }
    }

    /// Get the number of bytes read from the dataset.
    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }

    /// Get the current state of the parser.
    pub fn get_write_state(&self) -> WriteState {
        self.state
    }

    /// Get the transfer syntax the dataset is encoded in.
    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    /// Get the character set string values are encoded in.
    pub fn get_cs(&self) -> CSRef {
        self.cs
    }

    pub fn write_preamble(&mut self) -> Result<usize> {
        let mut bytes_written: usize = 0;

        if let Some(preamble) = self.file_preamble {
            bytes_written += self.dataset.write(&preamble)?;
            bytes_written += self.dataset.write(DICOM_PREFIX)?;
        }

        Ok(bytes_written)
    }

    pub fn write_file_meta(&mut self) -> Result<usize> {
        let mut bytes_written: usize = 0;

        let fme_length: u32 = 0;
        let group_len_element: DicomElement = self.new_fme(
            tags::FILE_META_INFORMATION_GROUP_LENGTH, &vr::UL,
            RawValue::UnsignedIntegers(vec![fme_length]))?;

        bytes_written += self.write_element(&group_len_element)?;

        Ok(bytes_written)
    }

    fn new_fme(&self, tag: u32, vr: VRRef, value: RawValue) -> Result<DicomElement> {
        let mut element = DicomElement::new(tag, vr,
                          ValueLength::UndefinedLength,
                          &ts::ExplicitVRLittleEndian, DEFAULT_CHARACTER_SET,
                          Vec::with_capacity(0), Vec::with_capacity(0));

        element.encode_value(value)
            .map_err(|err| <ParseError as Into<WriteError>>::into(err))?;

        Ok(element)
    }

    pub fn write_element(&mut self, element: &DicomElement) -> Result<usize> {
        let mut bytes_written: usize = 0;

        bytes_written += self.write_tag(element)?;
        bytes_written += self.write_vr(element)?;
        bytes_written += self.write_vl(element)?;
        bytes_written += self.write_data(element)?;

        Ok(bytes_written)
    }

    fn write_tag(&mut self, element: &DicomElement) -> Result<usize> {
        let mut bytes_written: usize = 0;
        let element_number: u16 = (element.tag | 0x00FF) as u16;
        let group_number: u16 = ((element.tag >> 4) | 0x00FF) as u16;

        if element.get_ts().is_big_endian() {
            bytes_written += self.dataset.write(&group_number.to_be_bytes())?;
            bytes_written += self.dataset.write(&element_number.to_be_bytes())?;
        } else {
            bytes_written += self.dataset.write(&group_number.to_le_bytes())?;
            bytes_written += self.dataset.write(&element_number.to_le_bytes())?;
        }

        Ok(bytes_written)
    }

    fn write_vr(&mut self, element: &DicomElement) -> Result<usize> {
        let mut bytes_written: usize = 0;

        if element.get_ts().is_explicit_vr() {
            bytes_written += self.dataset.write(element.vr.ident.as_bytes())?;
        }

        Ok(bytes_written)
    }

    fn write_vl(&mut self, element: &DicomElement) -> Result<usize> {
        let mut bytes_written: usize = 0;

        let write_4bytes: bool =
            !element.get_ts().is_explicit_vr() || element.vr.has_explicit_2byte_pad;

        match element.vl {
            ValueLength::UndefinedLength => {
                if !write_4bytes {
                    return Err(WriteError::InvalidUndefinedValueLengthError);
                }

                if element.get_ts().is_big_endian() {
                    bytes_written += self.dataset.write(&UNDEFINED_LENGTH.to_be_bytes())?;
                } else {
                    bytes_written += self.dataset.write(&UNDEFINED_LENGTH.to_le_bytes())?;
                }
            }

            ValueLength::Explicit(length) => {
                if write_4bytes {
                    if element.get_ts().is_big_endian() {
                        bytes_written += self.dataset.write(&length.to_be_bytes())?;
                    } else {
                        bytes_written += self.dataset.write(&length.to_le_bytes())?;
                    }
                } else {
                    let length: u16 = (length | 0x00FF) as u16;

                    if element.get_ts().is_big_endian() {
                        bytes_written += self.dataset.write(&length.to_be_bytes())?;
                    } else {
                        bytes_written += self.dataset.write(&length.to_le_bytes())?;
                    }
                }
            }
        }

        Ok(bytes_written)
    }

    fn write_data(&mut self, element: &DicomElement) -> Result<usize> {
        let mut bytes_written: usize = 0;

        #[cfg(feature = "deflate")]
        self.dataset.set_write_deflated(element.get_ts().is_deflated());

        bytes_written += self.dataset.write(element.get_data().as_slice())?;
        Ok(bytes_written)
    }
}
