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

#[derive(Debug, thiserror::Error)]
pub enum DimseError {
    #[error("invalid pdu type: {0:04X}")]
    InvalidPduType(u8),

    #[error("invalid ae title: {0:?}")]
    InvalidAeTitle(Vec<u8>),

    #[error("unexpected end of byte stream")]
    UnexpectedEOF,

    #[error("element missing from request: {0}")]
    ElementMissingFromRequest(String),

    #[error("invalid pdu parse state: {0}")]
    InvalidPduParseState(String),

    #[error("unexpected pdu type {0:?}")]
    UnexpectedPduType(PduType),

    #[error("error parsing value from request")]
    ParseError(#[from] ParseError),

    #[error("error decoding string")]
    CharsetError(#[from] CSError),

    #[error("error encoding DICOM")]
    WriteError(#[from] WriteError),

    /// Wrapper around `std::io::Error`.
    #[error("i/o error reading from dataset")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    GeneralError(String),
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

    #[must_use]
    pub fn rj_failure<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(2u8, 1u8, 1u8))),
            err: DimseError::from(err),
        }
    }

    #[must_use]
    pub fn rj_calling_aet<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(2u8, 1u8, 3u8))),
            err: DimseError::from(err),
        }
    }

    #[must_use]
    pub fn rj_called_aet<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(2u8, 1u8, 7u8))),
            err: DimseError::from(err),
        }
    }

    #[must_use]
    pub fn rj_unsupported<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        AssocError {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(2u8, 1u8, 2u8))),
            err: DimseError::from(err),
        }
    }

    /// Writes this error response, if any, to the given writer, consuming this error.
    ///
    /// # Errors
    /// I/O errors may occur writing the PDU to the writer, or flushing the writer.
    pub fn write<W: Write>(self, mut writer: W) -> Result<(), DimseError> {
        match self.rsp {
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
        Err(self.err)
    }
}
