//! Protocol Data Units for DIMSE. Part 8, Chapter 9.3
//!
//! PDU headers are encoded with Big Endian. The value fields are sent using the transfer syntax
//! negotiated during establishment of the association.

use std::{
    io::{Read, Write},
    mem::size_of,
};

use crate::{
    core::defn::{ts::TSRef, uid::UIDRef},
    dimse::DimseError,
};

/// This is the current DICOM standard defined version for Associations.
/// See Part 8, Section 9.3.2 (ASSOCIATE-RQ) and Section 9.3.3 (ASSOCIATE-AC).
pub(crate) static ASSOCIATION_VERSION: u16 = 0b0000_0000_0000_0001;

/// This is the current DICOM standard defined version for Common Extended Negotiation.
/// See Part 7, Annex D.3.3.6.1
pub(crate) static SOP_CLASS_COMMON_EXTENDED_NEGOTIATION_VERSION: u8 = 0b0000_0000;

/// This is the sole/single application context name for the current version of the DICOM standard.
/// See Part 7, Annex A.2.1
pub static STD_APP_CONTEXT_NAME: &str = "1.2.840.10008.3.1.1.1";

/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a DICOM dataset, and is
/// not the last fragment.
pub static P_DATA_DCM_DATASET: u8 = 0b00;
/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a DICOM dataset, and is
/// the last fragment.
pub static P_DATA_DCM_DATASET_LAST: u8 = 0b10;
/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a command, and is not
/// the last fragment.
pub static P_DATA_CMD: u8 = 0b01;
/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a command, and is the
/// last fragment.
pub static P_DATA_CMD_LAST: u8 = 0b11;

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
            PduType::PresentationDataItem => write!(f, "P-DATA"),
            PduType::PresentationDataItemPartial => write!(f, "P-DATA"),
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

            PduType::PresentationDataItem => 0x04,
            PduType::PresentationDataItemPartial => 0x04,

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

    pub fn read<R: Read>(mut dataset: R) -> Result<Pdu, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset
            .read_exact(&mut buf)
            .map_err(DimseError::IOError)?;

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

    pub fn read<R: Read>(mut dataset: R) -> Result<UserPdu, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset
            .read_exact(&mut buf)
            .map_err(DimseError::IOError)?;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocRQ {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    version: u16,
    /// Reserved, should be zeros.
    reserved_2: [u8; 2],
    called_ae: [u8; 16],
    calling_ae: [u8; 16],
    /// Reserved, should be zeros.
    reserved_3: [u8; 32],
    app_ctx: ApplicationContextItem,
    pres_ctxs: Vec<AssocRQPresentationContext>,
    user_info: UserInformationItem,
}

