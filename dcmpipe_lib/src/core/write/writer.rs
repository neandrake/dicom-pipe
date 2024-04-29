use std::io::Write;

use crate::core::charset::{self, CSRef};
use crate::core::dcmelement::DicomElement;
use crate::core::dcmsqelem::SequenceElement;
use crate::core::write::ds::dataset::Dataset;
use crate::core::write::error::WriteError;
use crate::core::{DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH};
use crate::defn::constants::{tags, ts};
use crate::defn::tag::Tag;
use crate::defn::ts::TSRef;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef};

pub type Result<T> = core::result::Result<T, WriteError>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WriteState {
    Preamble,
    Prefix,
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
}
