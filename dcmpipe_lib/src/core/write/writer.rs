use std::io::Write;

use crate::core::charset::{self, CSRef};
use crate::core::dcmelement::DicomElement;
use crate::core::dcmsqelem::SequenceElement;
use crate::core::write::ds::dataset::Dataset;
use crate::core::write::error::WriteError;
use crate::core::{DICOM_PREFIX, FILE_PREAMBLE_LENGTH};
use crate::defn::constants::{tags, ts};
use crate::defn::tag::Tag;
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

    pub fn write_element(&mut self, element: &DicomElement) -> Result<usize> {
        let mut bytes_written: usize = 0;

        bytes_written += self.write_tag(element.tag)?;
        bytes_written += self.write_vr(element.vr)?;
        bytes_written += self.write_vl(element.vl, element.vr)?;
        bytes_written += self.dataset.write(element.get_data().as_slice())?;

        Ok(bytes_written)
    }

    fn write_tag(&mut self, tag: u32) -> Result<usize> {
        let mut bytes_written: usize = 0;
        let element_number: u16 = (tag | 0x00FF) as u16;
        let group_number: u16 = ((tag >> 4) | 0x00FF) as u16;

        if self.ts.is_big_endian() {
            bytes_written += self.dataset.write(&group_number.to_be_bytes())?;
            bytes_written += self.dataset.write(&element_number.to_be_bytes())?;
        } else {
            bytes_written += self.dataset.write(&group_number.to_le_bytes())?;
            bytes_written += self.dataset.write(&element_number.to_le_bytes())?;
        }

        Ok(bytes_written)
    }

    fn write_vr(&mut self, vr: VRRef) -> Result<usize> {
        let mut bytes_written: usize = 0;

        if self.ts.is_explicit_vr() {
            bytes_written += self.dataset.write(&vr.ident.as_bytes())?;
        }

        Ok(bytes_written)
    }

    fn write_vl(&mut self, vl: ValueLength, vr: VRRef) -> Result<usize> {
        let mut bytes_written: usize = 0;

        let write_4bytes: bool = !self.ts.explicit_vr || vr.has_explicit_2byte_pad;

        match vl {
            ValueLength::UndefinedLength => {
                if !write_4bytes {
                    return Err(WriteError::InvalidUndefinedValueLengthError);
                }

                if self.ts.is_big_endian() {
                    bytes_written += self.dataset.write(&UNDEFINED_LENGTH.to_be_bytes())?;
                } else {
                    bytes_written += self.dataset.write(&UNDEFINED_LENGTH.to_le_bytes())?;
                }
            },

            ValueLength::Explicit(length) => {
                if write_4bytes {
                    if self.ts.is_big_endian() {
                        bytes_written += self.dataset.write(&length.to_be_bytes())?;
                    } else {
                        bytes_written += self.dataset.write(&length.to_le_bytes())?;
                    }
                } else {
                    let length: u16 = (length | 0x00FF) as u16;

                    if self.ts.is_big_endian() {
                        bytes_written += self.dataset.write(&length.to_be_bytes())?;
                    } else {
                        bytes_written += self.dataset.write(&length.to_le_bytes())?;
                    }
                }
            }
        }

        Ok(bytes_written)
    }
}
