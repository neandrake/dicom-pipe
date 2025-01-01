/*
   Copyright 2024-2025 Christopher Speck

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

//! Specify behavior while parsing.

use crate::core::read::stop::ParseStop;

#[derive(Clone, Debug)]
pub struct ParseBehavior {
    /// Specifies when the parser should stop parsing the dataset.
    ///
    /// Refer to documentation on `ParseStop`.
    ///
    /// Default: `ParseStop::EndOfDataset`.
    stop: ParseStop,

    /// Specifies how the parser should handle an error it encounters in the dataset while
    /// populating a `DicomObject` with elements it parses.
    ///
    /// If `true`, a partially-populated `DicomObject` will be returned instead of a `Result::Err`.
    ///
    /// Default: `false`.
    allow_partial_object: bool,
}

impl ParseBehavior {
    /// Specifies when the parser should stop parsing the dataset.
    ///
    /// Refer to documentation on `ParseStop`.
    ///
    /// Default: `ParseStop::EndOfDataset`.
    #[must_use]
    pub fn stop(&self) -> &ParseStop {
        &self.stop
    }

    /// Specifies how the parser should handle an error it encounters in the dataset while
    /// populating a `DicomObject` with elements it parses.
    ///
    /// If `true`, a partially-populated `DicomObject` will be returned instead of a `Result::Err`.
    ///
    /// Default: `false`.
    #[must_use]
    pub fn allow_partial_object(&self) -> bool {
        self.allow_partial_object
    }

    /// Specify when the parser should stop parsing the dataset.
    ///
    /// Refer to documentation on `ParseStop`.
    pub fn set_stop(&mut self, stop: ParseStop) {
        self.stop = stop;
    }

    /// Specify how the parser should handle an error in encounters in the dataset while populating
    /// a `DicomObject` with elements it parses.
    ///
    /// If `true`, a partially-populated `DicomObject` will be returned instead of a `Result::Err`.
    pub fn set_allow_partial_object(&mut self, allow_partial_object: bool) {
        self.allow_partial_object = allow_partial_object;
    }
}

impl Default for ParseBehavior {
    fn default() -> Self {
        Self {
            stop: ParseStop::EndOfDataset,
            allow_partial_object: false,
        }
    }
}
