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

    SOPClassExtendedNegotiationItem = 0x56,
    SOPClassCommonExtendedNegotiationItem = 0x57,
    UserIdentityItem = 0x58,
    UserIdentityNegotiationItem = 0x59,
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

            0x56 => Ok(PduType::SOPClassExtendedNegotiationItem),
            0x57 => Ok(PduType::SOPClassCommonExtendedNegotiationItem),
            0x58 => Ok(PduType::UserIdentityItem),
            0x59 => Ok(PduType::UserIdentityNegotiationItem),

            _ => Err(()),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum UserIdentityType {
    Username = 1,
    UsernameAndPasscode = 2,
    KerberosServiceTicket = 3,
    SAMLAssertion = 4,
    JWT = 5,
}

impl TryFrom<u8> for UserIdentityType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UserIdentityType::Username),
            2 => Ok(UserIdentityType::UsernameAndPasscode),
            3 => Ok(UserIdentityType::KerberosServiceTicket),
            4 => Ok(UserIdentityType::SAMLAssertion),
            5 => Ok(UserIdentityType::JWT),

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

#[derive(Debug)]
pub struct SOPClassExtendedNegotiationItem {
    /// The type of this PDU, `PduType::SOPClassExtendedNegotiationItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field the last byte of the Service
    /// Class Application Information field.
    length: u16,

    /// The number of bytes in the SOP Class UID field.
    sop_class_uid_length: u16,

    /// The SOP Class UID field.
    sop_class_uid: Vec<u8>,

    /// The Service Class Application Info field.
    service_class_app_info: Vec<u8>,
}

#[derive(Debug)]
pub struct SOPClassCommonExtendedNegotiationItem {
    /// The type of this PDU, `PduType::SOPClassCommonExtendedNegotiationItem`.
    pdu_type: PduType,

    /// The version of this item. The current standard version is 0.
    version: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reserved field.
    length: u16,

    /// The number of bytes in the SOP Class UID field.
    sop_class_length: u16,

    /// The SOP Class UID field.
    sop_class_uid: Vec<u8>,

    /// The number of bytes in the Service Class UID field.
    service_class_length: u16,

    /// The Service Class UID field.
    service_class_uid: Vec<u8>,

    /// The number of bytes in the Related General SOP Class Identification field. May be zero if
    /// that field is not present.
    rel_gen_sop_class_length: u16,

    /// The Related General SOP Class Identification fields.
    rel_gen_sop_classes: Vec<RelatedGeneralSOPClassUID>,
}

#[derive(Debug)]
pub struct RelatedGeneralSOPClassUID {
    /// The number of bytes in the Related General SOP Class UID field.
    length: u16,

    /// The Related General SOP Class UID field.
    rel_gen_sop_class: Vec<u8>,
}

pub struct UserIdentityItem {
    /// The type of this PDU, `PduType::UserIdentityItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the last
    /// field sent.
    length: u16,

    /// The user identity type.
    identity_type: UserIdentityType,

    /// Positive response requested.
    ///
    /// 0 - no response requested
    /// 1 - positive response requested
    pos_rsp_req: u8,

    /// The length of the primary field.
    pri_length: u16,

    /// The primary field value.
    pri_value: Vec<u8>,

    /// The length of the secondary field. This field should only be non-zero if the identity type
    /// is 2, `UserIdentityType::UsernameAndPasscode`.
    sec_length: u16,

    /// The secondary field. Only present if the identity type is 2,
    /// `UserIdentityType::UsernameAndPasscode`.
    sec_value: Vec<u8>,
}

#[derive(Debug)]
pub struct UserIdentityNegotiationItem {
    /// The type of this PDU, `PduType::UserIdentityNegotiationItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// final field.
    length: u16,

    /// Server response length. This field should only be non-zero if the identity type is 3,
    /// `UserIdentityType::KerberosServiceTicket`, or 4, `UserIdentityType::SAMLAssertion`.
    server_rsp_length: u16,

    /// Server response. If the identity type is 3, `UserIdentityType::KerberosServiceTicket`, then
    /// this will be the Kerberos Service ticket encoded in accordance with RFC-1510. If the
    /// identity type is 4, `UserIdentityType::SAMLAssertion`, then this will be the SAML response.
    server_rsp: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::{PduType, UserIdentityType};

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

    #[test]
    fn test_user_identity_roundtrip() {
        assert_eq!(
            UserIdentityType::Username,
            (UserIdentityType::Username as u8).try_into().unwrap()
        );
        assert_eq!(
            UserIdentityType::UsernameAndPasscode,
            (UserIdentityType::UsernameAndPasscode as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserIdentityType::KerberosServiceTicket,
            (UserIdentityType::KerberosServiceTicket as u8)
                .try_into()
                .unwrap()
        );
        assert_eq!(
            UserIdentityType::SAMLAssertion,
            (UserIdentityType::SAMLAssertion as u8).try_into().unwrap()
        );
        assert_eq!(
            UserIdentityType::JWT,
            (UserIdentityType::JWT as u8).try_into().unwrap()
        );
    }
}
