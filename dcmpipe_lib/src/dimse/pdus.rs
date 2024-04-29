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

use std::io::{Read, Write};

use crate::dimse::{
    pdus::{
        mainpdus::{
            Abort, AbstractSyntaxItem, ApplicationContextItem, AssocAC, AssocACPresentationContext,
            AssocRJ, AssocRQ, AssocRQPresentationContext, PresentationDataItem,
            PresentationDataItemPartial, ReleaseRP, ReleaseRQ, TransferSyntaxItem,
            UserInformationItem,
        },
        userpdus::{
            AsyncOperationsWindowItem, ImplementationClassUIDItem, ImplementationVersionNameItem,
            MaxLengthItem, RoleSelectionItem, SOPClassCommonExtendedNegotiationItem,
            SOPClassExtendedNegotiationItem, UserIdentityItem, UserIdentityNegotiationItem,
        },
    },
    DimseError,
};

pub mod mainpdus;
pub mod pduiter;
pub mod userpdus;

#[derive(PartialEq, Eq, Clone)]
pub enum PduType {
    AssocRQ,
    AssocAC,
    AssocRJ,

    PresentationDataItem,
    PresentationDataItemPartial,

    ReleaseRQ,
    ReleaseRP,
    Abort,

    ApplicationContextItem,

    AssocRQPresentationContext,
    AssocACPresentationContext,

    AbstractSyntaxItem,
    TransferSyntaxItem,
    UserInformationItem,

    INVALID(u8),
}

impl std::fmt::Debug for PduType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PduType::AssocRQ => write!(f, "A-ASSOCIATE-RQ"),
            PduType::AssocAC => write!(f, "A-ASSOCIATE-AC"),
            PduType::AssocRJ => write!(f, "A-ASSOCIATE-RJ"),
            PduType::PresentationDataItem | PduType::PresentationDataItemPartial => {
                write!(f, "P-DATA")
            }
            PduType::ReleaseRQ => write!(f, "A-RELEASE-RQ"),
            PduType::ReleaseRP => write!(f, "A-RELEASE-RP"),
            PduType::Abort => write!(f, "A-ABORT"),
            PduType::ApplicationContextItem => write!(f, "ApplicationContextItem"),
            PduType::AssocRQPresentationContext => write!(f, "AssocRQPresentationContext"),
            PduType::AssocACPresentationContext => write!(f, "AssocACPresentationContext"),
            PduType::AbstractSyntaxItem => write!(f, "AbstractSyntaxItem"),
            PduType::TransferSyntaxItem => write!(f, "TransferSyntaxItem"),
            PduType::UserInformationItem => write!(f, "UserInformationItem"),
            PduType::INVALID(c) => write!(f, "INVALID {c:04x}"),
        }
    }
}

impl From<&PduType> for u8 {
    fn from(value: &PduType) -> Self {
        match value {
            PduType::AssocRQ => 0x01,
            PduType::AssocAC => 0x02,
            PduType::AssocRJ => 0x03,

            PduType::PresentationDataItem | PduType::PresentationDataItemPartial => 0x04,

            PduType::ReleaseRQ => 0x05,
            PduType::ReleaseRP => 0x06,
            PduType::Abort => 0x07,

            PduType::ApplicationContextItem => 0x10,

            PduType::AssocRQPresentationContext => 0x20,
            PduType::AssocACPresentationContext => 0x21,

            PduType::AbstractSyntaxItem => 0x30,
            PduType::TransferSyntaxItem => 0x40,
            PduType::UserInformationItem => 0x50,

            PduType::INVALID(c) => *c,
        }
    }
}

