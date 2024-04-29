use crate::dimse::error::DimseError;

pub mod commands;
pub mod error;
pub mod pduiter;
pub mod pdus;

pub struct AeTitle([u8; 16]);

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
        let _ = &mut filled_ae[0..value.len()].copy_from_slice(value);
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
