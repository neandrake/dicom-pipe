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
    type Error = ();

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

            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct AssocRQ {
    /// The type of this PDU, `PduType::AssocRQ`.
    pdu_type: PduType,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (User Information).
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

    /// Application Context.
    app_ctx: ApplicationContextItem,

    /// Presentation Contexts, at least one.
    pres_ctxs: Vec<AssocRQPresentationContext>,

    /// User Information.
    user_info: UserInformationItem,
}

#[dervie(Debug)]
pub struct AssocAC {
    /// The type of this PDU, `PduType::AssocAC`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable items field (User Information).
    length: u32,

    /// Identifies each version of the DICOM Upper Layer protocol supported by the calling
    /// end-system. Currently this is version 1, identified with bit 0 set.
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

    /// Application Context.
    app_ctx: ApplicationContextItem,

    /// Presentation Contexts, at least one.
    pres_ctxs: Vec<AssocRQPresentationContext>,

    /// User Information.
    user_info: UserInformationItem,
}

#[derive(Debug)]
pub struct AssocRJ {
    /// The type of this PDU, `PduType::AssocRJ`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reason/Diag. field.
    length: u32,

    /// Reserved, should be zero.
    reserved_2: u8,

    /// Result.
    ///
    /// 1 - Rejected-permanent.
    /// 2 - Rejected-transient.
    result: u8,

    /// Source.
    ///
    /// 1 - DICOM Upper Layer Service User.
    /// 2 - DICOM Upper Layer Service Provider (ACSE-related function).
    /// 3 - DICOM Upper Layer Service Provider (Presentation-related function).
    source: u8,

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
    reason: u8,
}

#[derive(Debug)]
pub struct ReleaseRQ {
    /// The type of this PDU, `PduType::ReleaseRQ`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    length: u32,

    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

#[derive(Debug)]
pub struct ReleaseRP {
    /// The type of this PDU, `PduType::ReleaseRP`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    length: u32,

    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

#[derive(Debug)]
pub struct Abort {
    /// The type of this PDU, `PduType::Abort`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reason/Diag. field. This should be a fixed value of 4.
    length: u32,

    /// Reserved, should be zero.
    reserved_2: u8,

    /// Reserved, should be zero.
    reserved_3: u8,

    /// Source of the Abort.
    ///
    /// 0: DICOM Upper Layer service-user (initiated abort).
    /// 1: Reserved.
    /// 2: DICOM Upper Layer service-provider (initiated abort).
    source: u8,

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
    reason: u8,
}

#[derive(Debug)]
pub struct PresentationDataItem {
    /// The type of this PDU, `PduType::PresentationDataItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (Presentation Data values).
    length: u32,

    /// Presentation Data values.
    pres_data: Vec<>,
}

#[derive(Debug)]
pub struct PresentationDataValue {
    /// The number of bytes from the first byte of the following field to the last byte of the
    /// presentation data value field.
    length: u32,

    /// Context ID, an odd number between 1-255.
    ctx_id: u8,

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
    msg_header: u8,

    /// Presentation data, a fragment. Either a Command or a DICOM Data Set.
    data: Vec<u8>,
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
    ctx_id: u8,

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
pub struct AssocACPresentationContext {
    /// The type of this PDU, `PduType::AssocACPresentationContext`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved_1: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Transfer Syntax item.
    length: u16,

    /// Presentation context ID, an odd number between 1 and 255.
    ctx_id: u8,

    /// Reserved, should be zero.
    reserved_2: u8,

    /// Result/Reason.
    ///
    /// 0: Acceptance.
    /// 1: User-rejection.
    /// 2: No reason (provider rejection).
    /// 3: Abstract Syntax not supported (provider rejection).
    /// 4: Transfer Syntaxes not supported (provider rejection).
    result: u8,

    /// Reserved, should be zero.
    reserved_3: u8,

    /// The selected transfer syntax. When the `result` field has a value other than 0, this field
    /// shall not be significant and should be ignored. The `TransferSyntaxItem` should contain
    /// only a single transfer syntax.
    transfer_syntax: TransferSyntaxItem,
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
    abstract_syntax: Vec<u8>,
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

    /// The transfer syntax UIDs related to the proposed presentation context.
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
    /// data fields. This should be a fixed value of 4.
    length: u16,

    /// The maximum length.
    max_length: u32,
}

#[derive(Debug)]
pub struct ImplementationClassUIDItem {
    /// The type of this PDU, `PduType::ImplementationClassUIDItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes in the following Implementation Class UID field.
    length: u16,

    /// The Implementation Class UID.
    impl_class_uid: Vec<u8>,
}

#[derive(Debug)]
pub struct AsyncOperationsWindowItem {
    /// The type of this PDU, `PduType::AsyncOperationsWindowItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Maximum Number Operations Performed field. This should be a fixed value of 4.
    length: u16,

    /// Maximum Number of Operations Invoked.
    max_ops_invoked: u16,

    /// Maximum Number of Operations Performed.
    max_ops_performed: u16,
}

#[derive(Debug)]
pub struct RoleSelectionItem {
    /// The type of this PDU, `PduType::RoleSelectionItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes from the first byte of the following field to the last byte of the SCP
    /// Role field.
    length: u16,

    /// The number of bytes in the SOP Class UID field.
    uid_length: u16,

    /// The SOP Class UID field.
    sop_class_uid: Vec<u8>,

    /// Support for the SCU role.
    ///
    /// 0: non-support of the SCU role.
    /// 1: support of the SCU role.
    scu_role: u8,

    /// Support for the SCP role.
    ///
    /// 0: non-support of the SCP role.
    /// 1: support of the SCP role.
    scp_role: u8,
}

#[derive(Debug)]
pub struct ImplementationVersionNameItem {
    /// The type of this PDU, `PduType::ImplementationVersionNameItem`.
    pdu_type: PduType,

    /// Reserved, should be zero.
    reserved: u8,

    /// The number of bytes in the Implementation Version Name field.
    length: u16,

    /// Implementation Version Name.
    impl_ver_name: Vec<u8>,
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
    ///
    /// 1: Username.
    /// 2: Username and Passcode.
    /// 3: Kerberos Service Ticket.
    /// 4: SAML Assertion.
    /// 5: JSON Web Token.
    identity_type: u8,

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
    /// is 2.
    sec_length: u16,

    /// The secondary field. Only present if the identity type is 2.
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
            PduType::ReleaseRQ,
            (PduType::ReleaseRQ as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::ReleaseRP,
            (PduType::ReleaseRP as u8).try_into().unwrap()
        );
        assert_eq!(
            PduType::Abort,
            (PduType::Abort as u8).try_into().unwrap()
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
