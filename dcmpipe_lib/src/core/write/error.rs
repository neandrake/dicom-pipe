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

use thiserror::Error;

use crate::core::read::ParseError;

#[derive(Error, Debug)]
/// Errors that can occur during writing of a DICOM dataset.
pub enum WriteError {
    #[error("failed encoding element value")]
    EncodeValueError(#[from] ParseError),

    #[error("value length of undefined cannot be used with implicit VR")]
    InvalidValueLength,

    /// Wrapper around `std::io::Error`.
    #[error("i/o error writing to stream")]
    IOError {
        #[from]
        source: std::io::Error,
    },

    /// Wrapper around `std::io::Error` but includes additional details at the point of error.
    #[error("i/o error reading from stream: {detail}")]
    DetailedIOError {
        #[source]
        source: std::io::Error,
        detail: String,
    },
}
