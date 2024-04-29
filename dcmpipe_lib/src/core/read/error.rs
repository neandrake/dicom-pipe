//! Errors that can occur during parsing of a DICOM dataset.

use crate::core::{
    charset::{CSError, CSRef},
    dcmelement::DicomElement,
    defn::{dcmdict::DicomDictionary, tag::TagPath, vr::VRRef},
    DICOM_PREFIX_LENGTH,
};

use thiserror::Error;

const MAX_BYTES_IN_ERROR: usize = 16;

#[derive(Debug)]
pub(crate) struct ParseErrorInfo<'a>(
    pub &'a DicomElement,
    pub &'a str,
    pub Option<&'a dyn DicomDictionary>,
);
impl<'a> From<ParseErrorInfo<'a>> for ParseError {
    fn from(value: ParseErrorInfo<'a>) -> Self {
        let elem = value.0;
        let message = value.1;

        // TODO: How to get a dicom dictionary here for better error messages?
        let tagstring = TagPath::format_tagpath_to_display(&elem.create_tagpath(), None);
        ParseError::DecodeValueError {
            message: message.to_owned(),
            tagstring,
            vr: elem.vr(),
            cs: elem.cs(),
            bytes: elem
                .data()
                .iter()
                .take(MAX_BYTES_IN_ERROR)
                .cloned()
                .collect::<Vec<u8>>(),
        }
    }
}

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

    /// Wraps another `Error` and includes additional details from the state of the parser.
    ///
    /// This should never be used within the parser internals. It should only be used by the
    /// primary iteration done by the `parser::iter` module.
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
    DecodeValueError {
        message: String,
        tagstring: String,
        vr: VRRef,
        cs: CSRef,
        bytes: Vec<u8>,
    },

    /// An error when a text/string representation of a tagpath is unable to be parsed/resolved.
    #[error("unable to resolve tagpath: {string_path}")]
    InvalidTagPath { string_path: String },
}
