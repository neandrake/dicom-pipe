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

    fn num_bytes(&self) -> usize {
        74 + self.app_ctx.num_bytes()
            + self.pres_ctxs.iter().map(|p| p.num_bytes()).sum::<usize>()
            + self.user_info.num_bytes()
    }
}

impl From<&AssocRQ> for Vec<u8> {
    fn from(value: &AssocRQ) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(AssocRQ::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(&value.version.to_be_bytes());
        bytes.extend(&value.reserved_2);
        bytes.extend(&value.called_ae);
        bytes.extend(&value.calling_ae);
        bytes.extend(&value.reserved_3);
        bytes.extend(Into::<Vec<u8>>::into(&value.app_ctx));
        bytes.extend(
            value
                .pres_ctxs
                .iter()
                .flat_map(|p| Into::<Vec<u8>>::into(p)),
        );
        bytes
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

    fn num_bytes(&self) -> usize {
        74 + self.app_ctx.num_bytes() + self.pres_ctxs.iter().map(|p| p.num_bytes()).sum::<usize>()
    }
}

impl From<&AssocAC> for Vec<u8> {
    fn from(value: &AssocAC) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(AssocAC::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.version.to_be_bytes());
        bytes.extend(value.reserved_2);
        bytes.extend(value.reserved_3);
        bytes.extend(value.reserved_4);
        bytes.extend(value.reserved_5);
        bytes.extend(Into::<Vec<u8>>::into(&value.app_ctx));
        bytes.extend(
            value
                .pres_ctxs
                .iter()
                .flat_map(|p| Into::<Vec<u8>>::into(p)),
        );
        bytes.extend(Into::<Vec<u8>>::into(&value.user_info));
        bytes
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

    fn num_bytes(&self) -> usize {
        10
    }
}

impl From<&AssocRJ> for Vec<u8> {
    fn from(value: &AssocRJ) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(AssocRJ::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(value.length.to_be_bytes());
        bytes.push(value.reserved_2);
        bytes.push(value.result);
        bytes.push(value.source);
        bytes.push(value.reason);
        bytes
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

    fn num_bytes(&self) -> usize {
        10
    }
}

impl From<&ReleaseRQ> for Vec<u8> {
    fn from(value: &ReleaseRQ) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(ReleaseRQ::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.reserved_2);
        bytes
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

    fn num_bytes(&self) -> usize {
        10
    }
}

impl From<&ReleaseRP> for Vec<u8> {
    fn from(value: &ReleaseRP) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(ReleaseRP::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.reserved_2);
        bytes
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

