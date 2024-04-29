use std::io::Write;

use crate::{
    core::charset::CSRef,
    core::{charset::DEFAULT_CHARACTER_SET, FILE_PREAMBLE_LENGTH},
    defn::{constants::ts, ts::TSRef},
};

use super::{
    ds::dataset::Dataset,
    writer::{Writer, WriterState},
};

/// A builder for constructing a `Writer`.
pub struct WriterBuilder {
    /// Initial writer state. Defaults to `WriterState::Preamble`.
    state: Option<WriterState>,

    ts: Option<TSRef>,

    cs: Option<CSRef>,

    /// The file preamble to write to the dataset. Not all datasets may have a preamble.
    /// If a file preamble is specified then the `"DICM"` prefix will be written immediately
    /// after the file preamble is written.
    file_preamble: Option<[u8; FILE_PREAMBLE_LENGTH]>,
}

impl WriterBuilder {
    pub fn new() -> Self {
        Self {
            state: None,
            ts: None,
            cs: None,
            file_preamble: None,
        }
    }

    /// Initialize a `WriterBuilder` destined to a file, using default file preamble.
    pub fn for_file() -> Self {
        Self {
            state: Some(WriterState::Preamble),
            ts: None,
            cs: None,
            file_preamble: Some([0u8; FILE_PREAMBLE_LENGTH]),
        }
    }

    pub fn state(mut self, state: WriterState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn ts(mut self, ts: TSRef) -> Self {
        self.ts = Some(ts);
        self
    }

    pub fn cs(mut self, cs: CSRef) -> Self {
        self.cs = Some(cs);
        self
    }

    pub fn file_preamble(mut self, file_preamble: [u8; FILE_PREAMBLE_LENGTH]) -> Self {
        self.file_preamble = Some(file_preamble);
        self
    }

    /// Constructs a `Writer` from this builder.
    pub fn build<DatasetType: Write>(&self, dataset: DatasetType) -> Writer<DatasetType> {
        Writer {
            dataset: Dataset::new(dataset, 8 * 1024),
            state: self.state.unwrap_or(WriterState::Preamble),
            bytes_written: 0,
            ts: self.ts.unwrap_or(&ts::ExplicitVRLittleEndian),
            cs: self.cs.unwrap_or(DEFAULT_CHARACTER_SET),
            file_preamble: self.file_preamble,
        }
    }
}