impl From<u8> for PduType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => PduType::AssocRQ,
            0x02 => PduType::AssocAC,
            0x03 => PduType::AssocRJ,

            // 0x04 => PduType::PresentationDataItem,
            0x04 => PduType::PresentationDataItemPartial,

            0x05 => PduType::ReleaseRQ,
            0x06 => PduType::ReleaseRP,
            0x07 => PduType::Abort,

            0x10 => PduType::ApplicationContextItem,

            0x20 => PduType::AssocRQPresentationContext,
            0x21 => PduType::AssocACPresentationContext,

            0x30 => PduType::AbstractSyntaxItem,
            0x40 => PduType::TransferSyntaxItem,
            0x50 => PduType::UserInformationItem,

            c => PduType::INVALID(c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pdu {
    AssocRQ(AssocRQ),
    AssocAC(AssocAC),
    AssocRJ(AssocRJ),
    PresentationDataItem(PresentationDataItem),
    PresentationDataItemPartial(PresentationDataItemPartial),
    ReleaseRQ(ReleaseRQ),
    ReleaseRP(ReleaseRP),
    Abort(Abort),
    ApplicationContextItem(ApplicationContextItem),
    AssocRQPresentationContext(AssocRQPresentationContext),
    AssocACPresentationContext(AssocACPresentationContext),
    AbstractSyntaxItem(AbstractSyntaxItem),
    TransferSyntaxItem(TransferSyntaxItem),
    UserInformationItem(UserInformationItem),
}

impl Pdu {
    /// The `PduType` for this PDU.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        match self {
            Pdu::AssocRQ(pdu) => pdu.pdu_type(),
            Pdu::AssocAC(pdu) => pdu.pdu_type(),
            Pdu::AssocRJ(pdu) => pdu.pdu_type(),
            Pdu::PresentationDataItem(pdu) => pdu.pdu_type(),
            Pdu::PresentationDataItemPartial(pdu) => pdu.pdu_type(),
            Pdu::ReleaseRQ(pdu) => pdu.pdu_type(),
            Pdu::ReleaseRP(pdu) => pdu.pdu_type(),
            Pdu::Abort(pdu) => pdu.pdu_type(),
            Pdu::ApplicationContextItem(pdu) => pdu.pdu_type(),
            Pdu::AssocRQPresentationContext(pdu) => pdu.pdu_type(),
            Pdu::AssocACPresentationContext(pdu) => pdu.pdu_type(),
            Pdu::AbstractSyntaxItem(pdu) => pdu.pdu_type(),
            Pdu::TransferSyntaxItem(pdu) => pdu.pdu_type(),
            Pdu::UserInformationItem(pdu) => pdu.pdu_type(),
        }
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    ///
    /// # Notes
    /// See `PresentationDataItemPartial` which exists for more flexible memory management when
    /// dealing with potentially large `PresentationDataValue`s. Regardless of that, this will
    /// still return the total number of bytes for this entire PDU.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        match self {
            Pdu::AssocRQ(pdu) => pdu.byte_size(),
            Pdu::AssocAC(pdu) => pdu.byte_size(),
            Pdu::AssocRJ(pdu) => pdu.byte_size(),
            Pdu::PresentationDataItem(pdu) => pdu.byte_size(),
            Pdu::PresentationDataItemPartial(pdu) => pdu.byte_size(),
            Pdu::ReleaseRQ(pdu) => pdu.byte_size(),
            Pdu::ReleaseRP(pdu) => pdu.byte_size(),
            Pdu::Abort(pdu) => pdu.byte_size(),
            Pdu::ApplicationContextItem(pdu) => pdu.byte_size(),
            Pdu::AssocRQPresentationContext(pdu) => pdu.byte_size(),
            Pdu::AssocACPresentationContext(pdu) => pdu.byte_size(),
            Pdu::AbstractSyntaxItem(pdu) => pdu.byte_size(),
            Pdu::TransferSyntaxItem(pdu) => pdu.byte_size(),
            Pdu::UserInformationItem(pdu) => pdu.byte_size(),
        }
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, dataset: W) -> Result<(), DimseError> {
        match self {
            Pdu::AssocRQ(pdu) => pdu.write(dataset),
            Pdu::AssocAC(pdu) => pdu.write(dataset),
            Pdu::AssocRJ(pdu) => pdu.write(dataset),
            Pdu::PresentationDataItem(pdu) => pdu.write(dataset),
            Pdu::PresentationDataItemPartial(pdu) => pdu.write(dataset),
            Pdu::ReleaseRQ(pdu) => pdu.write(dataset),
            Pdu::ReleaseRP(pdu) => pdu.write(dataset),
            Pdu::Abort(pdu) => pdu.write(dataset),
            Pdu::ApplicationContextItem(pdu) => pdu.write(dataset),
            Pdu::AssocRQPresentationContext(pdu) => pdu.write(dataset),
            Pdu::AssocACPresentationContext(pdu) => pdu.write(dataset),
            Pdu::AbstractSyntaxItem(pdu) => pdu.write(dataset),
            Pdu::TransferSyntaxItem(pdu) => pdu.write(dataset),
            Pdu::UserInformationItem(pdu) => pdu.write(dataset),
        }
    }

    /// Read a PDU from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R) -> Result<Pdu, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf).map_err(DimseError::IOError)?;

        let pdu_type: PduType = PduType::from(buf[0]);

        let byte1 = buf[1];

        let pdu = match pdu_type {
            PduType::AssocRQ => Pdu::AssocRQ(AssocRQ::read(dataset, byte1)?),
            PduType::AssocAC => Pdu::AssocAC(AssocAC::read(dataset, byte1)?),
            PduType::AssocRJ => Pdu::AssocRJ(AssocRJ::read(dataset, byte1)?),
            PduType::PresentationDataItem => {
                Pdu::PresentationDataItem(PresentationDataItem::read(dataset, byte1)?)
            }
            PduType::PresentationDataItemPartial => {
                Pdu::PresentationDataItemPartial(PresentationDataItemPartial::read(dataset, byte1)?)
            }
            PduType::ReleaseRQ => Pdu::ReleaseRQ(ReleaseRQ::read(dataset, byte1)?),
            PduType::ReleaseRP => Pdu::ReleaseRP(ReleaseRP::read(dataset, byte1)?),
            PduType::Abort => Pdu::Abort(Abort::read(dataset, byte1)?),
            PduType::ApplicationContextItem => {
                Pdu::ApplicationContextItem(ApplicationContextItem::read(dataset, byte1)?)
            }
            PduType::AssocRQPresentationContext => {
                Pdu::AssocRQPresentationContext(AssocRQPresentationContext::read(dataset, byte1)?)
            }
            PduType::AssocACPresentationContext => {
                Pdu::AssocACPresentationContext(AssocACPresentationContext::read(dataset, byte1)?)
            }
            PduType::AbstractSyntaxItem => {
                Pdu::AbstractSyntaxItem(AbstractSyntaxItem::read(dataset, byte1)?)
            }
            PduType::TransferSyntaxItem => {
                Pdu::TransferSyntaxItem(TransferSyntaxItem::read(dataset, byte1)?)
            }
            PduType::UserInformationItem => {
                Pdu::UserInformationItem(UserInformationItem::read(dataset, byte1)?)
            }

            PduType::INVALID(c) => return Err(DimseError::InvalidPduType(c)),
        };

        Ok(pdu)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserPduType {
    MaxLengthItem,
    ImplementationClassUIDItem,
    AsyncOperationsWindowItem,
    RoleSelectionItem,
    ImplementationVersionNameItem,
    SOPClassExtendedNegotiationItem,
    SOPClassCommonExtendedNegotiationItem,
    UserIdentityItem,
    UserIdentityNegotiationItem,

    INVALID(u8),
}

