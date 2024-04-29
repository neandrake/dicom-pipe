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

use std::{fmt::Display, io::Write};

use crate::{
    core::{charset::CSError, read::ParseError, write::error::WriteError},
    dimse::pdus::{
        mainpdus::{Abort, AssocRJ},
        PduType,
    },
};

/// Errors related to the DIMSE protocol.
#[derive(Debug, thiserror::Error)]
pub enum DimseError {
    /// A PDU type was encountered that is unknown, likely non-standard or a corruption in the
    /// straem.
    #[error("invalid pdu type: {0:04X}")]
    InvalidPduType(u8),

    /// An AE Title which is not correctly formatted.
    #[error("invalid ae title: {0:?}")]
    InvalidAeTitle(Vec<u8>),

    /// The stream closed unexpectedly.
    #[error("unexpected end of byte stream")]
    UnexpectedEOF,

    /// DIMSE Command Messages are expected to have a minimal set of elements.
    #[error("element missing from request: {0}")]
    DimseElementMissing(String),

    /// A Pdu was encountered which was not anticipated.
    #[error("unexpected pdu type {0:?}")]
    UnexpectedPduType(PduType),

    /// Error while parsing a DICOM element in a DIMSE request/response.
    #[error("error parsing value from request")]
    ParseError(#[from] ParseError),

    /// Character encoding errors while parsing DICOM element values in a DIMSE request/response.
    #[error("error decoding string")]
    CharsetError(#[from] CSError),

    /// Errors when writing DICOM elements in a response request/response stream.
    #[error("error encoding DICOM")]
    WriteError(#[from] WriteError),

    /// Wrapper around `std::io::Error`.
    #[error("i/o error reading from dataset")]
    IOError(#[from] std::io::Error),

    /// Catch-all for error-states while interpreting the DIMSE stream.
    #[error("{0}")]
    GeneralError(String),

    /// Wrapper around other errors.
    #[error("error happened: {0}")]
    OtherError(Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub enum AssocRsp {
    RJ(AssocRJ),
    AB(Abort),
}

impl AssocRsp {
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        match self {
            AssocRsp::RJ(rj) => rj.pdu_type(),
            AssocRsp::AB(ab) => ab.pdu_type(),
        }
    }
}

/// Errors related to the association, which can aid in managing sending back appropriate rejection
/// or abort PDUs if needed.
#[derive(Debug)]
pub struct AssocError {
    rsp: Option<AssocRsp>,
    err: DimseError,
}

impl Display for AssocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl AssocError {
    #[must_use]
    pub fn rsp(&self) -> &Option<AssocRsp> {
        &self.rsp
    }

    #[must_use]
    pub fn into_err(self) -> DimseError {
        self.err
    }

    /// Generates a `AssocError`, wrapping the given error in `DimseError`. This error response
    /// will not result in sending an abort or rejection to the other end.
    #[must_use]
    pub fn error<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: None,
            err: DimseError::from(err),
        }
    }

    /// Generates a generic `Abort`, for closing the connection without further context or
    /// information.
    #[must_use]
    pub fn ab_failure<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::AB(Abort::new(0u8, 0u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates a `Abort`, indicating that an unexpected PDU was encountered.
    #[must_use]
    pub fn ab_unexpected_pdu<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::AB(Abort::new(2u8, 2u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates a `Abort`, indicating that the PDU is not valid or well-formed.
    #[must_use]
    pub fn ab_invalid_pdu<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::AB(Abort::new(2u8, 6u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates a generic `AssocRJ`, rejecting the association without further context or
    /// information.
    #[must_use]
    pub fn rj_failure<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 1u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates an `AssocRJ` indicating the calling AE Title has been rejected.
    #[must_use]
    pub fn rj_calling_aet<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 3u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates an `AssocRJ` indicating the called AE Title has been rejected.
    #[must_use]
    pub fn rj_called_aet<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 7u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates an `AssocRJ` indicating the Application Context Name isn't supported. This has
    /// limited use, for the initial association negotiation.
    #[must_use]
    pub fn rj_unsupported<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 2u8))),
            err: DimseError::from(err),
        }
    }

    /// Writes this error response, if any, to the given writer, consuming this error.
    ///
    /// # Errors
    /// I/O errors may occur writing the PDU to the writer, or flushing the writer.
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), DimseError> {
        match &self.rsp {
            Some(AssocRsp::RJ(rj)) => {
                rj.write(&mut writer)?;
                writer.flush().map_err(DimseError::IOError)?;
            }
            Some(AssocRsp::AB(ab)) => {
                ab.write(&mut writer)?;
                writer.flush().map_err(DimseError::IOError)?;
            }
            _ => {}
        }
        Ok(())
    }
}
