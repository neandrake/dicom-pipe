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

//! Protocol Data Units found in `UserInformationItem`, for DIMSE. Part 8, Chapter 9.3
//!
//! PDU headers are encoded with Big Endian. The value fields are sent using the transfer syntax
//! negotiated during establishment of the association.
//!
//! These are PDUs which appear in the `UserInformationItem` PDU. These are split out and handled
//! separate from other PDUs to avoid recursion during read/write, which, due to using generics
//! around `Read` and `Write` makes handling of the type references difficult and encounter
//! infinite recursion of type resolution.

use std::{
    io::{Read, Write},
    mem::size_of,
};

use crate::dimse::{pdus::UserPduType, DimseError};

/// This is the current DICOM standard defined version for Common Extended Negotiation.
/// See Part 7, Annex D.3.3.6.1
pub(crate) static SOP_CLASS_COMMON_EXTENDED_NEGOTIATION_VERSION: u8 = 0b0000_0000;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaxLengthItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    max_length: u32,
}

impl MaxLengthItem {
    /// The type of this PDU, `UserSubPduType::MaxLengthItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::MaxLengthItem
    }

    /// Create a new `MaxLengthItem`.
    #[must_use]
    pub fn new(max_length: u32) -> Self {
        let length: u16 = size_of::<u32>().try_into().unwrap_or_default(); // max_length

        Self {
            reserved: 0u8,
            length,
            max_length,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the user
    /// data fields. This should be a fixed value of 4.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The maximum length.
    #[must_use]
    pub fn max_length(&self) -> u32 {
        self.max_length
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u32>() // max_length
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.max_length.to_be_bytes())?;

        Ok(())
    }

    /// Read a `MaxLengthItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let max_length = u32::from_be_bytes(buf);

        Ok(Self {
            reserved,
            length,
            max_length,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImplementationClassUIDItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    impl_class_uid: Vec<u8>,
}

impl ImplementationClassUIDItem {
    /// The type of this PDU, `UserSubPduType::ImplementationClassUIDItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::ImplementationClassUIDItem
    }

    /// Create a new `ImplementationClassUIDItem`.
    #[must_use]
    pub fn new(impl_class_uid: Vec<u8>) -> Self {
        Self {
            reserved: 0u8,
            length: impl_class_uid.len().try_into().unwrap_or_default(),
            impl_class_uid,
        }
    }

    /// The number of bytes in the following Implementation Class UID field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The Implementation Class UID.
    #[must_use]
    pub fn impl_class_uid(&self) -> &Vec<u8> {
        &self.impl_class_uid
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.impl_class_uid.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.impl_class_uid)?;

        Ok(())
    }

    /// Read a `ImplementationClassUIDItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut impl_class_uid: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut impl_class_uid)?;

        Ok(Self {
            reserved,
            length,
            impl_class_uid,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AsyncOperationsWindowItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    max_ops_invoked: u16,
    max_ops_performed: u16,
}

impl AsyncOperationsWindowItem {
    /// The type of this PDU, `UserSubPduType::AsyncOperationsWindowItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::AsyncOperationsWindowItem
    }

    /// Create a new `AsyncOperationsWindowItem`.
    #[must_use]
    pub fn new(max_ops_invoked: u16, max_ops_performed: u16) -> Self {
        let length: usize = size_of::<u16>() // max_ops_invoked
            + size_of::<u16>(); // max_ops_performed

        Self {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            max_ops_invoked,
            max_ops_performed,
        }
    }

    /// Convenience for requesting/responding that only one message may be handled at a time within
    /// an association.
    #[must_use]
    pub fn synchronous() -> Self {
        Self::new(1, 1)
    }

    /// Convenience for requesting/responding that an unlimited number of messages may be performed
    /// simultaneously.
    #[must_use]
    pub fn unlimited() -> Self {
        Self::new(0, 0)
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Maximum Number Operations Performed field. This should be a fixed value of 4.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Maximum Number of Operations Invoked.
    #[must_use]
    pub fn max_ops_invoked(&self) -> u16 {
        self.max_ops_invoked
    }

    /// Maximum Number of Operations Performed.
    #[must_use]
    pub fn max_ops_performed(&self) -> u16 {
        self.max_ops_performed
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // max_ops_invoked
            + size_of::<u16>() // max_ops_performed
    }

    /// Convenience for checking if this `AsyncOperationsWindowItem` indicates that all message
    /// passing must be synchronous.
    #[must_use]
    pub fn is_synchronous(&self) -> bool {
        self.max_ops_invoked == 1 && self.max_ops_performed == 1
    }

    /// Convenience for checking if this `AsyncOperationsWindowItem` indicates that an unliminted
    /// number of messages are allowed asynchronously.
    #[must_use]
    pub fn is_unlimited(&self) -> bool {
        self.max_ops_invoked == 0 && self.max_ops_performed == 0
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.max_ops_invoked.to_be_bytes())?;
        dataset.write_all(&self.max_ops_performed.to_be_bytes())?;

        Ok(())
    }

    /// Read a `AsyncOperationsWindowItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let max_ops_invoked = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let max_ops_performed = u16::from_be_bytes(buf);

        Ok(Self {
            reserved,
            length,
            max_ops_invoked,
            max_ops_performed,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoleSelectionItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    sop_class_uid_length: u16,
    sop_class_uid: Vec<u8>,
    sc_user_role: u8,
    sc_provider_role: u8,
}

impl RoleSelectionItem {
    /// The type of this PDU, `UserSubPduType::RoleSelectionItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::RoleSelectionItem
    }

    /// Create a new `RoleSelectionItem`.
    ///
    /// # Notes
    /// `sc_user_role`, `sc_provider_role`:
    /// - 0: non-support of the SCU role.
    /// - 1: support of the SCU role.
    #[must_use]
    pub fn new(sop_class_uid: Vec<u8>, sc_user_role: u8, sc_provider_role: u8) -> Self {
        let length: usize = size_of::<u16>() // sop_class_uid_length
            + sop_class_uid.len()
            + size_of::<u8>() // sc_user_role
            + size_of::<u8>(); // sc_provider_role

        Self {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            sop_class_uid_length: sop_class_uid.len().try_into().unwrap_or_default(),
            sop_class_uid,
            sc_user_role,
            sc_provider_role,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the SCP
    /// Role field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The number of bytes in the SOP Class UID field.
    #[must_use]
    pub fn sop_class_uid_length(&self) -> u16 {
        self.sop_class_uid_length
    }

    /// The SOP Class UID field.
    #[must_use]
    pub fn sop_class_uid(&self) -> &Vec<u8> {
        &self.sop_class_uid
    }

    /// Support for the SCU role.
    ///
    /// # Notes
    /// - 0: non-support of the SCU role.
    /// - 1: support of the SCU role.
    #[must_use]
    pub fn sc_user_role(&self) -> u8 {
        self.sc_user_role
    }

    /// Support for the SCP role.
    ///
    /// # Notes
    /// - 0: non-support of the SCP role.
    /// - 1: support of the SCP role.
    #[must_use]
    pub fn sc_provider_role(&self) -> u8 {
        self.sc_provider_role
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // sop_class_uid_length
            + self.sop_class_uid.len()
            + size_of::<u8>() // sc_user_role
            + size_of::<u8>() // sc_provider_role
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let mut buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;

        buf[0] = self.sc_user_role;
        buf[1] = self.sc_provider_role;
        dataset.write_all(&buf)?;

        Ok(())
    }

    /// Read a `RoleSelectionItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let sop_class_uid_length = u16::from_be_bytes(buf);
        let mut sop_class_uid: Vec<u8> = vec![0u8; sop_class_uid_length.into()];
        dataset.read_exact(&mut sop_class_uid)?;

        dataset.read_exact(&mut buf)?;
        let sc_user_role = buf[0];
        let sc_provider_role = buf[1];

        Ok(Self {
            reserved,
            length,
            sop_class_uid_length,
            sop_class_uid,
            sc_user_role,
            sc_provider_role,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImplementationVersionNameItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    impl_ver_name: Vec<u8>,
}

impl ImplementationVersionNameItem {
    /// The type of this PDU, `UserSubPduType::ImplementationVersionNameItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::ImplementationVersionNameItem
    }

    /// Create a new `ImplementationVersionNameItem`.
    #[must_use]
    pub fn new(impl_ver_name: Vec<u8>) -> Self {
        Self {
            reserved: 0u8,
            length: impl_ver_name.len().try_into().unwrap_or_default(),
            impl_ver_name,
        }
    }

    /// The number of bytes in the Implementation Version Name field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Implementation Version Name.
    #[must_use]
    pub fn impl_ver_name(&self) -> &Vec<u8> {
        &self.impl_ver_name
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.impl_ver_name.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.impl_ver_name)?;

        Ok(())
    }

    /// Read a `ImplementationVersionNameItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut impl_ver_name: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut impl_ver_name)?;

        Ok(Self {
            reserved,
            length,
            impl_ver_name,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SOPClassExtendedNegotiationItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    sop_class_uid_length: u16,
    sop_class_uid: Vec<u8>,
    service_class_app_info: Vec<u8>,
}

impl SOPClassExtendedNegotiationItem {
    /// The type of this PDU, `UserSubPduType::SOPClassExtendedNegotiationItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::SOPClassExtendedNegotiationItem
    }

    /// Create a new `SOPClassExtendedNegotiationItem`.
    #[must_use]
    pub fn new(sop_class_uid: Vec<u8>, service_class_app_info: Vec<u8>) -> Self {
        let length: usize = size_of::<u16>() // sop_class_uid_length
            + sop_class_uid.len()
            + service_class_app_info.len();

        Self {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            sop_class_uid_length: sop_class_uid.len().try_into().unwrap_or_default(),
            sop_class_uid,
            service_class_app_info,
        }
    }

    /// The number of bytes from the first byte of the following field the last byte of the Service
    /// Class Application Information field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The number of bytes in the SOP Class UID field.
    #[must_use]
    pub fn sop_class_length(&self) -> u16 {
        self.sop_class_uid_length
    }

    /// The SOP Class UID field.
    #[must_use]
    pub fn sop_class_uid(&self) -> &Vec<u8> {
        &self.sop_class_uid
    }

    /// The Service Class Application Info field.
    #[must_use]
    pub fn service_class_app_info(&self) -> &Vec<u8> {
        &self.service_class_app_info
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // sop_class_uid_length
            + self.sop_class_uid.len()
            + self.service_class_app_info.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;
        dataset.write_all(&self.service_class_app_info)?;

        Ok(())
    }

    /// Read a `SOPClassExtendedNegotiationItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let sop_class_uid_length = u16::from_be_bytes(buf);
        let mut sop_class_uid: Vec<u8> = vec![0u8; sop_class_uid_length.into()];
        dataset.read_exact(&mut sop_class_uid)?;

        let length_field_bytesize: u16 = size_of::<u16>().try_into().unwrap_or_default();
        let service_class_app_info_length = length - length_field_bytesize - sop_class_uid_length;
        let mut service_class_app_info: Vec<u8> = vec![0u8; service_class_app_info_length.into()];
        dataset.read_exact(&mut service_class_app_info)?;

        Ok(Self {
            reserved,
            length,
            sop_class_uid_length,
            sop_class_uid,
            service_class_app_info,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SOPClassCommonExtendedNegotiationItem {
    version: u8,
    length: u16,
    sop_class_uid_length: u16,
    sop_class_uid: Vec<u8>,
    service_class_uid_length: u16,
    service_class_uid: Vec<u8>,
    rel_gen_sop_classes_length: u16,
    rel_gen_sop_classes: Vec<RelatedGeneralSOPClassUID>,
    reserved: Vec<u8>,
}

impl SOPClassCommonExtendedNegotiationItem {
    /// The type of this PDU, `UserSubPduType::SOPClassCommonExtendedNegotiationItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::SOPClassCommonExtendedNegotiationItem
    }

    /// Create a new `SOPClassCommonExtendedNegotiationItem`.
    #[must_use]
    pub fn new(
        sop_class_uid: Vec<u8>,
        service_class_uid: Vec<u8>,
        rel_gen_sop_classes: Vec<RelatedGeneralSOPClassUID>,
    ) -> Self {
        let rel_gen_sop_classes_length: usize = rel_gen_sop_classes
            .iter()
            .map(RelatedGeneralSOPClassUID::byte_size)
            .sum::<usize>();

        let length: usize = size_of::<u16>() // sop_class_uid_length
            + sop_class_uid.len()
            + size_of::<u16>() // service_class_uid_length
            + service_class_uid.len()
            + size_of::<u16>() // rel_gen_sop_classes_length
            + rel_gen_sop_classes_length;

        // zero-length for version 0 of this sub-item definition
        let reserved: Vec<u8> = Vec::with_capacity(0);

        Self {
            version: SOP_CLASS_COMMON_EXTENDED_NEGOTIATION_VERSION,
            length: length.try_into().unwrap_or_default(),
            sop_class_uid_length: sop_class_uid.len().try_into().unwrap_or_default(),
            sop_class_uid,
            service_class_uid_length: service_class_uid.len().try_into().unwrap_or_default(),
            service_class_uid,
            rel_gen_sop_classes_length: rel_gen_sop_classes_length.try_into().unwrap_or_default(),
            rel_gen_sop_classes,
            reserved,
        }
    }

    /// The version of this item. The current standard version is 0.
    #[must_use]
    pub fn version(&self) -> u8 {
        self.version
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reserved field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The number of bytes in the SOP Class UID field.
    #[must_use]
    pub fn sop_class_length(&self) -> u16 {
        self.sop_class_uid_length
    }

    /// The SOP Class UID field.
    #[must_use]
    pub fn sop_class_uid(&self) -> &Vec<u8> {
        &self.sop_class_uid
    }

    /// The number of bytes in the Service Class UID field.
    #[must_use]
    pub fn service_class_length(&self) -> u16 {
        self.service_class_uid_length
    }

    /// The Service Class UID field.
    #[must_use]
    pub fn service_class_uid(&self) -> &Vec<u8> {
        &self.service_class_uid
    }

    /// The number of bytes in the Related General SOP Class Identification field. May be zero if
    /// that field is not present.
    #[must_use]
    pub fn rel_gen_sop_class_length(&self) -> u16 {
        self.rel_gen_sop_classes_length
    }

    /// The Related General SOP Class Identification fields.
    #[must_use]
    pub fn rel_gen_sop_classes(&self) -> &Vec<RelatedGeneralSOPClassUID> {
        &self.rel_gen_sop_classes
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // version
            + size_of::<u16>() // length
            + size_of::<u16>() // sop_class_uid_length
            + self.sop_class_uid.len()
            + size_of::<u16>() // service_class_uid_length
            + self.service_class_uid.len()
            + size_of::<u16>() // rel_gen_sop_classes_length
            + self.rel_gen_sop_classes.iter().map(RelatedGeneralSOPClassUID::byte_size).sum::<usize>()
            + self.reserved.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.version];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;
        dataset.write_all(&self.service_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.service_class_uid)?;
        dataset.write_all(&self.rel_gen_sop_classes_length.to_be_bytes())?;
        for rel_gen_sop_class in &self.rel_gen_sop_classes {
            rel_gen_sop_class.write(&mut dataset)?;
        }

        if !self.reserved.is_empty() {
            dataset.write_all(&self.reserved)?;
        }

        Ok(())
    }

    /// Read a `SOPClassCommonExtendedNegotiationItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, version: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let sop_class_uid_length = u16::from_be_bytes(buf);
        let mut sop_class_uid: Vec<u8> = vec![0u8; sop_class_uid_length.into()];
        dataset.read_exact(&mut sop_class_uid)?;

        dataset.read_exact(&mut buf)?;
        let service_class_length = u16::from_be_bytes(buf);
        let mut service_class_uid: Vec<u8> = vec![0u8; service_class_length.into()];
        dataset.read_exact(&mut service_class_uid)?;

        dataset.read_exact(&mut buf)?;
        let rel_gen_sop_class_length = u16::from_be_bytes(buf);

        // The rel_gen_sop_class_length field is the number of bytes in total for all
        // RelatedGeneralSOPClasUIDs. After reading one, subtract its size in bytes so
        // we know when the last byte of this PDU has been read.
        let mut rel_gen_sop_class_len_marker: usize = rel_gen_sop_class_length.into();
        let mut rel_gen_sop_classes: Vec<RelatedGeneralSOPClassUID> = Vec::new();
        while rel_gen_sop_class_len_marker > 0 {
            let rel_gen_sop_class = RelatedGeneralSOPClassUID::read(&mut dataset)?;
            rel_gen_sop_class_len_marker -= rel_gen_sop_class.byte_size();
            rel_gen_sop_classes.push(rel_gen_sop_class);
        }

        let reserved_len: usize = usize::from(length)
            - (size_of::<u16>()
                + usize::from(sop_class_uid_length)
                + size_of::<u16>()
                + usize::from(service_class_length)
                + size_of::<u16>()
                + usize::from(rel_gen_sop_class_length));
        let reserved = if reserved_len > 0 {
            let mut reserved: Vec<u8> = vec![0u8; reserved_len];
            dataset.read_exact(&mut reserved)?;
            reserved
        } else {
            Vec::with_capacity(0)
        };

        Ok(Self {
            length,
            version,
            sop_class_uid_length,
            sop_class_uid,
            service_class_uid_length: service_class_length,
            service_class_uid,
            rel_gen_sop_classes_length: rel_gen_sop_class_length,
            rel_gen_sop_classes,
            reserved,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RelatedGeneralSOPClassUID {
    length: u16,
    rel_gen_sop_class: Vec<u8>,
}

impl RelatedGeneralSOPClassUID {
    /// Create a new `RelatedGeneralSOPClassUID`.
    #[must_use]
    pub fn new(rel_gen_sop_class: Vec<u8>) -> Self {
        Self {
            length: rel_gen_sop_class.len().try_into().unwrap_or_default(),
            rel_gen_sop_class,
        }
    }

    /// The number of bytes in the Related General SOP Class UID field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The Related General SOP Class UID field.
    #[must_use]
    pub fn rel_gen_sop_class(&self) -> &Vec<u8> {
        &self.rel_gen_sop_class
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u16>() // length
            + self.rel_gen_sop_class.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.rel_gen_sop_class)?;
        Ok(())
    }

    /// Read a `RelatedGeneralSOPClassUID` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut rel_gen_sop_class: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut rel_gen_sop_class)?;

        Ok(Self {
            length,
            rel_gen_sop_class,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserIdentityItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    identity_type: u8,
    pos_rsp_req: u8,
    pri_length: u16,
    pri_value: Vec<u8>,
    sec_length: u16,
    sec_value: Vec<u8>,
}

impl UserIdentityItem {
    /// The type of this PDU, `UserSubPduType::UserIdentityItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::UserIdentityItem
    }

    /// Create a new `UserIdentityItem`.
    #[must_use]
    pub fn new(identity_type: u8, pos_rsp_req: u8, pri_value: Vec<u8>, sec_value: Vec<u8>) -> Self {
        let length: usize = size_of::<u8>() // identity_type
            + size_of::<u8>() // pos_rsp_req
            + size_of::<u16>() // pri_length
            + pri_value.len()
            + size_of::<u16>() // sec_length
            + sec_value.len();

        Self {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            identity_type,
            pos_rsp_req,
            pri_length: pri_value.len().try_into().unwrap_or_default(),
            pri_value,
            sec_length: sec_value.len().try_into().unwrap_or_default(),
            sec_value,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the last
    /// field sent.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The user identity type.
    ///
    /// 1: Username.
    /// 2: Username and Passcode.
    /// 3: Kerberos Service Ticket.
    /// 4: SAML Assertion.
    /// 5: JSON Web Token.
    #[must_use]
    pub fn identity_type(&self) -> u8 {
        self.identity_type
    }

    /// Positive response requested.
    ///
    /// 0 - no response requested
    /// 1 - positive response requested
    #[must_use]
    pub fn pos_rsp_req(&self) -> u8 {
        self.pos_rsp_req
    }

    /// The length of the primary field.
    #[must_use]
    pub fn pri_length(&self) -> u16 {
        self.pri_length
    }

    /// The primary field value.
    #[must_use]
    pub fn pri_value(&self) -> &Vec<u8> {
        &self.pri_value
    }

    /// The length of the secondary field. This field should only be non-zero if the identity type
    /// is 2.
    #[must_use]
    pub fn sec_length(&self) -> u16 {
        self.sec_length
    }

    /// The secondary field. Only present if the identity type is 2.
    #[must_use]
    pub fn sec_value(&self) -> &Vec<u8> {
        &self.sec_value
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u8>() // identity_type
            + size_of::<u8>() // pos_rsp_req
            + size_of::<u16>() // pri_length
            + self.pri_value.len()
            + size_of::<u16>() // sec_length
            + self.sec_value.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 2] = [self.identity_type, self.pos_rsp_req];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.pri_length.to_be_bytes())?;
        dataset.write_all(&self.pri_value)?;

        dataset.write_all(&self.sec_length.to_be_bytes())?;
        dataset.write_all(&self.sec_value)?;
        Ok(())
    }

    /// Read a `UserIdentityItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let identity_type = buf[0];
        let pos_rsp_req = buf[1];

        dataset.read_exact(&mut buf)?;
        let pri_length = u16::from_be_bytes(buf);
        let mut pri_value: Vec<u8> = vec![0u8; pri_length.into()];
        dataset.read_exact(&mut pri_value)?;

        dataset.read_exact(&mut buf)?;
        let sec_length = u16::from_be_bytes(buf);
        let mut sec_value: Vec<u8> = vec![0u8; sec_length.into()];
        dataset.read_exact(&mut sec_value)?;

        Ok(Self {
            reserved,
            length,
            identity_type,
            pos_rsp_req,
            pri_length,
            pri_value,
            sec_length,
            sec_value,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserIdentityNegotiationItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    server_rsp_length: u16,
    server_rsp: Vec<u8>,
}

impl UserIdentityNegotiationItem {
    /// The type of this PDU, `UserSubPduType::UserIdentityNegotiationItem`.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::UserIdentityNegotiationItem
    }

    /// Create a new `UserIdentityNegotiationItem`.
    #[must_use]
    pub fn new(server_rsp: Vec<u8>) -> Self {
        let length: usize = size_of::<u16>() // server_rsp_length
            + server_rsp.len();

        Self {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            server_rsp_length: server_rsp.len().try_into().unwrap_or_default(),
            server_rsp,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// final field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Server response length. This field should only be non-zero if the identity type is 3 or 4.
    #[must_use]
    pub fn server_rsp_length(&self) -> u16 {
        self.server_rsp_length
    }

    /// Server response. If the identity type is 3 then this will be the Kerberos Service ticket
    /// encoded in accordance with RFC-1510. If the identity type is 4 then this will be the SAML
    /// response.
    #[must_use]
    pub fn server_rsp(&self) -> &Vec<u8> {
        &self.server_rsp
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // server_rsp_length
            + self.server_rsp.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;
        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.server_rsp_length.to_be_bytes())?;
        dataset.write_all(&self.server_rsp)?;
        Ok(())
    }

    /// Read a `UserIdentityNegotiationItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let server_rsp_length = u16::from_be_bytes(buf);
        let mut server_rsp: Vec<u8> = vec![0u8; server_rsp_length.into()];
        dataset.read_exact(&mut server_rsp)?;

        Ok(Self {
            reserved,
            length,
            server_rsp_length,
            server_rsp,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::dimse::pdus::{
        userpdus::{
            AsyncOperationsWindowItem, ImplementationClassUIDItem, ImplementationVersionNameItem,
            MaxLengthItem, RelatedGeneralSOPClassUID, RoleSelectionItem,
            SOPClassCommonExtendedNegotiationItem, SOPClassExtendedNegotiationItem,
            UserIdentityItem, UserIdentityNegotiationItem, UserPduType,
        },
        UserPdu,
    };

    /// Writes a PDU to an in-memory buffer, reads that buffer into a PDU, and asserts the result
    /// is equal to the input.
    fn assert_user_pdu_roundtrip(pdu: UserPdu) {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        pdu.write(&mut cursor).expect("unable to write pdu");

        // reset the position so reading from it starts at the beginning
        cursor.set_position(0);

        let roundtrip_pdu = UserPdu::read(&mut cursor).expect("unable to read pdu");
        assert_eq!(roundtrip_pdu, pdu);
    }

    #[test]
    fn test_max_length_roundtrip() {
        let pdu = MaxLengthItem::new(100);
        assert_user_pdu_roundtrip(UserPdu::MaxLengthItem(pdu));
    }

    #[test]
    fn test_impl_class_uid_roundtrip() {
        let pdu = ImplementationClassUIDItem::new(vec![3, 2, 1, 5, 4]);
        assert_user_pdu_roundtrip(UserPdu::ImplementationClassUIDItem(pdu));
    }

    #[test]
    fn test_async_win_item_roundtrip() {
        let pdu = AsyncOperationsWindowItem::new(65535, 256);
        assert_user_pdu_roundtrip(UserPdu::AsyncOperationsWindowItem(pdu));
    }

    #[test]
    fn test_role_item_roundtrip() {
        let pdu = RoleSelectionItem::new("2.3.4.5".into(), 4, 5);
        assert_user_pdu_roundtrip(UserPdu::RoleSelectionItem(pdu));
    }

    #[test]
    fn test_impl_ver_name_roundtrip() {
        let pdu = ImplementationVersionNameItem::new(vec![3, 2, 1, 5, 4]);
        assert_user_pdu_roundtrip(UserPdu::ImplementationVersionNameItem(pdu));
    }

    #[test]
    fn test_sop_class_extended_negotiation_roundtrip() {
        let pdu = SOPClassExtendedNegotiationItem::new("1.2.3.4".into(), "2.3.4.5.6".into());
        assert_user_pdu_roundtrip(UserPdu::SOPClassExtendedNegotiationItem(pdu));
    }

    #[test]
    fn test_sop_class_common_extended_negotiation_roundtrip() {
        let pdu = SOPClassCommonExtendedNegotiationItem::new(
            "1.2.3.4".into(),
            "2.3.4.5.6".into(),
            vec![
                RelatedGeneralSOPClassUID::new("1.2.3.4".into()),
                RelatedGeneralSOPClassUID::new("2.3.4.5.6".into()),
            ],
        );
        assert_user_pdu_roundtrip(UserPdu::SOPClassCommonExtendedNegotiationItem(pdu));
    }

    #[test]
    fn test_user_id() {
        let pdu = UserIdentityItem::new(2, 5, vec![3, 1, 4, 2, 5], vec![5, 3, 4, 2, 1]);
        assert_user_pdu_roundtrip(UserPdu::UserIdentityItem(pdu));
    }

    #[test]
    fn test_user_negotiation() {
        let pdu = UserIdentityNegotiationItem::new(vec![4, 5, 3, 2, 1]);
        assert_user_pdu_roundtrip(UserPdu::UserIdentityNegotiationItem(pdu));
    }

    #[test]
    fn test_user_pdu_type_roundtrip() {
        assert_eq!(
            UserPduType::MaxLengthItem,
            (u8::from(&UserPduType::MaxLengthItem)).into()
        );

        assert_eq!(
            UserPduType::ImplementationClassUIDItem,
            (u8::from(&UserPduType::ImplementationClassUIDItem)).into()
        );
        assert_eq!(
            UserPduType::AsyncOperationsWindowItem,
            (u8::from(&UserPduType::AsyncOperationsWindowItem)).into()
        );
        assert_eq!(
            UserPduType::RoleSelectionItem,
            (u8::from(&UserPduType::RoleSelectionItem)).into()
        );
        assert_eq!(
            UserPduType::ImplementationVersionNameItem,
            (u8::from(&UserPduType::ImplementationVersionNameItem)).into()
        );
        assert_eq!(
            UserPduType::SOPClassExtendedNegotiationItem,
            (u8::from(&UserPduType::SOPClassExtendedNegotiationItem)).into()
        );
        assert_eq!(
            UserPduType::SOPClassCommonExtendedNegotiationItem,
            (u8::from(&UserPduType::SOPClassCommonExtendedNegotiationItem)).into()
        );
        assert_eq!(
            UserPduType::UserIdentityItem,
            (u8::from(&UserPduType::UserIdentityItem)).into()
        );
        assert_eq!(
            UserPduType::UserIdentityNegotiationItem,
            (u8::from(&UserPduType::UserIdentityNegotiationItem)).into()
        );
    }
}