impl From<&UserPduType> for u8 {
    fn from(value: &UserPduType) -> Self {
        match value {
            UserPduType::MaxLengthItem => 0x51,
            UserPduType::ImplementationClassUIDItem => 0x52,
            UserPduType::AsyncOperationsWindowItem => 0x53,
            UserPduType::RoleSelectionItem => 0x54,
            UserPduType::ImplementationVersionNameItem => 0x55,
            UserPduType::SOPClassExtendedNegotiationItem => 0x56,
            UserPduType::SOPClassCommonExtendedNegotiationItem => 0x57,
            UserPduType::UserIdentityItem => 0x58,
            UserPduType::UserIdentityNegotiationItem => 0x59,

            UserPduType::INVALID(c) => *c,
        }
    }
}

impl From<u8> for UserPduType {
    fn from(value: u8) -> Self {
        match value {
            0x51 => UserPduType::MaxLengthItem,
            0x52 => UserPduType::ImplementationClassUIDItem,
            0x53 => UserPduType::AsyncOperationsWindowItem,
            0x54 => UserPduType::RoleSelectionItem,
            0x55 => UserPduType::ImplementationVersionNameItem,
            0x56 => UserPduType::SOPClassExtendedNegotiationItem,
            0x57 => UserPduType::SOPClassCommonExtendedNegotiationItem,
            0x58 => UserPduType::UserIdentityItem,
            0x59 => UserPduType::UserIdentityNegotiationItem,

            c => UserPduType::INVALID(c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserPdu {
    MaxLengthItem(MaxLengthItem),
    ImplementationClassUIDItem(ImplementationClassUIDItem),
    AsyncOperationsWindowItem(AsyncOperationsWindowItem),
    RoleSelectionItem(RoleSelectionItem),
    ImplementationVersionNameItem(ImplementationVersionNameItem),
    SOPClassExtendedNegotiationItem(SOPClassExtendedNegotiationItem),
    SOPClassCommonExtendedNegotiationItem(SOPClassCommonExtendedNegotiationItem),
    UserIdentityItem(UserIdentityItem),
    UserIdentityNegotiationItem(UserIdentityNegotiationItem),
}

impl UserPdu {
    /// The `UserPduType` for this User PDU.
    #[must_use]
    pub fn pdu_type(&self) -> UserPduType {
        match self {
            UserPdu::MaxLengthItem(pdu) => pdu.pdu_type(),
            UserPdu::ImplementationClassUIDItem(pdu) => pdu.pdu_type(),
            UserPdu::AsyncOperationsWindowItem(pdu) => pdu.pdu_type(),
            UserPdu::RoleSelectionItem(pdu) => pdu.pdu_type(),
            UserPdu::ImplementationVersionNameItem(pdu) => pdu.pdu_type(),
            UserPdu::SOPClassExtendedNegotiationItem(pdu) => pdu.pdu_type(),
            UserPdu::SOPClassCommonExtendedNegotiationItem(pdu) => pdu.pdu_type(),
            UserPdu::UserIdentityItem(pdu) => pdu.pdu_type(),
            UserPdu::UserIdentityNegotiationItem(pdu) => pdu.pdu_type(),
        }
    }

    /// The total number of bytes that this User PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        match self {
            UserPdu::MaxLengthItem(pdu) => pdu.byte_size(),
            UserPdu::ImplementationClassUIDItem(pdu) => pdu.byte_size(),
            UserPdu::AsyncOperationsWindowItem(pdu) => pdu.byte_size(),
            UserPdu::RoleSelectionItem(pdu) => pdu.byte_size(),
            UserPdu::ImplementationVersionNameItem(pdu) => pdu.byte_size(),
            UserPdu::SOPClassExtendedNegotiationItem(pdu) => pdu.byte_size(),
            UserPdu::SOPClassCommonExtendedNegotiationItem(pdu) => pdu.byte_size(),
            UserPdu::UserIdentityItem(pdu) => pdu.byte_size(),
            UserPdu::UserIdentityNegotiationItem(pdu) => pdu.byte_size(),
        }
    }

    /// Write this User PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, dataset: W) -> Result<(), DimseError> {
        match self {
            UserPdu::MaxLengthItem(pdu) => pdu.write(dataset),
            UserPdu::ImplementationClassUIDItem(pdu) => pdu.write(dataset),
            UserPdu::AsyncOperationsWindowItem(pdu) => pdu.write(dataset),
            UserPdu::RoleSelectionItem(pdu) => pdu.write(dataset),
            UserPdu::ImplementationVersionNameItem(pdu) => pdu.write(dataset),
            UserPdu::SOPClassExtendedNegotiationItem(pdu) => pdu.write(dataset),
            UserPdu::SOPClassCommonExtendedNegotiationItem(pdu) => pdu.write(dataset),
            UserPdu::UserIdentityItem(pdu) => pdu.write(dataset),
            UserPdu::UserIdentityNegotiationItem(pdu) => pdu.write(dataset),
        }
    }

    /// Read a User PDU from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R) -> Result<UserPdu, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf).map_err(DimseError::IOError)?;

        let pdu_type: UserPduType = UserPduType::from(buf[0]);

        let byte1 = buf[1];

        let pdu = match pdu_type {
            UserPduType::MaxLengthItem => {
                UserPdu::MaxLengthItem(MaxLengthItem::read(dataset, byte1)?)
            }
            UserPduType::ImplementationClassUIDItem => UserPdu::ImplementationClassUIDItem(
                ImplementationClassUIDItem::read(dataset, byte1)?,
            ),
            UserPduType::AsyncOperationsWindowItem => {
                UserPdu::AsyncOperationsWindowItem(AsyncOperationsWindowItem::read(dataset, byte1)?)
            }
            UserPduType::RoleSelectionItem => {
                UserPdu::RoleSelectionItem(RoleSelectionItem::read(dataset, byte1)?)
            }
            UserPduType::ImplementationVersionNameItem => UserPdu::ImplementationVersionNameItem(
                ImplementationVersionNameItem::read(dataset, byte1)?,
            ),
            UserPduType::SOPClassExtendedNegotiationItem => {
                UserPdu::SOPClassExtendedNegotiationItem(SOPClassExtendedNegotiationItem::read(
                    dataset, byte1,
                )?)
            }
            UserPduType::SOPClassCommonExtendedNegotiationItem => {
                UserPdu::SOPClassCommonExtendedNegotiationItem(
                    SOPClassCommonExtendedNegotiationItem::read(dataset, byte1)?,
                )
            }
            UserPduType::UserIdentityItem => {
                UserPdu::UserIdentityItem(UserIdentityItem::read(dataset, byte1)?)
            }
            UserPduType::UserIdentityNegotiationItem => UserPdu::UserIdentityNegotiationItem(
                UserIdentityNegotiationItem::read(dataset, byte1)?,
            ),

            UserPduType::INVALID(c) => return Err(DimseError::InvalidPduType(c)),
        };

        Ok(pdu)
    }
}
