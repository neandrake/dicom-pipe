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

use std::{
    collections::{HashMap, HashSet},
    io::{Read, Write},
};

use crate::{
    core::{
        charset::DEFAULT_CHARACTER_SET,
        dcmobject::DicomRoot,
        defn::{dcmdict::DicomDictionary, ts::TSRef, uid::UIDRef},
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY, transfer_syntaxes::ImplicitVRLittleEndian,
        uids::DICOMApplicationContextName,
    },
    dimse::{
        assoc::DimseMsg,
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                Abort, AssocAC, AssocACPresentationContext, AssocRQ, ReleaseRP, TransferSyntaxItem,
                UserInformationItem,
            },
            pduiter::{read_next_pdu, PduIterItem, PresDataIter},
            userpdus::{AsyncOperationsWindowItem, MaxLengthItem, RoleSelectionItem},
            Pdu, UserPdu,
        },
        Syntax,
    },
};

pub struct ServiceAssoc {
    /* Fields configured by this SCU. */
    _id: usize,
    host_ae: String,
    accept_aets: HashSet<String>,
    supported_abs: HashSet<UIDRef>,
    supported_ts: HashSet<TSRef>,
    my_user_data: Vec<UserPdu>,

    /* Fields negotiated with other SCU. */
    their_user_data: Vec<UserPdu>,
    negotiated_pres_ctx: HashMap<u8, AssocACPresentationContext>,
}

impl ServiceAssoc {
    /// Write the given PDU to the given writer.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the writer or flushing the writer.
    pub fn write_pdu<W: Write>(&self, pdu: &Pdu, mut writer: &mut W) -> Result<(), AssocError> {
        pdu.write(&mut writer).map_err(AssocError::error)?;
        writer
            .flush()
            .map_err(|e| AssocError::error(DimseError::IOError(e)))?;
        Ok(())
    }

    /// Retrieve the accepted presentation context by the given context ID.
    #[must_use]
    pub fn get_pres_ctx(&self, ctx_id: u8) -> Option<&AssocACPresentationContext> {
        self.negotiated_pres_ctx.get(&ctx_id)
    }

