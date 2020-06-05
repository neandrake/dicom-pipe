use thiserror::Error;
use crate::defn::vr::{VRRef};
use crate::defn::tag::Tag;
use super::parser::DICOM_PREFIX_LENGTH;


#[derive(Error, Debug)]
/// Errors that can occur during parsing of a DICOM dataset.
pub enum ParseError {
    #[error("dicom prefix is not 'DICM': {0:?}")]
    /// The DICOM dataset has an invalid DICOM prefix. Dataset is unlikely DICOM.
    BadDICOMPrefix([u8; DICOM_PREFIX_LENGTH]),

    /// If an unknown VR is parsed from an ExplicitVR stream.
    #[error("unknown explicit vr: {0:#06X}")]
    UnknownExplicitVR(u16),

    #[error("file/stream ended between dicom elements")]
    /// This is used internally during parsing when the stream ends while trying to read from
    /// the dataset, but occurs during acceptable boundaries -- such as at the end/start of a
    /// dicom element.
    ExpectedEOF,

    #[error("i/o error reading from stream")]
    /// Wrapper around `std::io::Error`.
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("i/o error reading from stream: {detail}")]
    /// Wrapper around `std::io::Error` but includes additional details at the point of error.
    DetailedIOError {
        #[source]
        source: std::io::Error,
        detail: String,
    },

    #[error("error parsing element value: {} [{vr:?}], {message} {bytes:?}", Tag::format_tag_to_display(* tag))]
    ValueParseError {
        message: String,
        tag: u32,
        vr: VRRef,
        bytes: Vec<u8>,
    },
}