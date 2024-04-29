//! Errors that can occur during parsing of a DICOM dataset.

use crate::core::charset::{CSError, CSRef};
use crate::core::DICOM_PREFIX_LENGTH;
use crate::defn::vr::VRRef;
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur during parsing of a DICOM dataset.
pub enum ParseError {
    /// The DICOM dataset has an invalid DICOM prefix. Dataset is unlikely DICOM.
    #[error("dicom prefix is not 'DICM': {0:?}")]
    BadDICOMPrefix([u8; DICOM_PREFIX_LENGTH]),

    /// If an unknown VR is parsed from an ExplicitVR stream.
    #[error("unknown explicit vr: {0:#06X}")]
    UnknownExplicitVR(u16),

    /// This is used internally during parsing when the stream ends while trying to read from
    /// the dataset, but occurs during acceptable boundaries -- such as at the end/start of a
    /// dicom element.
    #[error("file/stream ended between dicom elements")]
    ExpectedEOF,

    /// Wraper around `crate::core::charset::CSError`.
    #[error("charset decoding error")]
    CharsetError {
        #[source]
        source: CSError,
    },

    /// Wrapper around `std::io::Error`.
    #[error("i/o error reading from dataset")]
    IOError {
        #[from]
        source: std::io::Error,
    },

    /// Wrapper around `std::io::Error` but includes additional details at the point of error.
    #[error("error reading from dataset: {source:?}\n\t{detail}")]
    DetailedError {
        #[source]
        source: Box<ParseError>,
        detail: String,
    },

    /// An error occurs while parsing the value of a DICOM element.
    #[error(
        "error parsing element value: {message}\n\ttagpath: {tagstring}\n\tvr:{vr:?}, cs: {cs:?}\n\tvalue: {bytes:?}"
    )]
    ValueParseError {
        message: String,
        tagstring: String,
        vr: VRRef,
        cs: CSRef,
        bytes: Vec<u8>,
    },

    /// An error occurs when converting RawValue to bytes.
    #[error("error converting RawValue to bytes: {message}")]
    RawValueConversionError { message: String },
}
