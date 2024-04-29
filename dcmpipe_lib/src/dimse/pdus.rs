//! Protocol Data Units for DIMSE. Part 8, Chapter 9.3
//!
//! PDU headers are encoded with Big Endian. The value fields are sent using the transfer syntax
//! negotiated during establishment of the association.

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum PduType {
    AssocRQ = 0x01,
    AssocAC = 0x02,
    AssocRJ = 0x03,

    PresentationDataItem = 0x04,

    AssocReleaseReq = 0x05,
    AssocReleaseRsp = 0x06,
    AssocAbort = 0x07,

    ApplicationContextItem = 0x10,

    AssocRQPresentationContext = 0x20,
    AssocACPresentationContext = 0x21,

    AbstractSyntaxItem = 0x30,
    TransferSyntaxItem = 0x40,
    UserInformationItem = 0x50,

    MaxLengthItem = 0x51,
}

impl TryFrom<u8> for PduType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(PduType::AssocRQ),
            0x02 => Ok(PduType::AssocAC),
            0x03 => Ok(PduType::AssocRJ),

            0x04 => Ok(PduType::PresentationDataItem),

            0x05 => Ok(PduType::AssocReleaseReq),
            0x06 => Ok(PduType::AssocReleaseRsp),
            0x07 => Ok(PduType::AssocAbort),

            0x10 => Ok(PduType::ApplicationContextItem),

            0x20 => Ok(PduType::AssocRQPresentationContext),
            0x21 => Ok(PduType::AssocACPresentationContext),

            0x30 => Ok(PduType::AbstractSyntaxItem),
            0x40 => Ok(PduType::TransferSyntaxItem),
            0x50 => Ok(PduType::UserInformationItem),

            0x51 => Ok(PduType::MaxLengthItem),

            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct AssocRQ {
    /// The type of this PDU, `PduType::AssocRQ`.
    pdu_type: PduType,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field.
    length: u32,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// Identifies each version of the DICOM Upper Layer protocol supported by the calling
    /// end-system. Currently this is version 1, identified with bit 0 set.
    version: u16,

    /// Reserved, should be zeros.
    reserved_2: [u8; 2],

    /// Destination AE, 16 characters encoded with the Basic G0 Set (essentially ASCII), with
    /// leading and trailing spaces being non-significant.
    called_ae: [u8; 16],

    /// Source AE, 16 characters encoded with the Basic G0 Set (essentially ASCII), with leading
    /// and trailing spaces being non-significant.
    calling_ae: [u8; 16],

    /// Reserved, should be zeros.
    reserved_3: [u8; 32],

    /// The variable items in this PDU. It shall contain one Application Context Item, one or more
    /// Presentation Context Items, and one User Information Item.
    items: Vec<u8>,
}

#[derive(Debug)]
pub struct ApplicationContextItem {
    /// The type of this PDU, `PduType::ApplicationContextItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// application context field name.
    length: u16,

    /// A valid application context name (essentially a UID).
    app_context_name: Vec<u8>,
}

#[derive(Debug)]
pub struct AssocRQPresentationContext {
    /// The type of this PDU, `PduType::AssocRQPresentationContext`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the last
    /// transfer syntax item.
    length: u16,

    /// Presentation context ID, an odd number between 1 and 255.
    pres_context_id: u8,

    /// Reserved, should be zero.
    reserved_2: u8,

    /// Reserved, should be zero.
    reserved_3: u8,

    /// Reserved, should be zero.
    reserved_4: u8,

    /// The abstract syntax sub item.
    abstract_syntax: AbstractSyntaxItem,

    /// The transfer syntax sub-items.
    transfer_syntaxes: Vec<TransferSyntaxItem>,
}

#[derive(Debug)]
pub struct AbstractSyntaxItem {
    /// The type of this PDU, `PduType::AbstractSyntaxItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// abstract syntax name field.
    length: u16,

    /// The abstract syntax tree related to the proposed presentation context. This is essentially
    /// a UID.
    abstract_syntax_name: Vec<u8>,
}

#[derive(Debug)]
pub struct TransferSyntaxItem {
    /// The type of this PDU, `PduType::TransferSyntaxItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// transfer syntax name fields.
    length: u16,

    /// The transfer syntax names related to the proposed presentation context. This is essentially
    /// a list of UIDs.
    transfer_syntaxes: Vec<u8>,
}

#[derive(Debug)]
pub struct UserInformationItem {
    /// The type of this PDU, `PduType::UserInformationItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the user
    /// data fields.
    length: u16,

    /// User-data sub-items.
    user_data: Vec<u8>,
}

#[derive(Debug)]
pub struct MaxLengthItem {
    /// The type of this PDU, `PduType::MaxLengthItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the user
    /// data fields. This should always be 4.
    length: u16,

    /// The maximum length.
    max_length: u32,
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
            PduType::AssocReleaseReq,
            (PduType::AssocReleaseReq as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::AssocReleaseRsp,
            (PduType::AssocReleaseRsp as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::AssocAbort,
            (PduType::AssocAbort as u8).try_into().unwrap()
        );

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
    }
}