impl AssocRQ {
    /// The type of this PDU, `PduType::AssocRQ`.
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocRQ
    }

    pub fn new(
        called_ae: [u8; 16],
        calling_ae: [u8; 16],
        app_ctx: ApplicationContextItem,
        pres_ctxs: Vec<AssocRQPresentationContext>,
        user_info: UserInformationItem,
    ) -> AssocRQ {
        let length: usize = size_of::<u8>() + // version
            size_of::<[u8; 2]>() + // reserved_2
            size_of::<[u8; 16]>() + // called_ae
            size_of::<[u8; 16]>() + // calling_ae
            size_of::<[u8; 32]>() + // reserved_3
            app_ctx.byte_size() +
            pres_ctxs.iter().map(|p| p.byte_size()).sum::<usize>() +
            user_info.byte_size();

        AssocRQ {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            version: ASSOCIATION_VERSION,
            reserved_2: [0u8; 2],
            called_ae,
            calling_ae,
            reserved_3: [0u8; 32],
            app_ctx,
            pres_ctxs,
            user_info,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (User Information).
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Identifies each version of the DICOM Upper Layer protocol supported by the calling
    /// end-system. Currently this is version 1, identified with bit 0 set.
    pub fn version(&self) -> u16 {
        self.version
    }

    /// Destination AE, 16 characters encoded with the Basic G0 Set (essentially ASCII), with
    /// leading and trailing spaces being non-significant.
    pub fn called_ae(&self) -> &[u8; 16] {
        &self.called_ae
    }

    /// Source AE, 16 characters encoded with the Basic G0 Set (essentially ASCII), with leading
    /// and trailing spaces being non-significant.
    pub fn calling_ae(&self) -> &[u8; 16] {
        &self.calling_ae
    }

    /// The third reserved field, 32 bytes. This is exposed as the standard requires this value be
    /// copied in the AssocAC response.
    pub fn reserved_3(&self) -> &[u8; 32] {
        &self.reserved_3
    }

    /// Application Context.
    pub fn app_ctx(&self) -> &ApplicationContextItem {
        &self.app_ctx
    }

    /// Presentation Contexts, at least one.
    pub fn pres_ctxs(&self) -> &Vec<AssocRQPresentationContext> {
        &self.pres_ctxs
    }

    /// User Information.
    pub fn user_info(&self) -> &UserInformationItem {
        &self.user_info
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u16>() // version
            + self.reserved_2.len()
            + self.called_ae.len()
            + self.calling_ae.len()
            + self.reserved_3.len()
            + self.app_ctx.byte_size()
            + self.pres_ctxs.iter().map(|p| p.byte_size()).sum::<usize>()
            + self.user_info.byte_size()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.version.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;
        dataset.write_all(&self.called_ae)?;
        dataset.write_all(&self.calling_ae)?;
        dataset.write_all(&self.reserved_3)?;

        self.app_ctx.write(&mut dataset)?;
        for pres_ctx in &self.pres_ctxs {
            pres_ctx.write(&mut dataset)?;
        }
        self.user_info.write(&mut dataset)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<AssocRQ, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let version = u16::from_be_bytes(buf);

        let mut reserved_2 = [0u8; 2];
        dataset.read_exact(&mut reserved_2)?;

        let mut called_ae: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut called_ae)?;

        let mut calling_ae: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut calling_ae)?;

        let mut reserved_3: [u8; 32] = [0u8; 32];
        dataset.read_exact(&mut reserved_3)?;

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let app_ctx = ApplicationContextItem::read(&mut dataset, buf[1])?;

        let mut pres_ctxs: Vec<AssocRQPresentationContext> = Vec::new();
        dataset.read_exact(&mut buf)?;
        while buf[0] == u8::from(&PduType::AssocRQPresentationContext) {
            let pres_ctx = AssocRQPresentationContext::read(&mut dataset, buf[1])?;
            pres_ctxs.push(pres_ctx);
            dataset.read_exact(&mut buf)?;
        }

        let user_info = UserInformationItem::read(&mut dataset, buf[1])?;

        Ok(AssocRQ {
            reserved_1: reserved,
            length,
            version,
            reserved_2,
            called_ae,
            calling_ae,
            reserved_3,
            app_ctx,
            pres_ctxs,
            user_info,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocAC {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    version: u16,
    /// Reserved, should be zeros.
    reserved_2: [u8; 2],
    /// Reserved, but this should be populated with the Destination AE field from the RQ though
    /// its value should not be checked.
    reserved_3: [u8; 16],
    /// Reserved, but this should be populated with the Source AE field from the RQ though its
    /// value should not be checked.
    reserved_4: [u8; 16],
    /// Reserved, but this should be populated with the corresponding reserved field from the RQ,
    /// which should be all zeros though its value should not be checked.
    reserved_5: [u8; 32],
    app_ctx: ApplicationContextItem,
    pres_ctxs: Vec<AssocACPresentationContext>,
    user_info: UserInformationItem,
}

impl AssocAC {
    /// The type of this PDU, `PduType::AssocAC`.
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocAC
    }

    pub fn new(
        reserved_3: [u8; 16],
        reserved_4: [u8; 16],
        reserved_5: [u8; 32],
        app_ctx: ApplicationContextItem,
        pres_ctxs: Vec<AssocACPresentationContext>,
        user_info: UserInformationItem,
    ) -> AssocAC {
        let length: usize = size_of::<u16>() // version
            + size_of::<[u8; 2]>() // reserved_2
            + size_of::<[u8; 16]>() // reserved_3
            + size_of::<[u8; 16]>() // reserved_4
            + size_of::<[u8; 32]>() // reserved_5
            + app_ctx.byte_size()
            + pres_ctxs.iter().map(|p| p.byte_size()).sum::<usize>()
            + user_info.byte_size();

        AssocAC {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            version: ASSOCIATION_VERSION,
            reserved_2: [0u8; 2],
            reserved_3,
            reserved_4,
            reserved_5,
            app_ctx,
            pres_ctxs,
            user_info,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable items field (User Information).
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Identifies each version of the DICOM Upper Layer protocol supported by the calling
    /// end-system. Currently this is version 1, identified with bit 0 set.
    pub fn version(&self) -> u16 {
        self.version
    }

    /// Reserved, but this should be populated with the Destination AE field from the RQ though
    /// its value should not be checked.
    pub fn reserved_3(&self) -> &[u8; 16] {
        &self.reserved_3
    }

    /// Reserved, but this should be populated with the Source AE field from the RQ though its
    /// value should not be checked.
    pub fn reserved_4(&self) -> &[u8; 16] {
        &self.reserved_4
    }

    /// Application Context.
    pub fn app_ctx(&self) -> &ApplicationContextItem {
        &self.app_ctx
    }

    /// Presentation Contexts, at least one.
    pub fn pres_ctxs(&self) -> &Vec<AssocACPresentationContext> {
        &self.pres_ctxs
    }

    /// User Information.
    pub fn user_info(&self) -> &UserInformationItem {
        &self.user_info
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u16>() // version
            + self.reserved_2.len()
            + self.reserved_3.len()
            + self.reserved_4.len()
            + self.reserved_5.len()
            + self.app_ctx.byte_size()
            + self.pres_ctxs.iter().map(|p| p.byte_size()).sum::<usize>()
            + self.user_info.byte_size()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.version.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;
        dataset.write_all(&self.reserved_3)?;
        dataset.write_all(&self.reserved_4)?;
        dataset.write_all(&self.reserved_5)?;

        self.app_ctx.write(&mut dataset)?;
        for pres_ctx in &self.pres_ctxs {
            pres_ctx.write(&mut dataset)?;
        }
        self.user_info.write(&mut dataset)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<AssocAC, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let version = u16::from_be_bytes(buf);

        let mut reserved_2 = [0u8; 2];
        dataset.read_exact(&mut reserved_2)?;

        let mut reserved_3: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut reserved_3)?;

        let mut reserved_4: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut reserved_4)?;

        let mut reserved_5: [u8; 32] = [0u8; 32];
        dataset.read_exact(&mut reserved_5)?;

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let app_ctx = ApplicationContextItem::read(&mut dataset, buf[1])?;

        let mut pres_ctxs: Vec<AssocACPresentationContext> = Vec::new();
        dataset.read_exact(&mut buf)?;
        while buf[0] == u8::from(&PduType::AssocACPresentationContext) {
            let pres_ctx = AssocACPresentationContext::read(&mut dataset, buf[1])?;
            pres_ctxs.push(pres_ctx);
            dataset.read_exact(&mut buf)?;
        }

        let user_info = UserInformationItem::read(&mut dataset, buf[1])?;

        Ok(AssocAC {
            reserved_1: reserved,
            length,
            version,
            reserved_2,
            reserved_3,
            reserved_4,
            reserved_5,
            app_ctx,
            pres_ctxs,
            user_info,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocRJ {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zero.
    reserved_2: u8,
    result: u8,
    source: u8,
    reason: u8,
}

impl AssocRJ {
    /// The type of this PDU, `PduType::AssocRJ`.
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocRJ
    }

    /// Create a new A-ASSOCIATE-RJ
    ///
    /// `result`, whether the rejection is transient or permanent.
    ///   - 1: Rejected-permanent.
    ///   - 2: Rejected-transient.
    ///
    /// `source`, source of the rejection.
    ///   - 1: DICOM Upper Layer Service User.
    ///   - 2: DICOM Upper Layer Service Provider (ACSE-related function).
    ///   - 3: DICOM Upper Layer Service Provider (Presentation-related function).
    ///
    /// `reason`, why the association is rejected.
    ///   - If `source` is 1,
    ///     - 1: No reason given.
    ///     - 2: Application Context Name not supported.
    ///     - 3: Calling AE Title not recognized.
    ///     - 4-6: Reserved.
    ///     - 7: Called AE Title not recoginzed.
    ///     - 8-10: Reserved.
    ///   - If `source` is 2,
    ///     - 1: No reason given.
    ///     - 2: Protocol version not supported.
    ///   - If `source` is 3,
    ///     - 0: Reserved.
    ///     - 1: Temporary congestion.
    ///     - 2: Local limit exceeded.
    ///     - 3-7: Reserved.
    pub fn new(result: u8, source: u8, reason: u8) -> AssocRJ {
        let length: usize = size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // source
            + size_of::<u8>(); // reason

        AssocRJ {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: 0u8,
            result,
            source,
            reason,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reason/Diag. field. This should be a fixed value of 4.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Result.
    ///
    /// 1 - Rejected-permanent.
    /// 2 - Rejected-transient.
    pub fn result(&self) -> u8 {
        self.result
    }

    /// Source.
    ///
    /// 1 - DICOM Upper Layer Service User.
    /// 2 - DICOM Upper Layer Service Provider (ACSE-related function).
    /// 3 - DICOM Upper Layer Service Provider (Presentation-related function).
    pub fn source(&self) -> u8 {
        self.source
    }

    /// Reason / Diag.
    ///
    /// If Source is 1,
    ///   1: No reason given.
    ///   2: Application Context Name not supported.
    ///   3: Calling AE Title not recognized.
    ///   4-6: Reserved.
    ///   7: Called AE Title not recoginzed.
    ///   8-10: Reserved.
    ///
    /// If Source is 2,
    ///   1: No reason given.
    ///   2: Protocol version not supported.
    ///
    /// If Source is 3,
    ///   0: Reserved.
    ///   1: Temporary congestion.
    ///   2: Local limit exceeded.
    ///   3-7: Reserved.
    pub fn reason(&self) -> u8 {
        self.reason
    }

    /// Returns a description of the reason for A-ASSOCIATION-RJ.
    pub fn get_reason_desc(&self) -> &'static str {
        if self.source == 1 {
            match self.reason {
                1 => "No reason given.",
                2 => "Application Context Name not supported.",
                3 => "Calling AE Title not recognized.",
                7 => "Called AE Title not recognized.",
                _ => "Unknown reason.",
            }
        } else if self.source == 2 {
            match self.reason {
                1 => "No reason given.",
                2 => "Protocol version not supported.",
                _ => "Unknown reason.",
            }
        } else if self.source == 3 {
            match self.reason {
                1 => "Temporary congestion.",
                2 => "Local limit exceeded.",
                _ => "Unknown reason.",
            }
        } else {
            "Unknown reason."
        }
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // source
            + size_of::<u8>() // reason
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 4] = [self.reserved_2, self.result, self.source, self.reason];
        dataset.write_all(&buf)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<AssocRJ, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let reserved_2 = buf[0];
        let result = buf[1];
        let source = buf[2];
        let reason = buf[3];

        Ok(AssocRJ {
            reserved_1: reserved,
            length,
            reserved_2,
            result,
            source,
            reason,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReleaseRQ {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

impl ReleaseRQ {
    /// The type of this PDU, `PduType::ReleaseRQ`.
    pub fn pdu_type(&self) -> PduType {
        PduType::ReleaseRQ
    }

    pub fn new() -> ReleaseRQ {
        let length: usize = size_of::<[u8; 4]>(); // reserved_2

        ReleaseRQ {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: [0u8; 4],
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + self.reserved_2.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<ReleaseRQ, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;

        Ok(ReleaseRQ {
            reserved_1: reserved,
            length,
            reserved_2: buf,
        })
    }
}

impl Default for ReleaseRQ {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReleaseRP {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

impl ReleaseRP {
    /// The type of this PDU, `PduType::ReleaseRP`.
    pub fn pdu_type(&self) -> PduType {
        PduType::ReleaseRP
    }

    pub fn new() -> ReleaseRP {
        let length: usize = size_of::<[u8; 4]>(); // reserved_2

        ReleaseRP {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: [0u8; 4],
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + self.reserved_2.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<ReleaseRP, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;

        Ok(ReleaseRP {
            reserved_1: reserved,
            length,
            reserved_2: buf,
        })
    }
}

impl Default for ReleaseRP {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Abort {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zero.
    reserved_2: u8,
    /// Reserved, should be zero.
    reserved_3: u8,
    source: u8,
    reason: u8,
}

impl Abort {
    /// The type of this PDU, `PduType::Abort`.
    pub fn pdu_type(&self) -> PduType {
        PduType::Abort
    }

    /// Create a new A-ABORT.
    ///
    /// `source`, the source/initiator of the abort.
    ///   - 0: DICOM Upper Layer service-user.
    ///   - 1: Reserved.
    ///   - 2: DICOM Upper Layer service-provider.
    ///
    /// `reason`, the reason for the abort, only if `source` isn't `0`.
    ///   - 0: Reason not specified.
    ///   - 1: Unrecognized PDU.
    ///   - 2: Unexpected PDU.
    ///   - 3: Reserved / Unexpected session-service primitive.
    ///   - 4: Unrecognized PDU parameter.
    ///   - 5: Unexpected PDU parameter.
    ///   - 6: Invalid PDU parameter.
    pub fn new(source: u8, reason: u8) -> Abort {
        let length: usize = size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // source
            + size_of::<u8>(); // reason

        Abort {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: 0u8,
            reserved_3: 0u8,
            source,
            reason,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reason/Diag. field. This should be a fixed value of 4.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Source of the Abort.
    ///
    /// 0: DICOM Upper Layer service-user (initiated abort).
    /// 1: Reserved.
    /// 2: DICOM Upper Layer service-provider (initiated abort).
    pub fn source(&self) -> u8 {
        self.source
    }

    /// Reason/Diag.
    ///
    /// 0: Reason not specified.
    /// 1: Unrecognized PDU.
    /// 2: Unexpected PDU.
    /// 3: Reserved.
    /// 4: Unrecognized PDU parameter.
    /// 5: Unexpected PDU parameter.
    /// 6: Invalid PDU parameter.
    ///
    /// If the source field has a value of 0 (service-user) then this field shall not be
    /// significant and should be set to zero, but unchecked.
    pub fn reason(&self) -> u8 {
        self.reason
    }

    /// Returns a description of the reason for the A-ABORT.
    pub fn get_reason_desc(&self) -> &'static str {
        match self.reason {
            0 => "Not-specified",
            1 => "Unrecognized PDU",
            2 => "Unexpected PDU",
            3 => "Unexpected session-service primitive",
            4 => "Unrecognized PDU parameter",
            5 => "Unexpected PDU parameter",
            6 => "Invalid PDU parameter value",
            _ => "Unrecognized reason.",
        }
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // source
            + size_of::<u8>() // reason
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 4] = [self.reserved_2, self.reserved_3, self.source, self.reason];
        dataset.write_all(&buf)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Abort, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let reserved_2 = buf[0];
        let reserved_3 = buf[1];
        let source = buf[2];
        let reason = buf[3];

        Ok(Abort {
            reserved_1: reserved,
            length,
            reserved_2,
            reserved_3,
            source,
            reason,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PresentationDataItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u32,
    pres_data: Vec<PresentationDataValue>,
}

impl PresentationDataItem {
    /// The type of this PDU, `PduType::PresentationDataItem`.
    pub fn pdu_type(&self) -> PduType {
        PduType::PresentationDataItem
    }

    pub fn new(pres_data: Vec<PresentationDataValue>) -> PresentationDataItem {
        let length: usize = pres_data.iter().map(|p| p.byte_size()).sum::<usize>();

        PresentationDataItem {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            pres_data,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (Presentation Data values).
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Presentation Data values.
    pub fn pres_data(&self) -> &Vec<PresentationDataValue> {
        &self.pres_data
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u32>() // length
            + self.pres_data.iter().map(|p| p.byte_size()).sum::<usize>()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        for value in &self.pres_data {
            value.write(&mut dataset)?;
        }

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<PresentationDataItem, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut pres_data: Vec<PresentationDataValue> = Vec::new();
        let mut pres_data_len_marker = usize::try_from(length).unwrap_or_default();
        while pres_data_len_marker > 0 {
            let value = PresentationDataValue::read(&mut dataset)?;
            // The total bytes read for a single data value is 4 for the length field and then the
            // value of the length field which includes the ctx_id, msg_header, and data fields.
            let total_value_size =
                size_of::<u32>() + usize::try_from(value.length()).unwrap_or_default();
            pres_data_len_marker -= total_value_size;
            pres_data.push(value);
        }

        Ok(PresentationDataItem {
            reserved,
            length,
            pres_data,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PresentationDataValue {
    length: u32,
    ctx_id: u8,
    msg_header: u8,
    data: Vec<u8>,
}

impl PresentationDataValue {
    pub fn new(ctx_id: u8, msg_header: u8, data: Vec<u8>) -> PresentationDataValue {
        let length: usize = size_of::<u8>() // ctx_id
            + size_of::<u8>() // msg_header
            + data.len();

        PresentationDataValue {
            length: length.try_into().unwrap_or_default(),
            ctx_id,
            msg_header,
            data,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// presentation data value field.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Context ID, an odd number between 1-255.
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    /// Message Header, interpreted as bit fields.
    ///
    /// LSB 0,
    ///   0: The message contains a DICOM Data Set.
    ///   1: The message contains a Command.
    ///
    /// LSB 1,
    ///   0: The message fragment is not the last fragment.
    ///   1: The message fragment is the last fragment.
    ///
    /// The other bits shall be zeros, but unchecked.
    pub fn msg_header(&self) -> u8 {
        self.msg_header
    }

    /// Presentation data, a fragment. Either a Command or a DICOM Data Set.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u32>() // length
            + size_of::<u8>() // ctx_id
            + size_of::<u8>() // msg_header
            + self.data.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 2] = [self.ctx_id, self.msg_header];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.data)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R) -> Result<PresentationDataValue, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let msg_header = buf[1];

        let data_len = usize::try_from(length).unwrap_or_default() - size_of::<[u8; 2]>();
        let mut data: Vec<u8> = vec![0u8; data_len];
        dataset.read_exact(&mut data)?;

        Ok(PresentationDataValue {
            length,
            ctx_id,
            msg_header,
            data,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PresentationDataItemPartial {
    /// Reserved, should be zero.
    reserved: u8,
    length: u32,
}

impl PresentationDataItemPartial {
    /// The type of this PDU, `PduType::PresentationDataItemPartial`.
    pub fn pdu_type(&self) -> PduType {
        PduType::PresentationDataItemPartial
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (Presentation Data values).
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u32>() // length
            + usize::try_from(self.length).unwrap_or_default()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;
        dataset.write_all(&self.length.to_be_bytes())?;
        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<PresentationDataItemPartial, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        Ok(PresentationDataItemPartial { reserved, length })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PresentationDataValueHeader {
    length: u32,
    ctx_id: u8,
    msg_header: u8,
}

impl PresentationDataValueHeader {
    pub fn new(length: u32, ctx_id: u8, msg_header: u8) -> PresentationDataValueHeader {
        PresentationDataValueHeader {
            length,
            ctx_id,
            msg_header,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// presentation data value field.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Context ID, an odd number between 1-255.
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    /// Message Header, interpreted as bit fields.
    ///
    /// LSB 0,
    ///   0: The message contains a DICOM Data Set.
    ///   1: The message contains a Command.
    ///
    /// LSB 1,
    ///   0: The message fragment is not the last fragment.
    ///
    ///   1: The message fragment is the last fragment.
    /// The other bits shall be zeros, but unchecked.
    pub fn msg_header(&self) -> u8 {
        self.msg_header
    }

    /// The length of the data field.
    pub fn length_of_data(&self) -> u32 {
        // The length field appears before the ctx_id and msg_header fields, so the length's value
        // includes those two bytes which need subtracted.
        self.length - u32::try_from(size_of::<u8>() + size_of::<u8>()).unwrap_or_default()
    }

    /// Returns true if this value is a command message, false for a dicom dataset.
    pub fn is_command(&self) -> bool {
        self.msg_header & 0b01 == 0b01
    }

    /// Returns true if this value is the last fragment in a presentation data item, false if not.
    pub fn is_last_fragment(&self) -> bool {
        self.msg_header & 0b10 == 0b10
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u32>() // length
            + usize::try_from(self.length).unwrap_or_default()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 2] = [self.ctx_id, self.msg_header];
        dataset.write_all(&buf)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R) -> Result<PresentationDataValueHeader, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let msg_header = buf[1];

        Ok(PresentationDataValueHeader {
            length,
            ctx_id,
            msg_header,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ApplicationContextItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    app_context_name: Vec<u8>,
}

impl ApplicationContextItem {
    /// The type of this PDU, `PduType::ApplicationContextItem`.
    pub fn pdu_type(&self) -> PduType {
        PduType::ApplicationContextItem
    }

    pub fn new(app_context_name: Vec<u8>) -> ApplicationContextItem {
        let length: u16 = app_context_name.len().try_into().unwrap_or_default();

        ApplicationContextItem {
            reserved: 0u8,
            length,
            app_context_name,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// application context field name.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// A valid application context name (essentially a UID).
    pub fn app_context_name(&self) -> &Vec<u8> {
        &self.app_context_name
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.app_context_name.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.app_context_name)?;

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<ApplicationContextItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut app_context_name: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut app_context_name)?;

        Ok(ApplicationContextItem {
            reserved,
            length,
            app_context_name,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocRQPresentationContext {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u16,
    ctx_id: u8,
    /// Reserved, should be zero.
    reserved_2: u8,
    /// Reserved, should be zero.
    reserved_3: u8,
    /// Reserved, should be zero.
    reserved_4: u8,
    abstract_syntax: AbstractSyntaxItem,
    transfer_syntaxes: Vec<TransferSyntaxItem>,
}

impl AssocRQPresentationContext {
    /// The type of this PDU, `PduType::AssocRQPresentationContext`.
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocRQPresentationContext
    }

    pub fn new(
        ctx_id: u8,
        abstract_syntax: AbstractSyntaxItem,
        transfer_syntaxes: Vec<TransferSyntaxItem>,
    ) -> AssocRQPresentationContext {
        let length: usize = size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // reserved_4
            + abstract_syntax.byte_size()
            + transfer_syntaxes.iter().map(|p| p.byte_size()).sum::<usize>();

        AssocRQPresentationContext {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            ctx_id,
            reserved_2: 0u8,
            reserved_3: 0u8,
            reserved_4: 0u8,
            abstract_syntax,
            transfer_syntaxes,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the last
    /// transfer syntax item.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Presentation context ID, an odd number between 1 and 255.
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    /// The abstract syntax sub item.
    pub fn abstract_syntax(&self) -> &AbstractSyntaxItem {
        &self.abstract_syntax
    }

    /// The transfer syntax sub-items.
    pub fn transfer_syntaxes(&self) -> &Vec<TransferSyntaxItem> {
        &self.transfer_syntaxes
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u16>() // length
            + size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // reserved_4
            + self.abstract_syntax.byte_size()
            + self.transfer_syntaxes.iter().map(|t| t.byte_size()).sum::<usize>()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let mut buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        buf[0] = self.ctx_id;
        buf[1] = self.reserved_2;
        dataset.write_all(&buf)?;

        buf[0] = self.reserved_3;
        buf[1] = self.reserved_4;
        dataset.write_all(&buf)?;

        self.abstract_syntax.write(&mut dataset)?;

        for transfer_syntax in &self.transfer_syntaxes {
            transfer_syntax.write(&mut dataset)?;
        }

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<AssocRQPresentationContext, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut bytes_read = 0usize;
        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let reserved_2 = buf[1];
        bytes_read += buf.len();

        dataset.read_exact(&mut buf)?;
        let reserved_3 = buf[0];
        let reserved_4 = buf[1];
        bytes_read += buf.len();

        dataset.read_exact(&mut buf)?;
        let abstract_syntax = AbstractSyntaxItem::read(&mut dataset, buf[1])?;
        // The total bytes read for AbstractSyntaxItem, 2 bytes for the type and reserved, plus an
        // additional 2 for the length field, and the value of the length field being the size of
        // the last variable-length field.
        bytes_read += buf.len();
        bytes_read += usize::from(2 + abstract_syntax.length());

        // The length of total transfer syntax list is value of this PDU's length field less the
        // number of bytes before the transfer syntaxes field. There are no fields after the
        // transfer syntaxes.
        let mut transfer_syntax_len_marker = usize::from(length) - bytes_read;
        let mut transfer_syntaxes: Vec<TransferSyntaxItem> = Vec::new();
        while transfer_syntax_len_marker > 0 {
            dataset.read_exact(&mut buf)?;
            let transfer_syntax = TransferSyntaxItem::read(&mut dataset, buf[1])?;
            // Total bytes read is 2 for the type and reserved, plus another 2 for the length
            // field, and also the value of the length field being the size of the last
            // variable-length field.
            transfer_syntax_len_marker -= usize::from(4 + transfer_syntax.length());
            transfer_syntaxes.push(transfer_syntax);
        }

        Ok(AssocRQPresentationContext {
            reserved_1: reserved,
            length,
            ctx_id,
            reserved_2,
            reserved_3,
            reserved_4,
            abstract_syntax,
            transfer_syntaxes,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocACPresentationContext {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u16,
    ctx_id: u8,
    /// Reserved, should be zero.
    reserved_2: u8,
    result: u8,
    /// Reserved, should be zero.
    reserved_3: u8,
    transfer_syntax: TransferSyntaxItem,
}

impl AssocACPresentationContext {
    /// The type of this PDU, `PduType::AssocACPresentationContext`.
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocACPresentationContext
    }

    pub fn new(
        ctx_id: u8,
        result: u8,
        transfer_syntax: TransferSyntaxItem,
    ) -> AssocACPresentationContext {
        let length: usize = size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // reserved_3
            + transfer_syntax.byte_size();

        AssocACPresentationContext {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            ctx_id,
            reserved_2: 0u8,
            result,
            reserved_3: 0u8,
            transfer_syntax,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Transfer Syntax item.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Presentation context ID, an odd number between 1 and 255.
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    /// Result/Reason.
    ///
    /// 0: Acceptance.
    /// 1: User-rejection.
    /// 2: No reason (provider rejection).
    /// 3: Abstract Syntax not supported (provider rejection).
    /// 4: Transfer Syntaxes not supported (provider rejection).
    pub fn result(&self) -> u8 {
        self.result
    }

    /// The selected transfer syntax. When the `result` field has a value other than 0, this field
    /// shall not be significant and should be ignored. The `TransferSyntaxItem` should contain
    /// only a single transfer syntax.
    pub fn transfer_syntax(&self) -> &TransferSyntaxItem {
        &self.transfer_syntax
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u16>() // length
            + size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // reserved_3
            + self.transfer_syntax.byte_size()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let mut buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        buf[0] = self.ctx_id;
        buf[1] = self.reserved_2;
        dataset.write_all(&buf)?;

        buf[0] = self.result;
        buf[1] = self.reserved_3;
        dataset.write_all(&buf)?;

        self.transfer_syntax.write(&mut dataset)?;

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<AssocACPresentationContext, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let reserved_2 = buf[1];

        dataset.read_exact(&mut buf)?;
        let result = buf[0];
        let reserved_3 = buf[1];

        dataset.read_exact(&mut buf)?;
        let transfer_syntax = TransferSyntaxItem::read(dataset, buf[1])?;

        Ok(AssocACPresentationContext {
            reserved_1: reserved,
            length,
            ctx_id,
            reserved_2,
            result,
            reserved_3,
            transfer_syntax,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbstractSyntaxItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    abstract_syntax: Vec<u8>,
}

impl AbstractSyntaxItem {
    /// The type of this PDU, `PduType::AbstractSyntaxItem`.
    pub fn pdu_type(&self) -> PduType {
        PduType::AbstractSyntaxItem
    }

    pub fn new(abstract_syntax: Vec<u8>) -> AbstractSyntaxItem {
        AbstractSyntaxItem {
            reserved: 0u8,
            length: abstract_syntax.len().try_into().unwrap_or_default(),
            abstract_syntax,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// abstract syntax name field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The abstract syntax UID related to the proposed presentation context.
    pub fn abstract_syntax(&self) -> &Vec<u8> {
        &self.abstract_syntax
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.abstract_syntax.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.abstract_syntax)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<AbstractSyntaxItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut abstract_syntax: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut abstract_syntax)?;

        Ok(AbstractSyntaxItem {
            reserved,
            length,
            abstract_syntax,
        })
    }
}

impl From<UIDRef> for AbstractSyntaxItem {
    fn from(value: UIDRef) -> Self {
        Self::new(Vec::from(value.uid().as_bytes()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransferSyntaxItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    transfer_syntaxes: Vec<u8>,
}

impl TransferSyntaxItem {
    /// The type of this PDU, `PduType::TransferSyntaxItem`.
    pub fn pdu_type(&self) -> PduType {
        PduType::TransferSyntaxItem
    }

    pub fn new(transfer_syntaxes: Vec<u8>) -> TransferSyntaxItem {
        TransferSyntaxItem {
            reserved: 0u8,
            length: transfer_syntaxes.len().try_into().unwrap_or_default(),
            transfer_syntaxes,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// transfer syntax name fields.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The transfer syntax UIDs related to the proposed presentation context.
    pub fn transfer_syntaxes(&self) -> &Vec<u8> {
        &self.transfer_syntaxes
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.transfer_syntaxes.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.transfer_syntaxes)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<TransferSyntaxItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut transfer_syntaxes: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut transfer_syntaxes)?;

        Ok(TransferSyntaxItem {
            reserved,
            length,
            transfer_syntaxes,
        })
    }
}

impl From<TSRef> for TransferSyntaxItem {
    fn from(value: TSRef) -> Self {
        Self::new(Vec::from(value.uid().uid().as_bytes()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserInformationItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    user_data: Vec<UserPdu>,
}

impl UserInformationItem {
    /// The type of this PDU, `PduType::UserInformationItem`.
    pub fn pdu_type(&self) -> PduType {
        PduType::UserInformationItem
    }

    pub fn new(user_data: Vec<UserPdu>) -> UserInformationItem {
        let length = u16::try_from(user_data.iter().map(UserPdu::byte_size).sum::<usize>())
            .unwrap_or_default();
        UserInformationItem {
            reserved: 0u8,
            length,
            user_data,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the user
    /// data fields.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// User-data sub-items.
    pub fn user_data(&self) -> &Vec<UserPdu> {
        &self.user_data
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.user_data.iter().map(UserPdu::byte_size).sum::<usize>()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        for pdu in &self.user_data {
            pdu.write(&mut dataset)?;
        }

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<UserInformationItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut user_data: Vec<UserPdu> = Vec::new();
        let mut bytes_left = length as usize;
        while bytes_left > 0 {
            let pdu = UserPdu::read(&mut dataset)?;
            bytes_left -= pdu.byte_size();
            user_data.push(pdu);
        }

        Ok(UserInformationItem {
            reserved,
            length,
            user_data,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaxLengthItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    max_length: u32,
}

impl MaxLengthItem {
    /// The type of this PDU, `UserSubPduType::MaxLengthItem`.
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::MaxLengthItem
    }

    pub fn new(max_length: u32) -> MaxLengthItem {
        let length: u16 = size_of::<u32>().try_into().unwrap_or_default(); // max_length

        MaxLengthItem {
            reserved: 0u8,
            length,
            max_length,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the user
    /// data fields. This should be a fixed value of 4.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The maximum length.
    pub fn max_length(&self) -> u32 {
        self.max_length
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u32>() // max_length
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.max_length.to_be_bytes())?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<MaxLengthItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let max_length = u32::from_be_bytes(buf);

        Ok(MaxLengthItem {
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
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::ImplementationClassUIDItem
    }

    pub fn new(impl_class_uid: Vec<u8>) -> ImplementationClassUIDItem {
        ImplementationClassUIDItem {
            reserved: 0u8,
            length: impl_class_uid.len().try_into().unwrap_or_default(),
            impl_class_uid,
        }
    }

    /// The number of bytes in the following Implementation Class UID field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The Implementation Class UID.
    pub fn impl_class_uid(&self) -> &Vec<u8> {
        &self.impl_class_uid
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.impl_class_uid.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.impl_class_uid)?;

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<ImplementationClassUIDItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut impl_class_uid: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut impl_class_uid)?;

        Ok(ImplementationClassUIDItem {
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
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::AsyncOperationsWindowItem
    }

    pub fn new(max_ops_invoked: u16, max_ops_performed: u16) -> AsyncOperationsWindowItem {
        let length: usize = size_of::<u16>() // max_ops_invoked
            + size_of::<u16>(); // max_ops_performed

        AsyncOperationsWindowItem {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            max_ops_invoked,
            max_ops_performed,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Maximum Number Operations Performed field. This should be a fixed value of 4.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Maximum Number of Operations Invoked.
    pub fn max_ops_invoked(&self) -> u16 {
        self.max_ops_invoked
    }

    /// Maximum Number of Operations Performed.
    pub fn max_ops_performed(&self) -> u16 {
        self.max_ops_performed
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // max_ops_invoked
            + size_of::<u16>() // max_ops_performed
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.max_ops_invoked.to_be_bytes())?;
        dataset.write_all(&self.max_ops_performed.to_be_bytes())?;

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<AsyncOperationsWindowItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let max_ops_invoked = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let max_ops_performed = u16::from_be_bytes(buf);

        Ok(AsyncOperationsWindowItem {
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
    scu_role: u8,
    scp_role: u8,
}

impl RoleSelectionItem {
    /// The type of this PDU, `UserSubPduType::RoleSelectionItem`.
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::RoleSelectionItem
    }

    pub fn new(sop_class_uid: Vec<u8>, scu_role: u8, scp_role: u8) -> RoleSelectionItem {
        let length: usize = size_of::<u16>() // sop_class_uid_length
            + sop_class_uid.len()
            + size_of::<u8>() // scu_role
            + size_of::<u8>(); // scp_role

        RoleSelectionItem {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            sop_class_uid_length: sop_class_uid.len().try_into().unwrap_or_default(),
            sop_class_uid,
            scu_role,
            scp_role,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the SCP
    /// Role field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The number of bytes in the SOP Class UID field.
    pub fn sop_class_uid_length(&self) -> u16 {
        self.sop_class_uid_length
    }

    /// The SOP Class UID field.
    pub fn sop_class_uid(&self) -> &Vec<u8> {
        &self.sop_class_uid
    }

    /// Support for the SCU role.
    ///
    /// 0: non-support of the SCU role.
    /// 1: support of the SCU role.
    pub fn scu_role(&self) -> u8 {
        self.scu_role
    }

    /// Support for the SCP role.
    ///
    /// 0: non-support of the SCP role.
    /// 1: support of the SCP role.
    pub fn scp_role(&self) -> u8 {
        self.scp_role
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // sop_class_uid_length
            + self.sop_class_uid.len()
            + size_of::<u8>() // scu_role
            + size_of::<u8>() // scp_role
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let mut buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;

        buf[0] = self.scu_role;
        buf[1] = self.scp_role;
        dataset.write_all(&buf)?;

        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<RoleSelectionItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let sop_class_uid_length = u16::from_be_bytes(buf);
        let mut sop_class_uid: Vec<u8> = vec![0u8; sop_class_uid_length.into()];
        dataset.read_exact(&mut sop_class_uid)?;

        dataset.read_exact(&mut buf)?;
        let scu_role = buf[0];
        let scp_role = buf[1];

        Ok(RoleSelectionItem {
            reserved,
            length,
            sop_class_uid_length,
            sop_class_uid,
            scu_role,
            scp_role,
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
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::ImplementationVersionNameItem
    }

    pub fn new(impl_ver_name: Vec<u8>) -> ImplementationVersionNameItem {
        ImplementationVersionNameItem {
            reserved: 0u8,
            length: impl_ver_name.len().try_into().unwrap_or_default(),
            impl_ver_name,
        }
    }

    /// The number of bytes in the Implementation Version Name field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Implementation Version Name.
    pub fn impl_ver_name(&self) -> &Vec<u8> {
        &self.impl_ver_name
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.impl_ver_name.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.impl_ver_name)?;

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<ImplementationVersionNameItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut impl_ver_name: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut impl_ver_name)?;

        Ok(ImplementationVersionNameItem {
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
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::SOPClassExtendedNegotiationItem
    }

    pub fn new(
        sop_class_uid: Vec<u8>,
        service_class_app_info: Vec<u8>,
    ) -> SOPClassExtendedNegotiationItem {
        let length: usize = size_of::<u16>() // sop_class_uid_length
            + sop_class_uid.len()
            + service_class_app_info.len();

        SOPClassExtendedNegotiationItem {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            sop_class_uid_length: sop_class_uid.len().try_into().unwrap_or_default(),
            sop_class_uid,
            service_class_app_info,
        }
    }

    /// The number of bytes from the first byte of the following field the last byte of the Service
    /// Class Application Information field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The number of bytes in the SOP Class UID field.
    pub fn sop_class_length(&self) -> u16 {
        self.sop_class_uid_length
    }

    /// The SOP Class UID field.
    pub fn sop_class_uid(&self) -> &Vec<u8> {
        &self.sop_class_uid
    }

    /// The Service Class Application Info field.
    pub fn service_class_app_info(&self) -> &Vec<u8> {
        &self.service_class_app_info
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // sop_class_uid_length
            + self.sop_class_uid.len()
            + self.service_class_app_info.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;
        dataset.write_all(&self.service_class_app_info)?;

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<SOPClassExtendedNegotiationItem, DimseError> {
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

        Ok(SOPClassExtendedNegotiationItem {
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
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::SOPClassCommonExtendedNegotiationItem
    }

    pub fn new(
        sop_class_uid: Vec<u8>,
        service_class_uid: Vec<u8>,
        rel_gen_sop_classes: Vec<RelatedGeneralSOPClassUID>,
    ) -> SOPClassCommonExtendedNegotiationItem {
        let rel_gen_sop_classes_length: usize = rel_gen_sop_classes
            .iter()
            .map(|r| r.byte_size())
            .sum::<usize>();

        let length: usize = size_of::<u16>() // sop_class_uid_length
            + sop_class_uid.len()
            + size_of::<u16>() // service_class_uid_length
            + service_class_uid.len()
            + size_of::<u16>() // rel_gen_sop_classes_length
            + rel_gen_sop_classes_length;

        // zero-length for version 0 of this sub-item definition
        let reserved: Vec<u8> = Vec::with_capacity(0);

        SOPClassCommonExtendedNegotiationItem {
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
    pub fn version(&self) -> u8 {
        self.version
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reserved field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The number of bytes in the SOP Class UID field.
    pub fn sop_class_length(&self) -> u16 {
        self.sop_class_uid_length
    }

    /// The SOP Class UID field.
    pub fn sop_class_uid(&self) -> &Vec<u8> {
        &self.sop_class_uid
    }

    /// The number of bytes in the Service Class UID field.
    pub fn service_class_length(&self) -> u16 {
        self.service_class_uid_length
    }

    /// The Service Class UID field.
    pub fn service_class_uid(&self) -> &Vec<u8> {
        &self.service_class_uid
    }

    /// The number of bytes in the Related General SOP Class Identification field. May be zero if
    /// that field is not present.
    pub fn rel_gen_sop_class_length(&self) -> u16 {
        self.rel_gen_sop_classes_length
    }

    /// The Related General SOP Class Identification fields.
    pub fn rel_gen_sop_classes(&self) -> &Vec<RelatedGeneralSOPClassUID> {
        &self.rel_gen_sop_classes
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // version
            + size_of::<u16>() // length
            + size_of::<u16>() // sop_class_uid_length
            + self.sop_class_uid.len()
            + size_of::<u16>() // service_class_uid_length
            + self.service_class_uid.len()
            + size_of::<u16>() // rel_gen_sop_classes_length
            + self.rel_gen_sop_classes.iter().map(|r| r.byte_size()).sum::<usize>()
            + self.reserved.len()
    }

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

    pub fn read<R: Read>(
        mut dataset: R,
        version: u8,
    ) -> Result<SOPClassCommonExtendedNegotiationItem, DimseError> {
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

        Ok(SOPClassCommonExtendedNegotiationItem {
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
    pub fn new(rel_gen_sop_class: Vec<u8>) -> RelatedGeneralSOPClassUID {
        RelatedGeneralSOPClassUID {
            length: rel_gen_sop_class.len().try_into().unwrap_or_default(),
            rel_gen_sop_class,
        }
    }

    /// The number of bytes in the Related General SOP Class UID field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The Related General SOP Class UID field.
    pub fn rel_gen_sop_class(&self) -> &Vec<u8> {
        &self.rel_gen_sop_class
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u16>() // length
            + self.rel_gen_sop_class.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.rel_gen_sop_class)?;
        Ok(())
    }

    pub fn read<R: Read>(mut dataset: R) -> Result<RelatedGeneralSOPClassUID, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut rel_gen_sop_class: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut rel_gen_sop_class)?;

        Ok(RelatedGeneralSOPClassUID {
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
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::UserIdentityItem
    }

    pub fn new(
        identity_type: u8,
        pos_rsp_req: u8,
        pri_value: Vec<u8>,
        sec_value: Vec<u8>,
    ) -> UserIdentityItem {
        let length: usize = size_of::<u8>() // identity_type
            + size_of::<u8>() // pos_rsp_req
            + size_of::<u16>() // pri_length
            + pri_value.len()
            + size_of::<u16>() // sec_length
            + sec_value.len();

        UserIdentityItem {
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
    pub fn identity_type(&self) -> u8 {
        self.identity_type
    }

    /// Positive response requested.
    ///
    /// 0 - no response requested
    /// 1 - positive response requested
    pub fn pos_rsp_req(&self) -> u8 {
        self.pos_rsp_req
    }

    /// The length of the primary field.
    pub fn pri_length(&self) -> u16 {
        self.pri_length
    }

    /// The primary field value.
    pub fn pri_value(&self) -> &Vec<u8> {
        &self.pri_value
    }

    /// The length of the secondary field. This field should only be non-zero if the identity type
    /// is 2.
    pub fn sec_length(&self) -> u16 {
        self.sec_length
    }

    /// The secondary field. Only present if the identity type is 2.
    pub fn sec_value(&self) -> &Vec<u8> {
        &self.sec_value
    }

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

    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<UserIdentityItem, DimseError> {
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

        Ok(UserIdentityItem {
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
    pub fn pdu_type(&self) -> UserPduType {
        UserPduType::UserIdentityNegotiationItem
    }

    pub fn new(server_rsp: Vec<u8>) -> UserIdentityNegotiationItem {
        let length: usize = size_of::<u16>() // server_rsp_length
            + server_rsp.len();

        UserIdentityNegotiationItem {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            server_rsp_length: server_rsp.len().try_into().unwrap_or_default(),
            server_rsp,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// final field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Server response length. This field should only be non-zero if the identity type is 3 or 4.
    pub fn server_rsp_length(&self) -> u16 {
        self.server_rsp_length
    }

    /// Server response. If the identity type is 3 then this will be the Kerberos Service ticket
    /// encoded in accordance with RFC-1510. If the identity type is 4 then this will be the SAML
    /// response.
    pub fn server_rsp(&self) -> &Vec<u8> {
        &self.server_rsp
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + size_of::<u16>() // server_rsp_length
            + self.server_rsp.len()
    }

    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;
        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.server_rsp_length.to_be_bytes())?;
        dataset.write_all(&self.server_rsp)?;
        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: R,
        reserved: u8,
    ) -> Result<UserIdentityNegotiationItem, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let server_rsp_length = u16::from_be_bytes(buf);
        let mut server_rsp: Vec<u8> = vec![0u8; server_rsp_length.into()];
        dataset.read_exact(&mut server_rsp)?;

        Ok(UserIdentityNegotiationItem {
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

    use crate::dimse::{pdus::UserPduType, AeTitle};

    use super::{
        Abort, AbstractSyntaxItem, ApplicationContextItem, AssocAC, AssocACPresentationContext,
        AssocRJ, AssocRQ, AssocRQPresentationContext, AsyncOperationsWindowItem,
        ImplementationClassUIDItem, ImplementationVersionNameItem, MaxLengthItem, Pdu, PduType,
        PresentationDataItem, PresentationDataValue, RelatedGeneralSOPClassUID, ReleaseRP,
        ReleaseRQ, RoleSelectionItem, SOPClassCommonExtendedNegotiationItem,
        SOPClassExtendedNegotiationItem, TransferSyntaxItem, UserIdentityItem,
        UserIdentityNegotiationItem, UserInformationItem, UserPdu,
    };

    /// Creates a dummy AssocRQ populated
    fn create_assoc_rq() -> AssocRQ {
        let ae_dest: AeTitle = "AE_DEST".as_bytes().try_into().expect("create ae_dest");
        let ae_source: AeTitle = "AE_SOURCE".as_bytes().try_into().expect("create ae_source");
        let app_context_name: Vec<u8> = "My Sever App Context v1.1".into();
        let pres_ctxs: Vec<AssocRQPresentationContext> = vec![
            AssocRQPresentationContext::new(
                4u8,
                AbstractSyntaxItem::new("1.2.3.4".into()),
                vec![
                    TransferSyntaxItem::new("2.3.4.5".into()),
                    TransferSyntaxItem::new("3.4.5.6.7".into()),
                ],
            ),
            AssocRQPresentationContext::new(
                4u8,
                AbstractSyntaxItem::new("2.3.4.5.6".into()),
                vec![
                    TransferSyntaxItem::new("2.3.4.5".into()),
                    TransferSyntaxItem::new("3.4.5.6.7".into()),
                ],
            ),
        ];
        let user_data: Vec<UserPdu> = vec![
            UserPdu::MaxLengthItem(MaxLengthItem::new(101)),
            UserPdu::UserIdentityItem(UserIdentityItem::new(1, 2, vec![3, 4, 5], vec![6, 7, 8, 9])),
        ];
        let user_info = UserInformationItem::new(user_data);

        let app_ctx = ApplicationContextItem::new(app_context_name);
        AssocRQ::new(
            ae_dest.into(),
            ae_source.into(),
            app_ctx,
            pres_ctxs,
            user_info,
        )
    }

    /// Writes a PDU to an in-memory buffer, reads that buffer into a PDU, and asserts the result
    /// is equal to the input.
    fn assert_pdu_roundtrip(pdu: Pdu) {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        pdu.write(&mut cursor).expect("unable to write pdu");

        // reset the position so reading from it starts at the beginning
        cursor.set_position(0);

        let roundtrip_pdu = Pdu::read(&mut cursor).expect("unable to read pdu");
        assert_eq!(roundtrip_pdu, pdu);
    }

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
    fn test_assoc_rq_roundtrip() {
        let pdu = create_assoc_rq();
        assert_pdu_roundtrip(Pdu::AssocRQ(pdu));
    }

    #[test]
    fn test_assoc_ac_roundtrip() {
        let assrq_pdu = create_assoc_rq();

        let pdu = AssocAC::new(
            assrq_pdu.called_ae().to_owned(),
            assrq_pdu.calling_ae().to_owned(),
            assrq_pdu.reserved_3().to_owned(),
            assrq_pdu.app_ctx().to_owned(),
            vec![
                AssocACPresentationContext::new(
                    4u8,
                    0u8,
                    TransferSyntaxItem::new("2.3.4.5".into()),
                ),
                AssocACPresentationContext::new(
                    4u8,
                    0u8,
                    TransferSyntaxItem::new("3.4.5.6.7".into()),
                ),
            ],
            assrq_pdu.user_info().to_owned(),
        );

        assert_pdu_roundtrip(Pdu::AssocAC(pdu));
    }

    #[test]
    fn test_assoc_rj_roundtrip() {
        let pdu = AssocRJ::new(1u8, 1u8, 1u8);
        assert_pdu_roundtrip(Pdu::AssocRJ(pdu));
    }

    #[test]
    fn test_release_rq_roundtrip() {
        let pdu = ReleaseRQ::new();
        assert_pdu_roundtrip(Pdu::ReleaseRQ(pdu));
    }

    #[test]
    fn test_release_rp_roundtrip() {
        let pdu = ReleaseRP::new();
        assert_pdu_roundtrip(Pdu::ReleaseRP(pdu));
    }

    #[test]
    fn test_abort_roundtrip() {
        let pdu = Abort::new(1u8, 1u8);
        assert_pdu_roundtrip(Pdu::Abort(pdu));
    }

    #[test]
    // Ignoring because PresentationDataItem does not round trip now that it's partially managed
    // instead of fully.
    #[ignore]
    fn test_pres_data_item_roundtrip() {
        let pres_data_vals = vec![
            PresentationDataValue::new(1u8, 1u8, vec![1, 2, 3, 4]),
            PresentationDataValue::new(2u8, 2u8, vec![2, 3, 4, 5, 6]),
        ];
        let pres_data = PresentationDataItem::new(pres_data_vals);
        assert_pdu_roundtrip(Pdu::PresentationDataItem(pres_data));
    }

    #[test]
    fn test_app_ctx_roundtrip() {
        let pdu = ApplicationContextItem::new(vec![3, 2, 1, 5, 4]);
        assert_pdu_roundtrip(Pdu::ApplicationContextItem(pdu));
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
    fn test_pdu_type_roundtrip() {
        assert_eq!(
            PduType::AssocRQ,
            (u8::from(&PduType::AssocRQ)).try_into().unwrap()
        );
        assert_eq!(
            PduType::AssocAC,
            (u8::from(&PduType::AssocAC)).try_into().unwrap()
        );
        assert_eq!(
            PduType::AssocRJ,
            (u8::from(&PduType::AssocRJ)).try_into().unwrap()
        );

        assert_eq!(
            PduType::PresentationDataItemPartial,
            (u8::from(&PduType::PresentationDataItemPartial))
                .try_into()
                .unwrap()
        );

        assert_eq!(
            PduType::ReleaseRQ,
            (u8::from(&PduType::ReleaseRQ)).try_into().unwrap()
        );
        assert_eq!(
            PduType::ReleaseRP,
            (u8::from(&PduType::ReleaseRP)).try_into().unwrap()
        );
        assert_eq!(
            PduType::Abort,
            (u8::from(&PduType::Abort)).try_into().unwrap()
        );

        assert_eq!(
            PduType::ApplicationContextItem,
            (u8::from(&PduType::ApplicationContextItem))
                .try_into()
                .unwrap()
        );

        assert_eq!(
            PduType::AssocRQPresentationContext,
            (u8::from(&PduType::AssocRQPresentationContext))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            PduType::AssocACPresentationContext,
            (u8::from(&PduType::AssocACPresentationContext))
                .try_into()
                .unwrap()
        );

        assert_eq!(
            PduType::AbstractSyntaxItem,
            (u8::from(&PduType::AbstractSyntaxItem)).try_into().unwrap()
        );
        assert_eq!(
            PduType::TransferSyntaxItem,
            (u8::from(&PduType::TransferSyntaxItem)).try_into().unwrap()
        );
        assert_eq!(
            PduType::UserInformationItem,
            (u8::from(&PduType::UserInformationItem))
                .try_into()
                .unwrap()
        );

        assert_eq!(
            UserPduType::MaxLengthItem,
            (u8::from(&UserPduType::MaxLengthItem)).try_into().unwrap()
        );

        assert_eq!(
            UserPduType::ImplementationClassUIDItem,
            (u8::from(&UserPduType::ImplementationClassUIDItem))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserPduType::AsyncOperationsWindowItem,
            (u8::from(&UserPduType::AsyncOperationsWindowItem))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserPduType::RoleSelectionItem,
            (u8::from(&UserPduType::RoleSelectionItem))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserPduType::ImplementationVersionNameItem,
            (u8::from(&UserPduType::ImplementationVersionNameItem))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserPduType::SOPClassExtendedNegotiationItem,
            (u8::from(&UserPduType::SOPClassExtendedNegotiationItem))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserPduType::SOPClassCommonExtendedNegotiationItem,
            (u8::from(&UserPduType::SOPClassCommonExtendedNegotiationItem))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserPduType::UserIdentityItem,
            (u8::from(&UserPduType::UserIdentityItem))
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserPduType::UserIdentityNegotiationItem,
            (u8::from(&UserPduType::UserIdentityNegotiationItem))
                .try_into()
                .unwrap()
        );
    }
}
