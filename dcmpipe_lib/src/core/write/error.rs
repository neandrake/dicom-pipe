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
