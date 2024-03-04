use std::io::Read;

use super::pdus::{
    Abort, AbstractSyntaxItem, ApplicationContextItem, AssocAC, AssocACPresentationContext,
    AssocRJ, AssocRQ, AssocRQPresentationContext, AsyncOperationsWindowItem,
    ImplementationClassUIDItem, ImplementationVersionNameItem, MaxLengthItem, Pdu, PduError,
    PduType, PresentationDataItem, ReleaseRP, ReleaseRQ, RoleSelectionItem,
    SOPClassCommonExtendedNegotiationItem, SOPClassExtendedNegotiationItem, TransferSyntaxItem,
    UserIdentityItem, UserIdentityNegotiationItem, UserInformationItem,
};

pub struct PduParser<R: Read> {
    dataset: R,
}

impl<R: Read> PduParser<R> {
    pub fn read_pdu(&mut self) -> Result<Pdu, PduError> {
        let mut buf: [u8; 2] = [0u8; 2];
        self.dataset
            .read_exact(&mut buf)
            .map_err(|e| PduError::IOError { source: e })?;

        let pdu_type: PduType = TryFrom::try_from(buf[0])?;

        let byte1 = buf[1];

        let pdu = match pdu_type {
            PduType::AssocRQ => Pdu::AssocRQ(AssocRQ::read(&mut self.dataset, byte1)?),
            PduType::AssocAC => Pdu::AssocAC(AssocAC::read(&mut self.dataset, byte1)?),
            PduType::AssocRJ => Pdu::AssocRJ(AssocRJ::read(&mut self.dataset, byte1)?),
            PduType::PresentationDataItem => {
                Pdu::PresentationDataItem(PresentationDataItem::read(&mut self.dataset, byte1)?)
            }
            PduType::ReleaseRQ => Pdu::ReleaseRQ(ReleaseRQ::read(&mut self.dataset, byte1)?),
            PduType::ReleaseRP => Pdu::ReleaseRP(ReleaseRP::read(&mut self.dataset, byte1)?),
            PduType::Abort => Pdu::Abort(Abort::read(&mut self.dataset, byte1)?),
            PduType::ApplicationContextItem => {
                Pdu::ApplicationContextItem(ApplicationContextItem::read(&mut self.dataset, byte1)?)
            }
            PduType::AssocRQPresentationContext => Pdu::AssocRQPresentationContext(
                AssocRQPresentationContext::read(&mut self.dataset, byte1)?,
            ),
            PduType::AssocACPresentationContext => Pdu::AssocACPresentationContext(
                AssocACPresentationContext::read(&mut self.dataset, byte1)?,
            ),
            PduType::AbstractSyntaxItem => {
                Pdu::AbstractSyntaxItem(AbstractSyntaxItem::read(&mut self.dataset, byte1)?)
            }
            PduType::TransferSyntaxItem => {
                Pdu::TransferSyntaxItem(TransferSyntaxItem::read(&mut self.dataset, byte1)?)
            }
            PduType::UserInformationItem => {
                Pdu::UserInformationItem(UserInformationItem::read(&mut self.dataset, byte1)?)
            }
            PduType::MaxLengthItem => {
                Pdu::MaxLengthItem(MaxLengthItem::read(&mut self.dataset, byte1)?)
            }
            PduType::ImplementationClassUIDItem => Pdu::ImplementationClassUIDItem(
                ImplementationClassUIDItem::read(&mut self.dataset, byte1)?,
            ),
            PduType::AsyncOperationsWindowItem => Pdu::AsyncOperationsWindowItem(
                AsyncOperationsWindowItem::read(&mut self.dataset, byte1)?,
            ),
            PduType::RoleSelectionItem => {
                Pdu::RoleSelectionItem(RoleSelectionItem::read(&mut self.dataset, byte1)?)
            }
            PduType::ImplementationVersionNameItem => Pdu::ImplementationVersionNameItem(
                ImplementationVersionNameItem::read(&mut self.dataset, byte1)?,
            ),
            PduType::SOPClassExtendedNegotiationItem => Pdu::SOPClassExtendedNegotiationItem(
                SOPClassExtendedNegotiationItem::read(&mut self.dataset, byte1)?,
            ),
            PduType::SOPClassCommonExtendedNegotiationItem => {
                Pdu::SOPClassCommonExtendedNegotiationItem(
                    SOPClassCommonExtendedNegotiationItem::read(&mut self.dataset, byte1)?,
                )
            }
            PduType::UserIdentityItem => {
                Pdu::UserIdentityItem(UserIdentityItem::read(&mut self.dataset, byte1)?)
            }
            PduType::UserIdentityNegotiationItem => Pdu::UserIdentityNegotiationItem(
                UserIdentityNegotiationItem::read(&mut self.dataset, byte1)?,
            ),
        };

        Ok(pdu)
    }
}
