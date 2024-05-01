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

#![allow(clippy::module_name_repetitions)]

use crate::{core::charset::CSRef, dimse::error::DimseError};

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

impl TryFrom<&str> for AeTitle {
    type Error = DimseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AeTitle::try_from(value.as_bytes())
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
        CSRef::default()
            .decode(value.0)
            .map(|s| s.trim_end_matches('\0').to_owned())
            .map_err(DimseError::CharsetError)
    }
}
