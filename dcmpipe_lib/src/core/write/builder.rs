/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::io::Write;

use crate::core::{
    charset::{CSRef, DEFAULT_CHARACTER_SET},
    defn::{constants::ts, ts::TSRef},
    FILE_PREAMBLE_LENGTH,
};

use super::{
    ds::dataset::Dataset,
    writer::{Writer, WriterState},
};

/// A builder for constructing a `Writer`.
#[derive(Debug, Default)]
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
    /// Initialize a `WriterBuilder` destined to a file, using default file preamble.
    #[must_use]
    pub fn for_file() -> Self {
        Self::default()
            .state(WriterState::WritePreamble)
            .file_preamble([0u8; FILE_PREAMBLE_LENGTH])
    }

    /// Sets the initial `WriterState` indicating how to start writing the dataset.
    #[must_use]
    pub fn state(mut self, state: WriterState) -> Self {
        self.state = Some(state);
        self
    }

    /// Sets the transfer syntax to use for writing the dataset.
    #[must_use]
    pub fn ts(mut self, ts: TSRef) -> Self {
        self.ts = Some(ts);
        self
    }

    /// Sets the character set to use for encoding string values to the dataset.
    #[must_use]
    pub fn cs(mut self, cs: CSRef) -> Self {
        self.cs = Some(cs);
        self
    }

    /// Sets the file preamble to use if writing the dataset to a file.
    #[must_use]
    pub fn file_preamble(mut self, file_preamble: [u8; FILE_PREAMBLE_LENGTH]) -> Self {
        self.file_preamble = Some(file_preamble);
        self
    }

    /// Constructs a `Writer` from this builder.
    pub fn build<DatasetType: Write>(&self, dataset: DatasetType) -> Writer<DatasetType> {
        Writer {
            dataset: Dataset::new(dataset),
            state: self.state.unwrap_or(WriterState::WritePreamble),
            bytes_written: 0,
            ts: self.ts.unwrap_or(&ts::ExplicitVRLittleEndian),
            cs: self.cs.unwrap_or(DEFAULT_CHARACTER_SET),
            file_preamble: self.file_preamble,
        }
    }
}
