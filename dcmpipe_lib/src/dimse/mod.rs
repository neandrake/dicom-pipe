#![allow(clippy::module_name_repetitions)]

use crate::{core::charset::DEFAULT_CHARACTER_SET, dimse::error::DimseError};

pub mod assoc;
pub mod commands;
pub mod error;
pub mod pdus;

pub struct AeTitle(pub [u8; 16]);

impl From<[u8; 16]> for AeTitle {
    fn from(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for AeTitle {
    type Error = DimseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 16 {
            return Err(DimseError::InvalidAeTitle(value.into()));
        }
        // Create vec of size 16, copy the input, convert to an array, then into AeTitle
        let mut filled_ae: Vec<u8> = vec![b' '; 16];
        let () = &mut filled_ae[0..value.len()].copy_from_slice(value);
        TryInto::<[u8; 16]>::try_into(filled_ae)
            .map(AeTitle::from)
            .map_err(DimseError::InvalidAeTitle)
    }
}

impl From<AeTitle> for [u8; 16] {
    fn from(value: AeTitle) -> Self {
        value.0
    }
}

pub struct Syntax<'s>(pub &'s [u8]);

impl<'s> TryFrom<&'s Syntax<'s>> for String {
    type Error = DimseError;

    fn try_from(value: &'s Syntax) -> Result<Self, Self::Error> {
        DEFAULT_CHARACTER_SET
            .decode(value.0)
            .map(|s| s.trim_end_matches('\0').to_owned())
            .map_err(DimseError::CharsetError)
    }
}