    fn num_bytes(&self) -> usize {
        10
    }
}

impl From<&Abort> for Vec<u8> {
    fn from(value: &Abort) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(Abort::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(value.length.to_be_bytes());
        bytes.push(value.reserved_2);
        bytes.push(value.reserved_3);
        bytes.push(value.source);
        bytes.push(value.reason);
        bytes
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

    fn num_bytes(&self) -> usize {
        6 + self.pres_data.iter().map(|p| p.num_bytes()).sum::<usize>()
    }
}

impl From<&PresentationDataItem> for Vec<u8> {
    fn from(value: &PresentationDataItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(PresentationDataItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(
            value
                .pres_data
                .iter()
                .flat_map(|p| Into::<Vec<u8>>::into(p)),
        );
        bytes
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

    fn num_bytes(&self) -> usize {
        6 + self.data.len()
    }
}

impl From<&PresentationDataValue> for Vec<u8> {
    fn from(value: &PresentationDataValue) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.extend(value.length.to_be_bytes());
        bytes.push(value.ctx_id);
        bytes.push(value.msg_header);
        bytes.extend(&value.data);
        bytes
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

    fn num_bytes(&self) -> usize {
        4 + self.app_context_name.len()
    }
}

impl From<&ApplicationContextItem> for Vec<u8> {
    fn from(value: &ApplicationContextItem) -> Self {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(ApplicationContextItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(&value.length.to_be_bytes());
        bytes.extend(&value.app_context_name);
        bytes
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

    fn num_bytes(&self) -> usize {
        8 + self.abstract_syntax.num_bytes()
            + self
                .transfer_syntaxes()
                .iter()
                .map(|t| t.num_bytes())
                .sum::<usize>()
    }
}

impl From<&AssocRQPresentationContext> for Vec<u8> {
    fn from(value: &AssocRQPresentationContext) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(AssocRQPresentationContext::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(value.length.to_be_bytes());
        bytes.push(value.ctx_id);
        bytes.push(value.reserved_2);
        bytes.push(value.reserved_3);
        bytes.push(value.reserved_4);
        bytes.extend(Into::<Vec<u8>>::into(&value.abstract_syntax));
        bytes
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

    fn num_bytes(&self) -> usize {
        8 + self.transfer_syntax.num_bytes()
    }
}

impl From<&AssocACPresentationContext> for Vec<u8> {
    fn from(value: &AssocACPresentationContext) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(AssocACPresentationContext::pdu_type() as u8);
        bytes.push(value.reserved_1);
        bytes.extend(value.length.to_be_bytes());
        bytes.push(value.ctx_id);
        bytes.push(value.reserved_2);
        bytes.push(value.result);
        bytes.push(value.reserved_3);
        bytes.extend(Into::<Vec<u8>>::into(&value.transfer_syntax));
        bytes
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

    fn num_bytes(&self) -> usize {
        4 + self.abstract_syntax.len()
    }
}

impl From<&AbstractSyntaxItem> for Vec<u8> {
    fn from(value: &AbstractSyntaxItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(AbstractSyntaxItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(&value.abstract_syntax);
        bytes
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

    fn num_bytes(&self) -> usize {
        4 + self.transfer_syntaxes.len()
    }
}

impl From<&TransferSyntaxItem> for Vec<u8> {
    fn from(value: &TransferSyntaxItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(TransferSyntaxItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(&value.transfer_syntaxes);
        bytes
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

    fn num_bytes(&self) -> usize {
        4 + self.user_data.len()
    }
}

impl From<&UserInformationItem> for Vec<u8> {
    fn from(value: &UserInformationItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(UserInformationItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(&value.user_data);
        bytes
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

    fn num_bytes(&self) -> usize {
        8
    }
}

impl From<&MaxLengthItem> for Vec<u8> {
    fn from(value: &MaxLengthItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(MaxLengthItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.max_length.to_be_bytes());
        bytes
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

    fn num_bytes(&self) -> usize {
        4 + self.impl_class_uid.len()
    }
}

impl From<&ImplementationClassUIDItem> for Vec<u8> {
    fn from(value: &ImplementationClassUIDItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(ImplementationClassUIDItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(&value.impl_class_uid);
        bytes
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

    fn num_bytes(&self) -> usize {
        8
    }
}

impl From<&AsyncOperationsWindowItem> for Vec<u8> {
    fn from(value: &AsyncOperationsWindowItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(AsyncOperationsWindowItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.max_ops_invoked.to_be_bytes());
        bytes.extend(value.max_ops_performed.to_be_bytes());
        bytes
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

    fn num_bytes(&self) -> usize {
        8 + self.sop_class_uid.len()
    }
}

impl From<&RoleSelectionItem> for Vec<u8> {
    fn from(value: &RoleSelectionItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(RoleSelectionItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.sop_class_uid_length.to_be_bytes());
        bytes.extend(&value.sop_class_uid);
        bytes.push(value.scu_role);
        bytes.push(value.scp_role);
        bytes
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

    fn num_bytes(&self) -> usize {
        4 + self.impl_ver_name.len()
    }
}

impl From<&ImplementationVersionNameItem> for Vec<u8> {
    fn from(value: &ImplementationVersionNameItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(ImplementationVersionNameItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(&value.impl_ver_name);
        bytes
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

    fn num_bytes(&self) -> usize {
        6 + self.sop_class_uid.len() + self.service_class_app_info.len()
    }
}

impl From<&SOPClassExtendedNegotiationItem> for Vec<u8> {
    fn from(value: &SOPClassExtendedNegotiationItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(SOPClassExtendedNegotiationItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.sop_class_uid_length.to_be_bytes());
        bytes.extend(&value.sop_class_uid);
        bytes.extend(&value.service_class_app_info);
        bytes
    }
}

#[derive(Debug)]
pub struct SOPClassCommonExtendedNegotiationItem {
    version: u8,
    length: u16,
    sop_class_length: u16,
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
        self.sop_class_length
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

    fn num_bytes(&self) -> usize {
        10 + self.sop_class_uid().len()
            + self.service_class_uid().len()
            + self
                .rel_gen_sop_classes
                .iter()
                .map(|s| s.num_bytes())
                .sum::<usize>()
    }
}

impl From<&SOPClassCommonExtendedNegotiationItem> for Vec<u8> {
    fn from(value: &SOPClassCommonExtendedNegotiationItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(SOPClassCommonExtendedNegotiationItem::pdu_type() as u8);
        bytes.push(value.version());
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.sop_class_length.to_be_bytes());
        bytes.extend(&value.sop_class_uid);
        bytes.extend(value.service_class_length.to_be_bytes());
        bytes.extend(&value.service_class_uid);
        bytes.extend(value.rel_gen_sop_class_length.to_be_bytes());
        bytes.extend(
            value
                .rel_gen_sop_classes
                .iter()
                .flat_map(|s| Into::<Vec<u8>>::into(s))
        );
        bytes.extend(&value.reserved);
        bytes
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

    fn num_bytes(&self) -> usize {
        2 + self.rel_gen_sop_class.len()
    }
}

impl From<&RelatedGeneralSOPClassUID> for Vec<u8> {
    fn from(value: &RelatedGeneralSOPClassUID) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(&value.rel_gen_sop_class);
        bytes
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

    fn num_bytes(&self) -> usize {
        10 + self.pri_value.len() + self.sec_value.len()
    }
}

impl From<&UserIdentityItem> for Vec<u8> {
    fn from(value: &UserIdentityItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(UserIdentityItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.push(value.identity_type);
        bytes.push(value.pos_rsp_req);
        bytes.extend(value.pri_length.to_be_bytes());
        bytes.extend(&value.pri_value);
        bytes.extend(value.sec_length.to_be_bytes());
        bytes.extend(&value.sec_value);
        bytes
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

    fn num_bytes(&self) -> usize {
        6 + self.server_rsp.len()
    }
}

impl From<&UserIdentityNegotiationItem> for Vec<u8> {
    fn from(value: &UserIdentityNegotiationItem) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(value.num_bytes());
        bytes.push(UserIdentityNegotiationItem::pdu_type() as u8);
        bytes.push(value.reserved);
        bytes.extend(value.length.to_be_bytes());
        bytes.extend(value.server_rsp_length.to_be_bytes());
        bytes.extend(&value.server_rsp);
        bytes
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
