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

//! Errors that can occur during parsing of a DICOM dataset.

use crate::core::{
    charset::{CSError, CSRef},
    dcmelement::DicomElement,
    defn::{dcmdict::DicomDictionary, tag::TagPath, vr::VRRef},
    DICOM_PREFIX_LENGTH,
};

use thiserror::Error;

const MAX_BYTES_IN_ERROR: usize = 16;

/// A simple structure for collecting information about parsing values from a `DicomElement`, which
/// can be easily converted to a `ParseError`.
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
        let dict = value.2;

        let tagstring = TagPath::format_tagpath_to_display(&elem.create_tagpath(), dict);
        ParseError::DecodeValueError {
            message: message.to_owned(),
            tagstring,
            vr: elem.vr(),
            cs: elem.cs(),
            bytes: elem
                .data()
                .iter()
                .take(MAX_BYTES_IN_ERROR)
                .copied()
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

    /// If an unknown VR is parsed from an `ExplicitVR` stream.
    #[error("unknown explicit vr: {0:#06X}")]
    UnknownExplicitVR(u16),

    /// This is used internally during parsing when the stream ends while trying to read from
    /// the dataset, but occurs during acceptable boundaries -- such as at the end/start of a
    /// dicom element.
    #[error("file/stream ended between dicom elements")]
    ExpectedEOF,

    /// Wrapper around `crate::core::charset::CSError`.
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
    #[error("unable to resolve tagpath, {string_path}: {details}")]
    InvalidTagPath {
        string_path: String,
        details: String,
    },

    /// A general decode error that may not be directly tied to a DICOM element.
    #[error("error decoding: {0}")]
    GeneralDecodeError(String),
}
