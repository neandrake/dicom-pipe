//! Protocol Data Units for DIMSE. Part 8, Chapter 9.3
//!
//! PDU headers are encoded with Big Endian. The value fields are sent using the transfer syntax
//! negotiated during establishment of the association.

use std::io::{Read, Write};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PduError {
    #[error("invalid pdu type: {0:04X}")]
    InvalidPduType(u8),

    /// Wrapper around `std::io::Error`.
    #[error("i/o error reading from dataset")]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("unexpected end of byte stream")]
    UnexpectedEOF,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum PduType {
    AssocRQ = 0x01,
    AssocAC = 0x02,
    AssocRJ = 0x03,

    PresentationDataItem = 0x04,

    ReleaseRQ = 0x05,
    ReleaseRP = 0x06,
    Abort = 0x07,

    ApplicationContextItem = 0x10,

    AssocRQPresentationContext = 0x20,
    AssocACPresentationContext = 0x21,

    AbstractSyntaxItem = 0x30,
    TransferSyntaxItem = 0x40,
    UserInformationItem = 0x50,

    MaxLengthItem = 0x51,
    ImplementationClassUIDItem = 0x52,
    AsyncOperationsWindowItem = 0x53,
    RoleSelectionItem = 0x54,
    ImplementationVersionNameItem = 0x55,
    SOPClassExtendedNegotiationItem = 0x56,
    SOPClassCommonExtendedNegotiationItem = 0x57,
    UserIdentityItem = 0x58,
    UserIdentityNegotiationItem = 0x59,
}

impl TryFrom<u8> for PduType {
    type Error = PduError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(PduType::AssocRQ),
            0x02 => Ok(PduType::AssocAC),
            0x03 => Ok(PduType::AssocRJ),

            0x04 => Ok(PduType::PresentationDataItem),

            0x05 => Ok(PduType::ReleaseRQ),
            0x06 => Ok(PduType::ReleaseRP),
            0x07 => Ok(PduType::Abort),

            0x10 => Ok(PduType::ApplicationContextItem),

            0x20 => Ok(PduType::AssocRQPresentationContext),
            0x21 => Ok(PduType::AssocACPresentationContext),

            0x30 => Ok(PduType::AbstractSyntaxItem),
            0x40 => Ok(PduType::TransferSyntaxItem),
            0x50 => Ok(PduType::UserInformationItem),

            0x51 => Ok(PduType::MaxLengthItem),
            0x52 => Ok(PduType::ImplementationClassUIDItem),
            0x53 => Ok(PduType::AsyncOperationsWindowItem),
            0x54 => Ok(PduType::RoleSelectionItem),
            0x55 => Ok(PduType::ImplementationVersionNameItem),
            0x56 => Ok(PduType::SOPClassExtendedNegotiationItem),
            0x57 => Ok(PduType::SOPClassCommonExtendedNegotiationItem),
            0x58 => Ok(PduType::UserIdentityItem),
            0x59 => Ok(PduType::UserIdentityNegotiationItem),

            byte => Err(PduError::InvalidPduType(byte)),
        }
    }
}

