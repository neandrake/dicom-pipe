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

use crate::{
    core::{
        dcmobject::DicomRoot,
        write::{builder::WriterBuilder, writer::WriterState},
    },
    dict::transfer_syntaxes::ImplicitVRLittleEndian,
    dimse::{
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
        pdus::mainpdus::{Abort, PresentationDataValue},
    },
};

pub mod scp;
pub mod scu;

#[derive(Debug)]
pub enum DimseMsg {
    Cmd(CommandMessage),
    Dataset(PresentationDataValue),
    ReleaseRQ,
    Abort(Abort),
}

/// Serialize the given `DicomRoot` into in-memory bytes, using IVRLE.
///
/// # Errors
/// An I/O error may occur when writing to the in-memory `Vec`.
pub fn serialize(dicom: &DicomRoot) -> Result<Vec<u8>, AssocError> {
    let mut ds_writer = WriterBuilder::default()
        .state(WriterState::Element)
        .ts(&ImplicitVRLittleEndian)
        .build(Vec::<u8>::new());
    ds_writer
        .write_dcmroot(dicom)
        .map_err(|e| AssocError::ab_invalid_pdu(DimseError::WriteError(e)))?;
    Ok(ds_writer.into_dataset())
}
