use std::{
    collections::{HashMap, HashSet},
    io::{Read, Write},
};

use crate::{
    core::{
        charset::DEFAULT_CHARACTER_SET as CS,
        dcmobject::DicomRoot,
        defn::{dcmdict::DicomDictionary, ts::TSRef, uid::UIDRef},
        write::{builder::WriterBuilder, writer::WriterState},
    },
    dict::{stdlookup::STANDARD_DICOM_DICTIONARY, transfer_syntaxes::ImplicitVRLittleEndian},
    dimse::{
        commands::CommandType,
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                Abort, AssocAC, AssocACPresentationContext, AssocRQ, ReleaseRP, TransferSyntaxItem,
            },
            pduiter::{DimseMsg, DimseMsgIter},
            Pdu, PduType,
        },
        Syntax,
    },
};

pub type MsgHandler = fn(TSRef, &DimseMsg, &mut dyn Read, &mut dyn Write) -> Result<(), AssocError>;

struct AssocAcResult {
    ac: AssocAC,
    ab: UIDRef,
    ts: TSRef,
}

pub struct Association {
    _id: usize,
    host_ae: String,
    accept_aets: HashSet<String>,
    accept_abs: HashSet<UIDRef>,
    accept_ts: HashSet<TSRef>,
    handlers: HashMap<CommandType, MsgHandler>,
}