pub enum Pdu {
    AssocRQ(AssocRQ),
    AssocAC(AssocAC),
    AssocRJ(AssocRJ),
    PresentationDataItem(PresentationDataItem),
    ReleaseRQ(ReleaseRQ),
    ReleaseRP(ReleaseRP),
    Abort(Abort),
    ApplicationContextItem(ApplicationContextItem),
    AssocRQPresentationContext(AssocRQPresentationContext),
    AssocACPresentationContext(AssocACPresentationContext),
    AbstractSyntaxItem(AbstractSyntaxItem),
    TransferSyntaxItem(TransferSyntaxItem),
    UserInformationItem(UserInformationItem),
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

#[derive(Debug)]
pub struct AssocRQ {
    length: u32,
    /// Reserved, should be zero.
    reserved_1: u8,
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
    pub fn pdu_type() -> PduType {
        PduType::AssocRQ
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

    pub fn write<W: Write>(&self, mut dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
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

    pub fn read<R: Read>(mut dataset: &mut R, reserved: u8) -> Result<AssocRQ, PduError> {
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
        let app_ctx = ApplicationContextItem::read(dataset, buf[1])?;

        let mut pres_ctxs: Vec<AssocRQPresentationContext> = Vec::new();
        dataset.read_exact(&mut buf)?;
        while buf[0] == PduType::AssocACPresentationContext as u8 {
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

#[derive(Debug)]
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
    pub fn pdu_type() -> PduType {
        PduType::AssocAC
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

    pub fn write<W: Write>(&self, mut dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
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

    pub fn read<R: Read>(mut dataset: &mut R, reserved: u8) -> Result<AssocAC, PduError> {
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
        let app_ctx = ApplicationContextItem::read(dataset, buf[1])?;

        let mut pres_ctxs: Vec<AssocACPresentationContext> = Vec::new();
        dataset.read_exact(&mut buf)?;
        while buf[0] == PduType::AssocACPresentationContext as u8 {
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

#[derive(Debug)]
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
    pub fn pdu_type() -> PduType {
        PduType::AssocRJ
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 4] = [self.reserved_2, self.result, self.source, self.reason];
        dataset.write_all(&buf)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<AssocRJ, PduError> {
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

#[derive(Debug)]
pub struct ReleaseRQ {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

impl ReleaseRQ {
    /// The type of this PDU, `PduType::ReleaseRQ`.
    pub fn pdu_type() -> PduType {
        PduType::ReleaseRQ
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<ReleaseRQ, PduError> {
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

#[derive(Debug)]
pub struct ReleaseRP {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

impl ReleaseRP {
    /// The type of this PDU, `PduType::ReleaseRP`.
    pub fn pdu_type() -> PduType {
        PduType::ReleaseRP
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<ReleaseRP, PduError> {
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

#[derive(Debug)]
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
    pub fn pdu_type() -> PduType {
        PduType::Abort
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 4] = [self.reserved_2, self.reserved_3, self.source, self.reason];
        dataset.write_all(&buf)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<Abort, PduError> {
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

#[derive(Debug)]
pub struct PresentationDataItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u32,
    pres_data: Vec<PresentationDataValue>,
}

impl PresentationDataItem {
    /// The type of this PDU, `PduType::PresentationDataItem`.
    pub fn pdu_type() -> PduType {
        PduType::PresentationDataItem
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

    pub fn write<W: Write>(&self, mut dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        for value in &self.pres_data {
            value.write(&mut dataset)?;
        }

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: &mut R,
        reserved: u8,
    ) -> Result<PresentationDataItem, PduError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut pres_data: Vec<PresentationDataValue> = Vec::new();
        let mut pres_data_len_marker = length as usize;
        while pres_data_len_marker > 0 {
            let value = PresentationDataValue::read(&mut dataset)?;
            // The total bytes read for a single data value is 4 for the length field and then the
            // value of the length field which includes the ctx_id, msg_header, and data fields.
            pres_data_len_marker -= 4 + value.length() as usize;
            pres_data.push(value);
        }

        Ok(PresentationDataItem {
            reserved,
            length,
            pres_data,
        })
    }
}

#[derive(Debug)]
pub struct PresentationDataValue {
    length: u32,
    ctx_id: u8,
    msg_header: u8,
    data: Vec<u8>,
}

impl PresentationDataValue {
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 2] = [self.ctx_id, self.msg_header];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.data)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R) -> Result<PresentationDataValue, PduError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let msg_header = buf[1];

        let data_len = length as usize - 2;
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

#[derive(Debug)]
pub struct ApplicationContextItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    app_context_name: Vec<u8>,
}

impl ApplicationContextItem {
    /// The type of this PDU, `PduType::ApplicationContextItem`.
    pub fn pdu_type() -> PduType {
        PduType::ApplicationContextItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.app_context_name)?;

        Ok(())
    }

    pub fn read<R: Read>(
        dataset: &mut R,
        reserved: u8,
    ) -> Result<ApplicationContextItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut app_context_name: Vec<u8> = vec![0u8; length as usize];
        dataset.read_exact(&mut app_context_name)?;

        Ok(ApplicationContextItem {
            reserved,
            length,
            app_context_name,
        })
    }
}

#[derive(Debug)]
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
    pub fn pdu_type() -> PduType {
        PduType::AssocRQPresentationContext
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

    pub fn write<W: Write>(&self, mut dataset: &mut W) -> Result<(), PduError> {
        let mut buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
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
        mut dataset: &mut R,
        reserved: u8,
    ) -> Result<AssocRQPresentationContext, PduError> {
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
        let abstract_syntax = AbstractSyntaxItem::read(dataset, buf[1])?;
        // The total bytes read for AbstractSyntaxItem, 2 bytes for the type and reserved, plus an
        // additional 2 for the length field, and the value of the length field being the size of
        // the last variable-length field.
        bytes_read += buf.len();
        bytes_read += 2 + abstract_syntax.length() as usize;

        // The length of total transfer syntax list is value of this PDU's length field less the
        // number of bytes before the transfer syntaxes field. There are no fields after the
        // transfer syntaxes.
        let mut transfer_syntax_len_marker = length as usize - bytes_read;
        let mut transfer_syntaxes: Vec<TransferSyntaxItem> = Vec::new();
        while transfer_syntax_len_marker > 0 {
            dataset.read_exact(&mut buf)?;
            let transfer_syntax = TransferSyntaxItem::read(&mut dataset, buf[1])?;
            // Total bytes read is 2 for the type and reserved, plus another 2 for the length
            // field, and also the value of the length field being the size of the last
            // variable-length field.
            transfer_syntax_len_marker -= 4 + transfer_syntax.length() as usize;
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

#[derive(Debug)]
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
    pub fn pdu_type() -> PduType {
        PduType::AssocACPresentationContext
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

    pub fn write<W: Write>(&self, mut dataset: &mut W) -> Result<(), PduError> {
        let mut buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved_1];
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
        dataset: &mut R,
        reserved: u8,
    ) -> Result<AssocACPresentationContext, PduError> {
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

#[derive(Debug)]
pub struct AbstractSyntaxItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    abstract_syntax: Vec<u8>,
}

impl AbstractSyntaxItem {
    /// The type of this PDU, `PduType::AbstractSyntaxItem`.
    pub fn pdu_type() -> PduType {
        PduType::AbstractSyntaxItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.abstract_syntax)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<AbstractSyntaxItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut abstract_syntax: Vec<u8> = vec![0u8; length as usize];
        dataset.read_exact(&mut abstract_syntax)?;

        Ok(AbstractSyntaxItem {
            reserved,
            length,
            abstract_syntax,
        })
    }
}

#[derive(Debug)]
pub struct TransferSyntaxItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    transfer_syntaxes: Vec<u8>,
}

impl TransferSyntaxItem {
    /// The type of this PDU, `PduType::TransferSyntaxItem`.
    pub fn pdu_type() -> PduType {
        PduType::TransferSyntaxItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.transfer_syntaxes)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<TransferSyntaxItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut transfer_syntaxes: Vec<u8> = vec![0u8; length as usize];
        dataset.read_exact(&mut transfer_syntaxes)?;

        Ok(TransferSyntaxItem {
            reserved,
            length,
            transfer_syntaxes,
        })
    }
}

#[derive(Debug)]
pub struct UserInformationItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    user_data: Vec<u8>,
}

impl UserInformationItem {
    /// The type of this PDU, `PduType::UserInformationItem`.
    pub fn pdu_type() -> PduType {
        PduType::UserInformationItem
    }

    /// The number of bytes from the first byte of the following field to the last byte of the user
    /// data fields.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// User-data sub-items.
    pub fn user_data(&self) -> &Vec<u8> {
        &self.user_data
    }

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.user_data)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<UserInformationItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut user_data: Vec<u8> = vec![0u8; length as usize];
        dataset.read_exact(&mut user_data)?;

        Ok(UserInformationItem {
            reserved,
            length,
            user_data,
        })
    }
}

#[derive(Debug)]
pub struct MaxLengthItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    max_length: u32,
}

impl MaxLengthItem {
    /// The type of this PDU, `PduType::MaxLengthItem`.
    pub fn pdu_type() -> PduType {
        PduType::MaxLengthItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.max_length.to_be_bytes())?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<MaxLengthItem, PduError> {
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

#[derive(Debug)]
pub struct ImplementationClassUIDItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    impl_class_uid: Vec<u8>,
}

impl ImplementationClassUIDItem {
    /// The type of this PDU, `PduType::ImplementationClassUIDItem`.
    pub fn pdu_type() -> PduType {
        PduType::ImplementationClassUIDItem
    }

    /// The number of bytes in the following Implementation Class UID field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The Implementation Class UID.
    pub fn impl_class_uid(&self) -> &Vec<u8> {
        &self.impl_class_uid
    }

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.impl_class_uid)?;

        Ok(())
    }

    pub fn read<R: Read>(
        dataset: &mut R,
        reserved: u8,
    ) -> Result<ImplementationClassUIDItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut impl_class_uid: Vec<u8> = vec![0u8; length as usize];
        dataset.read_exact(&mut impl_class_uid)?;

        Ok(ImplementationClassUIDItem {
            reserved,
            length,
            impl_class_uid,
        })
    }
}

#[derive(Debug)]
pub struct AsyncOperationsWindowItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    max_ops_invoked: u16,
    max_ops_performed: u16,
}

impl AsyncOperationsWindowItem {
    /// The type of this PDU, `PduType::AsyncOperationsWindowItem`.
    pub fn pdu_type() -> PduType {
        PduType::AsyncOperationsWindowItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.max_ops_invoked.to_be_bytes())?;
        dataset.write_all(&self.max_ops_performed.to_be_bytes())?;

        Ok(())
    }

    pub fn read<R: Read>(
        dataset: &mut R,
        reserved: u8,
    ) -> Result<AsyncOperationsWindowItem, PduError> {
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

#[derive(Debug)]
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
    /// The type of this PDU, `PduType::RoleSelectionItem`.
    pub fn pdu_type() -> PduType {
        PduType::RoleSelectionItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let mut buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;

        buf[0] = self.scu_role;
        buf[1] = self.scp_role;
        dataset.write_all(&buf)?;

        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<RoleSelectionItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let sop_class_uid_length = u16::from_be_bytes(buf);
        let mut sop_class_uid: Vec<u8> = vec![0u8; sop_class_uid_length as usize];
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

#[derive(Debug)]
pub struct ImplementationVersionNameItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    impl_ver_name: Vec<u8>,
}

impl ImplementationVersionNameItem {
    /// The type of this PDU, `PduType::ImplementationVersionNameItem`.
    pub fn pdu_type() -> PduType {
        PduType::ImplementationVersionNameItem
    }

    /// The number of bytes in the Implementation Version Name field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Implementation Version Name.
    pub fn impl_ver_name(&self) -> &Vec<u8> {
        &self.impl_ver_name
    }

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.impl_ver_name)?;

        Ok(())
    }

    pub fn read<R: Read>(
        dataset: &mut R,
        reserved: u8,
    ) -> Result<ImplementationVersionNameItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut impl_ver_name: Vec<u8> = vec![0u8; length as usize];
        dataset.read_exact(&mut impl_ver_name)?;

        Ok(ImplementationVersionNameItem {
            reserved,
            length,
            impl_ver_name,
        })
    }
}

#[derive(Debug)]
pub struct SOPClassExtendedNegotiationItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    sop_class_uid_length: u16,
    sop_class_uid: Vec<u8>,
    service_class_app_info: Vec<u8>,
}

impl SOPClassExtendedNegotiationItem {
    /// The type of this PDU, `PduType::SOPClassExtendedNegotiationItem`.
    pub fn pdu_type() -> PduType {
        PduType::SOPClassExtendedNegotiationItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;
        dataset.write_all(&self.service_class_app_info)?;

        Ok(())
    }

    pub fn read<R: Read>(
        dataset: &mut R,
        reserved: u8,
    ) -> Result<SOPClassExtendedNegotiationItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let sop_class_uid_length = u16::from_be_bytes(buf);
        let mut sop_class_uid: Vec<u8> = vec![0u8; sop_class_uid_length as usize];
        dataset.read_exact(&mut sop_class_uid)?;

        let service_class_app_info_length = length - sop_class_uid_length;
        let mut service_class_app_info: Vec<u8> = vec![0u8; service_class_app_info_length as usize];
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

#[derive(Debug)]
pub struct SOPClassCommonExtendedNegotiationItem {
    version: u8,
    length: u16,
    sop_class_uid_length: u16,
    sop_class_uid: Vec<u8>,
    service_class_length: u16,
    service_class_uid: Vec<u8>,
    rel_gen_sop_class_length: u16,
    rel_gen_sop_classes: Vec<RelatedGeneralSOPClassUID>,
    reserved: Vec<u8>,
}

impl SOPClassCommonExtendedNegotiationItem {
    /// The type of this PDU, `PduType::SOPClassCommonExtendedNegotiationItem`.
    pub fn pdu_type() -> PduType {
        PduType::SOPClassCommonExtendedNegotiationItem
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
        self.service_class_length
    }

    /// The Service Class UID field.
    pub fn service_class_uid(&self) -> &Vec<u8> {
        &self.service_class_uid
    }

    /// The number of bytes in the Related General SOP Class Identification field. May be zero if
    /// that field is not present.
    pub fn rel_gen_sop_class_length(&self) -> u16 {
        self.rel_gen_sop_class_length
    }

    /// The Related General SOP Class Identification fields.
    pub fn rel_gen_sop_classes(&self) -> &Vec<RelatedGeneralSOPClassUID> {
        &self.rel_gen_sop_classes
    }

    pub fn write<W: Write>(&self, mut dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.version];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid_length.to_be_bytes())?;
        dataset.write_all(&self.sop_class_uid)?;
        dataset.write_all(&self.service_class_length.to_be_bytes())?;
        dataset.write_all(&self.rel_gen_sop_class_length.to_be_bytes())?;
        for rel_gen_sop_class in &self.rel_gen_sop_classes {
            rel_gen_sop_class.write(&mut dataset)?;
        }

        if !self.reserved.is_empty() {
            dataset.write_all(&self.reserved)?;
        }

        Ok(())
    }

    pub fn read<R: Read>(
        mut dataset: &mut R,
        version: u8,
    ) -> Result<SOPClassCommonExtendedNegotiationItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let sop_class_uid_length = u16::from_be_bytes(buf);
        let mut sop_class_uid: Vec<u8> = vec![0u8; sop_class_uid_length as usize];
        dataset.read_exact(&mut sop_class_uid)?;

        dataset.read_exact(&mut buf)?;
        let service_class_length = u16::from_be_bytes(buf);
        let service_class_uid: Vec<u8> = vec![0u8; service_class_length as usize];
        dataset.read_exact(&mut sop_class_uid)?;

        dataset.read_exact(&mut buf)?;
        let rel_gen_sop_class_length = u16::from_be_bytes(buf);

        // The rel_gen_sop_class_length field is the number of bytes in total for all
        // RelatedGeneralSOPClasUIDs. After reading one, subtract its size in bytes so
        // we know when the last byte of this PDU has been read.
        let mut rel_gen_sop_class_len_marker = rel_gen_sop_class_length as usize;
        let mut rel_gen_sop_classes: Vec<RelatedGeneralSOPClassUID> = Vec::new();
        while rel_gen_sop_class_len_marker > 0 {
            let rel_gen_sop_class = RelatedGeneralSOPClassUID::read(&mut dataset)?;
            // The number of bytes in this struct is the length field (u16) plus that many number
            // of bytes.
            let size = 2 + rel_gen_sop_class.length() as usize;
            rel_gen_sop_class_len_marker -= size;
            rel_gen_sop_classes.push(rel_gen_sop_class);
        }

        let reserved_len =
            length - sop_class_uid_length - service_class_length - rel_gen_sop_class_length;
        let reserved = if reserved_len > 0 {
            let mut reserved: Vec<u8> = vec![0u8; reserved_len as usize];
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
            service_class_length,
            service_class_uid,
            rel_gen_sop_class_length,
            rel_gen_sop_classes,
            reserved,
        })
    }
}

#[derive(Debug)]
pub struct RelatedGeneralSOPClassUID {
    length: u16,
    rel_gen_sop_class: Vec<u8>,
}

impl RelatedGeneralSOPClassUID {
    /// The number of bytes in the Related General SOP Class UID field.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The Related General SOP Class UID field.
    pub fn rel_gen_sop_class(&self) -> &Vec<u8> {
        &self.rel_gen_sop_class
    }

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.rel_gen_sop_class)?;
        Ok(())
    }

    pub fn read<R: Read>(dataset: &mut R) -> Result<RelatedGeneralSOPClassUID, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut rel_gen_sop_class: Vec<u8> = vec![0u8; length as usize];
        dataset.read_exact(&mut rel_gen_sop_class)?;

        Ok(RelatedGeneralSOPClassUID {
            length,
            rel_gen_sop_class,
        })
    }
}

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
    /// The type of this PDU, `PduType::UserIdentityItem`.
    pub fn pdu_type() -> PduType {
        PduType::UserIdentityItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
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

    pub fn read<R: Read>(dataset: &mut R, reserved: u8) -> Result<UserIdentityItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let identity_type = buf[0];
        let pos_rsp_req = buf[1];

        dataset.read_exact(&mut buf)?;
        let pri_length = u16::from_be_bytes(buf);
        let mut pri_value: Vec<u8> = vec![0u8; pri_length as usize];
        dataset.read_exact(&mut pri_value)?;

        dataset.read_exact(&mut buf)?;
        let sec_length = u16::from_be_bytes(buf);
        let mut sec_value: Vec<u8> = vec![0u8; sec_length as usize];
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

#[derive(Debug)]
pub struct UserIdentityNegotiationItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    server_rsp_length: u16,
    server_rsp: Vec<u8>,
}