    /// Retrieve the accepted presentation context and its negotiated transfer syntax, by the given
    /// context ID.
    ///
    /// # Errors
    /// - `AssocError` may occur if the negotiated presentation context cannot be resolved, or if a
    /// known transfer syntax for it cannot be resolved.
    pub fn get_pres_ctx_and_ts(
        &self,
        ctx_id: u8,
    ) -> Result<(&AssocACPresentationContext, TSRef), AssocError> {
        let Some(pres_ctx) = self.get_pres_ctx(ctx_id) else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                "Negotiated Presentation Context ID not found: {ctx_id}"
            ))));
        };

        let ts = String::try_from(&Syntax(pres_ctx.transfer_syntax().transfer_syntaxes()))
            .ok()
            .and_then(|v| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(&v))
            .ok_or_else(|| {
                AssocError::ab_failure(DimseError::GeneralError(
                    "Failed to resolve transfer syntax".to_string(),
                ))
            })?;

        Ok((pres_ctx, ts))
    }

    #[must_use]
    pub fn get_pdu_max_snd_size(&self) -> usize {
        for user_pdu in &self.their_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return usize::try_from(mli.max_length()).unwrap_or_default();
            }
        }
        // 0 means no limit specified.
        0
    }

    #[must_use]
    pub fn get_pdu_max_rcv_size(&self) -> usize {
        for user_pdu in &self.my_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return usize::try_from(mli.max_length()).unwrap_or_default();
            }
        }
        // 0 means no limit specified.
        0
    }

    /// Accept the association request, negotiating the association parameters and leaving the
    /// reader/writer in a state to start exchanging PDUs.
    ///
    /// # Errors
    /// - I/O errors may occur when reading/writing from the reader/writer.
    /// - Any misbehaving SCU will be managed within, and this will not propagate these as errors.
    pub fn accept<R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), AssocError> {
        let rq = Pdu::read(&mut reader)
            .map_err(AssocError::ab_failure)
            .and_then(|rq| match rq {
                Pdu::AssocRQ(rq) => Ok(rq),
                pdu => Err(AssocError::ab_unexpected_pdu(
                    DimseError::UnexpectedPduType(pdu.pdu_type()),
                )),
            });

        let rq = match rq {
            Ok(rq) => rq,
            Err(e) => return Err(e),
        };

        let assoc_ac = match self.validate_assoc_rq(&rq) {
            Ok(rq) => rq,
            Err(e) => return Err(e),
        };

        for pres_ctx in assoc_ac.pres_ctxs() {
            if pres_ctx.is_accepted() {
                self.negotiated_pres_ctx
                    .insert(pres_ctx.ctx_id(), pres_ctx.to_owned());
            }
        }

        self.write_pdu(&Pdu::AssocAC(assoc_ac), &mut writer)?;

        Ok(())
    }

    /// Validates the association request, checking that this `Association`'s configuration can
    /// handle and will accept the request.
    ///
    /// # Return
    /// Whether to accept, reject, or abort the association request. If accepted, also includes the
    /// accepted abstract and transfer syntaxes.
    ///
    /// # Errors
    /// - If the result of the request is to reject or abort, those are propagated as an
    /// `AssocError`.
    fn validate_assoc_rq(&mut self, rq: &AssocRQ) -> Result<AssocAC, AssocError> {
        let host_ae = self.host_ae.trim();

        let calling_ae = DEFAULT_CHARACTER_SET
            .decode(rq.calling_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::from(e)))?;
        if !self.accept_aets.is_empty() && !self.accept_aets.contains(&calling_ae) {
            return Err(AssocError::rj_calling_aet(DimseError::GeneralError(
                format!("Calling AE Title \"{calling_ae}\" not in accepted list"),
            )));
        }

        let called_ae = DEFAULT_CHARACTER_SET
            .decode(rq.called_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::CharsetError(e)))?;
        if called_ae != host_ae {
            return Err(AssocError::rj_called_aet(DimseError::GeneralError(
                format!("Called AE \"{called_ae}\" is not host AE \"{host_ae}\""),
            )));
        }

        // There's only a single DICOM Standard defined Application Context Name. Private
        // Application Context Names are allowed by the standard.
        let app_ctx = String::try_from(&Syntax(rq.app_ctx().app_context_name()))
            .map_err(AssocError::rj_unsupported)?;

        let app_ctx_uid = STANDARD_DICOM_DICTIONARY
            .get_uid_by_uid(&app_ctx)
            .ok_or_else(|| {
                AssocError::rj_unsupported(DimseError::GeneralError(format!(
                    "Application Context not found: {app_ctx}"
                )))
            })?;

        if app_ctx_uid != &DICOMApplicationContextName {
            return Err(AssocError::rj_unsupported(DimseError::GeneralError(
                format!(
                    "Unsupported Application Context: {}, {}",
                    app_ctx_uid.name(),
                    app_ctx_uid.uid()
                ),
            )));
        }

        self.their_user_data.clear();
        self.their_user_data
            .append(rq.user_info().user_data().clone().as_mut());

        // TODO: Do things with SOPClassCommonExtendedNegotiationItem, UserIdentityItem, etc.

        // Check the proposed presentation contexts and create responses for each.
        let mut rsp_pres_ctx: Vec<AssocACPresentationContext> =
            Vec::with_capacity(rq.pres_ctxs().len());
        for req_pres_ctx in rq.pres_ctxs() {
            let ts = req_pres_ctx
                .transfer_syntaxes()
                .iter()
                .filter_map(|ts| String::try_from(&Syntax(ts.transfer_syntaxes())).ok())
                .filter_map(|ts| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(&ts))
                .find(|ts| self.supported_ts.contains(ts));

            let Some(ts) = ts else {
                let deny_pres_ctx = AssocACPresentationContext::new(
                    req_pres_ctx.ctx_id(),
                    4,
                    TransferSyntaxItem::new(Vec::with_capacity(0)),
                );
                rsp_pres_ctx.push(deny_pres_ctx);
                continue;
            };

            let ab = String::try_from(&Syntax(req_pres_ctx.abstract_syntax().abstract_syntax()))
                .ok()
                .and_then(|ab| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&ab))
                .filter(|ab| self.supported_abs.contains(ab));

            if ab.is_none() {
                let deny_pres_ctx = AssocACPresentationContext::new(
                    req_pres_ctx.ctx_id(),
                    3,
                    TransferSyntaxItem::new(Vec::with_capacity(0)),
                );
                rsp_pres_ctx.push(deny_pres_ctx);
                continue;
            }

            let acc_pres_ctx = AssocACPresentationContext::new(
                req_pres_ctx.ctx_id(),
                0,
                TransferSyntaxItem::from(ts),
            );
            rsp_pres_ctx.push(acc_pres_ctx);
        }

        Ok(AssocAC::new(
            rq.called_ae().to_owned(),
            rq.calling_ae().to_owned(),
            rq.reserved_3().to_owned(),
            rq.app_ctx().to_owned(),
            rsp_pres_ctx,
            UserInformationItem::new(self.my_user_data.clone()),
        ))
    }

    /// Parse the next message, either a Command, DICOM Dataset, release/abort, or unexpected PDU.
    ///
    /// # Errors
    /// - I/O errors may occcur attempting to read PDU from the reader, or write an appropriate
    /// disconnect response to the writer.
    pub fn next_msg<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<DimseMsg, AssocError> {
        match read_next_pdu(reader) {
            Some(Ok(PduIterItem::Pdu(pdu))) => self.handle_disconnect(pdu, writer),
            Some(Ok(PduIterItem::CmdMessage(cmd))) => Ok(DimseMsg::Cmd(cmd)),
            Some(Ok(PduIterItem::Dataset(dataset))) => Ok(DimseMsg::Dataset(dataset)),
            Some(Err(err)) => Err(AssocError::ab_failure(err)),
            None => Err(AssocError::ab_failure(DimseError::GeneralError(
                "No DIMSE message received".to_owned(),
            ))),
        }
    }

    /// Continuously reads DICOM `PresentationDataValue` PDUs from the reader and writes the bytes
    /// to the given `out_writer`, stopping after processing the last fragment.
    ///
    /// # Parameters
    /// `reader` - The reader the `PresentationDataValue` PDUs will be read from.
    /// `writer` - The protocol's corresponding writer for sending A-ABORT if encountering errors.
    /// `out_writer` - The destination to write the `PresentationDataValue` data bytes to.
    ///
    /// # Errors
    /// - I/O errors may occur with the reader/writer.
    pub fn read_dataset<R: Read, W: Write, OW: Write>(
        &self,
        mut reader: &mut R,
        mut writer: &mut W,
        out_writer: &mut OW,
    ) -> Result<(), AssocError> {
        let mut all_read = false;
        while !all_read {
            let dcm_msg = self.next_msg(&mut reader, &mut writer)?;
            let DimseMsg::Dataset(pdv) = dcm_msg else {
                return Err(AssocError::ab_failure(DimseError::GeneralError(
                    "Expected DICOM dataset".to_string(),
                )));
            };

            all_read = pdv.is_last_fragment();
            out_writer
                .write_all(pdv.data())
                .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))?;
        }

        Ok(())
    }

    /// Writes the given command, chunking into `PresentationDataItem`'s based on the SCU's
    /// indicated `MaxLengthItem`.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the stream.
    /// - Parsing/encoding errors may occur when serializing the given command to
    /// `PresentationDataItem`s.
    pub fn write_command<W: Write>(
        &mut self,
        cmd: &CommandMessage,
        mut writer: &mut W,
    ) -> Result<(), AssocError> {
        let elements = cmd.message().flatten();
        let pdi_iter = PresDataIter::new(
            cmd.ctx_id(),
            self.get_pdu_max_snd_size(),
            true,
            elements.iter().copied(),
            &ImplicitVRLittleEndian,
            DEFAULT_CHARACTER_SET,
        );
        for pdi in pdi_iter {
            match pdi {
                Ok(pdi) => self.write_pdu(&Pdu::PresentationDataItem(pdi), &mut writer)?,
                Err(e) => return Err(AssocError::ab_failure(e)),
            }
        }
        Ok(())
    }

    /// Writes the given dataset, chunking into `PresentationDataItem`s based on the SCU's
    /// indicated `MaxLengthItem`.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the stream.
    /// - Parsing/encoding errors may occur when serializing the given dataset to
    /// `PresentationDataItem`s.
    pub fn write_dataset<W: Write>(
        &mut self,
        ctx_id: u8,
        dataset: &DicomRoot,
        mut writer: &mut W,
    ) -> Result<(), AssocError> {
        let elements = dataset.flatten();
        let pdi_iter = PresDataIter::new(
            ctx_id,
            self.get_pdu_max_snd_size(),
            false,
            elements.iter().copied(),
            dataset.ts(),
            dataset.cs(),
        );
        for pdi in pdi_iter {
            match pdi {
                Ok(pdi) => self.write_pdu(&Pdu::PresentationDataItem(pdi), &mut writer)?,
                Err(e) => return Err(AssocError::ab_failure(e)),
            }
        }
        Ok(())
    }

    /// Handles a PDU that is not a `PresentationDataItem`, after the association is negotiated. In
    /// this scenario the only valid PDUs are `ReleaseRQ` or `Abort`.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the stream.
    /// - `DimseError`s may occur if the response is unexpected.
    fn handle_disconnect<W: Write>(
        &self,
        pdu: Pdu,
        writer: &mut W,
    ) -> Result<DimseMsg, AssocError> {
        match pdu {
            Pdu::ReleaseRQ(_rq) => {
                self.write_pdu(&Pdu::ReleaseRP(ReleaseRP::new()), writer)?;
                Ok(DimseMsg::ReleaseRQ)
            }
            Pdu::ReleaseRP(_rp) => Ok(DimseMsg::ReleaseRP),
            Pdu::Abort(ab) => Ok(DimseMsg::Abort(ab)),
            other => {
                self.write_pdu(&Pdu::Abort(Abort::new(2, 2)), writer)?;
                Err(AssocError::error(DimseError::UnexpectedPduType(
                    other.pdu_type(),
                )))
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct ServiceAssocBuilder {
    id: usize,
    host_ae: String,
    accept_aets: HashSet<String>,
    supported_abs: HashSet<UIDRef>,
    supported_ts: HashSet<TSRef>,
    pdu_rcv_max_len: u32,
}

impl ServiceAssocBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    #[must_use]
    pub fn host_ae(mut self, host_ae: String) -> Self {
        self.host_ae = host_ae;
        self
    }

    #[must_use]
    pub fn accept_aets(mut self, accept_aets: HashSet<String>) -> Self {
        self.accept_aets = accept_aets;
        self
    }

    #[must_use]
    pub fn supported_abs(mut self, accept_abs: HashSet<UIDRef>) -> Self {
        self.supported_abs = accept_abs;
        self
    }

    #[must_use]
    pub fn supported_ts(mut self, accept_ts: HashSet<TSRef>) -> Self {
        self.supported_ts = accept_ts;
        self
    }

    #[must_use]
    pub fn pdu_rcv_max_len(mut self, pdu_rcv_max_len: u32) -> Self {
        self.pdu_rcv_max_len = pdu_rcv_max_len;
        self
    }

    #[must_use]
    pub fn build(self) -> ServiceAssoc {
        let num_abs = self.supported_abs.len();
        // MaxLengthItem + AsyncOperationsWindowItem + RoleSelectionItem * supported_abs.len()
        let num_user_data = 2 + self.supported_abs.len();
        let mut my_user_data = Vec::<UserPdu>::with_capacity(num_user_data);
        my_user_data.push(UserPdu::MaxLengthItem(MaxLengthItem::new(
            self.pdu_rcv_max_len,
        )));

        // Require synchronous transfers, until async is incorporated.
        my_user_data.push(UserPdu::AsyncOperationsWindowItem(
            AsyncOperationsWindowItem::new(1, 1),
        ));

        // TODO: Double-check how roles should be set up. Based on behavior it seems like this is
        // indicating which roles to accept from incoming associations, and not what roles this
        // device enacts?
        for ab in &self.supported_abs {
            my_user_data.push(UserPdu::RoleSelectionItem(RoleSelectionItem::new(
                ab.uid().into(),
                1,
                0,
            )));
        }

        ServiceAssoc {
            _id: self.id,
            host_ae: self.host_ae,
            accept_aets: self.accept_aets,
            supported_abs: self.supported_abs,
            supported_ts: self.supported_ts,
            my_user_data,

            their_user_data: Vec::with_capacity(num_user_data),
            negotiated_pres_ctx: HashMap::with_capacity(num_abs),
        }
    }
}