impl Association {
    /// Serialize the given `DicomRoot` into in-memory bytes.
    ///
    /// # Errors
    /// An I/O error may occur when writing to the in-memory `Vec`.
    pub fn serialize(dicom: &DicomRoot) -> Result<Vec<u8>, AssocError> {
        let mut ds_writer = WriterBuilder::default()
            .state(WriterState::Element)
            .ts(&ImplicitVRLittleEndian)
            .build(Vec::<u8>::new());
        ds_writer
            .write_dcmroot(dicom)
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::WriteError(e)))?;
        Ok(ds_writer.into_dataset())
    }

    /// Write the given PDU to the given writer.
    ///
    /// # Errors
    /// I/O errors may occur when writing to the writer or flushing the writer.
    pub fn write_pdu<W: Write>(&self, pdu: &Pdu, mut writer: &mut W) -> Result<(), AssocError> {
        pdu.write(&mut writer).map_err(AssocError::error)?;
        writer
            .flush()
            .map_err(|e| AssocError::error(DimseError::IOError(e)))?;
        Ok(())
    }

    /// Begin processing an `Association` acting as a Service Class Provider, reading requests from
    /// the  reader and writing responses to the writer.
    ///
    /// # Errors
    /// - I/O errors may occur when reading/writing from the reader/writer.
    /// - Any misbehaving SCU will be managed within, and this will not propagate these as errors.
    pub fn start<R: Read, W: Write>(&self, mut reader: R, mut writer: W) -> Result<(), AssocError> {
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
            Err(e) => return e.write(&mut writer).map_err(AssocError::error),
        };

        let assoc_ac = match self.validate_assoc_rq(&rq) {
            Ok(rq) => rq,
            Err(e) => return e.write(&mut writer).map_err(AssocError::error),
        };
        let (ab, ts) = (assoc_ac.ab, assoc_ac.ts);

        self.write_pdu(&Pdu::AssocAC(assoc_ac.ac), &mut writer)?;

        if let Err(e) = self.msg_loop(ab, ts, &mut reader, &mut writer) {
            e.write(&mut writer).map_err(AssocError::error)
        } else {
            Ok(())
        }
    }

    /// Validates the association request, checking that this `Association`'s configuration can
    /// handle and will accept the request.
    ///
    /// # Return
    /// Whether to accept, reject, or abort the association request. If accepted, also includes the
    /// accepted abstract and transfer syntaxes.
    ///
    /// # Errors
    /// If the result of the request is to reject or abort, those are propagated as an `AssocError`.
    fn validate_assoc_rq(&self, rq: &AssocRQ) -> Result<AssocAcResult, AssocError> {
        let host_ae = self.host_ae.trim();

        let calling_ae = CS
            .decode(rq.calling_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::from(e)))?;
        if !self.accept_aets.is_empty() && !self.accept_aets.contains(&calling_ae) {
            return Err(AssocError::rj_calling_aet(DimseError::GeneralError(
                format!("Calling AE Title \"{calling_ae}\" not in accepted list"),
            )));
        }

        let called_ae = CS
            .decode(rq.called_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::CharsetError(e)))?;
        if called_ae != host_ae {
            return Err(AssocError::rj_called_aet(DimseError::GeneralError(
                format!("Called AE \"{called_ae}\" is not host AE \"{host_ae}\""),
            )));
        }

        let pres_ctx = rq.pres_ctxs().first().ok_or_else(|| {
            AssocError::rj_failure(DimseError::GeneralError(
                "No presentation context items defined".to_string(),
            ))
        })?;

        let ab = String::try_from(&Syntax(pres_ctx.abstract_syntax().abstract_syntax()))
            .ok()
            .and_then(|ab| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&ab));

        let Some(ab) = ab else {
            let ab = pres_ctx.abstract_syntax().abstract_syntax().clone();
            let ab = String::from_utf8(ab.clone()).unwrap_or_else(|_e| format!("{ab:?}"));
            return Err(AssocError::rj_failure(DimseError::GeneralError(format!(
                "Unsupported abstract syntax: {ab:?}"
            ))));
        };

        if !self.accept_abs.contains(ab) {
            return Err(AssocError::rj_failure(DimseError::GeneralError(format!(
                "Unsupported abstract syntax: {}",
                ab.uid()
            ))));
        }

        let ts = pres_ctx
            .transfer_syntaxes()
            .iter()
            .find_map(|ts| String::try_from(&Syntax(ts.transfer_syntaxes())).ok())
            .and_then(|ts| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(&ts))
            .filter(|ts| self.accept_ts.contains(ts))
            .ok_or_else(|| {
                AssocError::rj_unsupported(DimseError::GeneralError(format!(
                    "No transfer syntax supported: {:?}",
                    pres_ctx.transfer_syntaxes()
                )))
            })?;

        let ac = AssocAC::new(
            rq.called_ae().to_owned(),
            rq.calling_ae().to_owned(),
            rq.reserved_3().to_owned(),
            rq.app_ctx().to_owned(),
            vec![AssocACPresentationContext::new(
                pres_ctx.ctx_id(),
                0u8, // Acceptance
                TransferSyntaxItem::from(ts),
            )],
            rq.user_info().to_owned(),
        );

        Ok(AssocAcResult { ac, ab, ts })
    }

    /// The main request/resposne processing loop.
    fn msg_loop<R: Read, W: Write>(
        &self,
        _ab: UIDRef,
        ts: TSRef,
        mut reader: &mut R,
        mut writer: &mut W,
    ) -> Result<(), AssocError> {
        loop {
            // Read one command message then handle it. The handling can't be done within a loop
            // over the command message iterator as both the iterator and the handler need to read
            // from the reader.
            let mut msg_iter = DimseMsgIter::new(&mut reader);
            let msg = match msg_iter.next() {
                Some(Ok(msg)) => msg,

                Some(Err(DimseError::UnexpectedPduType(pdu))) => {
                    return self.handle_disconnect(pdu, writer);
                }
                Some(Err(err)) => {
                    return Err(AssocError::ab_failure(err));
                }
                None => {
                    return Err(AssocError::ab_failure(DimseError::GeneralError(
                        "No DIMSE message received".to_owned(),
                    )));
                }
            };

            if let Some(handler) = self.handlers.get(msg.cmd().cmd_type()) {
                handler(ts, &msg, &mut reader, &mut writer)?;
            }
        }
    }

    /// Handles a PDU that is not a `PresentationDataItem`, after the association is negotiated. In
    /// this scenario the only valid PDUs are `ReleaseRQ` or `Abort`.
    fn handle_disconnect<W: Write>(
        &self,
        pdu_type: PduType,
        writer: &mut W,
    ) -> Result<(), AssocError> {
        match pdu_type {
            PduType::ReleaseRQ => {
                self.write_pdu(&Pdu::ReleaseRP(ReleaseRP::new()), writer)?;
                Ok(())
            }
            PduType::Abort => Ok(()),
            pdu_type => {
                self.write_pdu(&Pdu::Abort(Abort::new(2, 2)), writer)?;
                Err(AssocError::error(DimseError::UnexpectedPduType(pdu_type)))
            }
        }
    }
}

#[derive(Default)]
pub struct AssociationBuilder {
    id: usize,
    host_ae: String,
    accept_aets: HashSet<String>,
    accept_abs: HashSet<UIDRef>,
    accept_ts: HashSet<TSRef>,
    handlers: HashMap<CommandType, MsgHandler>,
}

impl AssociationBuilder {
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
    pub fn accept_abs(mut self, accept_abs: HashSet<UIDRef>) -> Self {
        self.accept_abs = accept_abs;
        self
    }

    #[must_use]
    pub fn accept_ts(mut self, accept_ts: HashSet<TSRef>) -> Self {
        self.accept_ts = accept_ts;
        self
    }

    #[must_use]
    pub fn handler(mut self, cmd_type: CommandType, handler: MsgHandler) -> Self {
        self.handlers.insert(cmd_type, handler);
        self
    }

    #[must_use]
    pub fn build(self) -> Association {
        Association {
            _id: self.id,
            host_ae: self.host_ae,
            accept_aets: self.accept_aets,
            accept_abs: self.accept_abs,
            accept_ts: self.accept_ts,
            handlers: self.handlers,
        }
    }
}