impl UserIdentityNegotiationItem {
    /// The type of this PDU, `PduType::UserIdentityNegotiationItem`.
    pub fn pdu_type() -> PduType {
        PduType::UserIdentityNegotiationItem
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

    pub fn write<W: Write>(&self, dataset: &mut W) -> Result<(), PduError> {
        let buf: [u8; 2] = [Self::pdu_type() as u8, self.reserved];
        dataset.write_all(&buf)?;
        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.server_rsp_length.to_be_bytes())?;
        dataset.write_all(&self.server_rsp)?;
        Ok(())
    }

    pub fn read<R: Read>(
        dataset: &mut R,
        reserved: u8,
    ) -> Result<UserIdentityNegotiationItem, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let server_rsp_length = u16::from_be_bytes(buf);
        let mut server_rsp: Vec<u8> = vec![0u8; server_rsp_length as usize];
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
    use super::PduType;

    #[test]
    fn test_pdu_type_roundtrip() {
        assert_eq!(
            PduType::AssocRQ,
            (PduType::AssocRQ as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::AssocAC,
            (PduType::AssocAC as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::AssocRJ,
            (PduType::AssocRJ as u8).try_into().unwrap()
        );

        assert_eq!(
            PduType::PresentationDataItem,
            (PduType::PresentationDataItem as u8).try_into().unwrap()
        );

        assert_eq!(
            PduType::ReleaseRQ,
            (PduType::ReleaseRQ as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::ReleaseRP,
            (PduType::ReleaseRP as u8).try_into().unwrap()
        );
        assert_eq!(PduType::Abort, (PduType::Abort as u8).try_into().unwrap());

        assert_eq!(
            PduType::ApplicationContextItem,
            (PduType::ApplicationContextItem as u8).try_into().unwrap()
        );

        assert_eq!(
            PduType::AssocRQPresentationContext,
            (PduType::AssocRQPresentationContext as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            PduType::AssocACPresentationContext,
            (PduType::AssocACPresentationContext as u8)
                .try_into()
                .unwrap()
        );

        assert_eq!(
            PduType::AbstractSyntaxItem,
            (PduType::AbstractSyntaxItem as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::TransferSyntaxItem,
            (PduType::TransferSyntaxItem as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::UserInformationItem,
            (PduType::UserInformationItem as u8).try_into().unwrap()
        );

        assert_eq!(
            PduType::MaxLengthItem,
            (PduType::MaxLengthItem as u8).try_into().unwrap()
        );

        assert_eq!(
            PduType::ImplementationClassUIDItem,
            (PduType::ImplementationClassUIDItem as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            PduType::AsyncOperationsWindowItem,
            (PduType::AsyncOperationsWindowItem as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            PduType::RoleSelectionItem,
            (PduType::RoleSelectionItem as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::ImplementationVersionNameItem,
            (PduType::ImplementationVersionNameItem as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            PduType::SOPClassExtendedNegotiationItem,
            (PduType::SOPClassExtendedNegotiationItem as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            PduType::SOPClassCommonExtendedNegotiationItem,
            (PduType::SOPClassCommonExtendedNegotiationItem as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            PduType::UserIdentityItem,
            (PduType::UserIdentityItem as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::UserIdentityNegotiationItem,
            (PduType::UserIdentityNegotiationItem as u8)
                .try_into()
                .unwrap()
        );
    }
}
