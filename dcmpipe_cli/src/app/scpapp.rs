use crate::{app::CommandApplication, args::SvcProviderArgs, threadpool::ThreadPool};
use anyhow::Result;
use dcmpipe_lib::{
    core::{
        charset::{CSRef, DEFAULT_CHARACTER_SET},
        dcmobject::DicomRoot,
        defn::{
            constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
            dcmdict::DicomDictionary,
            ts::TSRef,
            uid::UIDRef,
            vr::US,
        },
        read::{stop::ParseStop, ParserBuilder, ParserState},
        write::{builder::WriterBuilder, writer::WriterState},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{CommandField, PatientID, PatientsName},
        uids::{
            ModalityWorklistInformationModelFIND, PatientRootQueryRetrieveInformationModelFIND,
            PatientStudyOnlyQueryRetrieveInformationModelFIND,
            StudyRootQueryRetrieveInformationModelFIND, VerificationSOPClass,
        },
    },
    dimse::{
        commands::{messages::CommandMessage, CommandStatus, CommandType},
        error::{AssocError, DimseError},
        pduiter::{PduIter, PduIterItem},
        pdus::{
            Abort, AssocAC, AssocACPresentationContext, AssocRQ, Pdu, PresentationDataItem,
            PresentationDataValue, PresentationDataValueHeader, ReleaseRP, TransferSyntaxItem,
            P_DATA_CMD_LAST, P_DATA_DCM_DATASET_LAST,
        },
        Syntax,
    },
};
use std::{
    collections::HashSet,
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

static CS: CSRef = DEFAULT_CHARACTER_SET;

pub struct SvcProviderApp {
    args: SvcProviderArgs,
}

impl SvcProviderApp {
    pub fn new(args: SvcProviderArgs) -> SvcProviderApp {
        SvcProviderApp { args }
    }
}

impl CommandApplication for SvcProviderApp {
    fn run(&mut self) -> Result<()> {
        // XXX: This enforces max associations with a thread pool, but will fail to send a response
        //      indicating that max assocations has been reached.
        let pool = ThreadPool::new(self.args.max_assoc);
        let listener = TcpListener::bind(&self.args.host)?;
        println!(
            "[info <>]: Listening for associations on {}",
            self.args.host
        );

        let accept_aets = if let Some(aets) = &self.args.accept_aets {
            aets.split(',').map(str::to_owned).collect()
        } else {
            HashSet::<String>::with_capacity(0)
        };

        let accept_abs = HashSet::from([
            &VerificationSOPClass,
            &PatientRootQueryRetrieveInformationModelFIND,
            &StudyRootQueryRetrieveInformationModelFIND,
            &ModalityWorklistInformationModelFIND,
            &PatientStudyOnlyQueryRetrieveInformationModelFIND,
        ]);
        let accept_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);

        for (stream_id, stream) in listener.incoming().enumerate() {
            let id = stream_id;
            let stream = stream?;
            let host_ae = self.args.aetitle.clone();
            let accept_aets = accept_aets.clone();
            let accept_abs = accept_abs.clone();
            let accept_ts = accept_ts.clone();
            let mut assoc = Association {
                _id: id,
                stream,
                host_ae,
                accept_aets,
                accept_abs,
                accept_ts,
            };
            pool.execute(move || {
                if let Err(e) = assoc.start_association() {
                    eprintln!("[ err ><]: {e}");
                } else {
                    println!("[info ><]: Association Ended");
                }
            })?;
        }
        Ok(())
    }
}

struct AssocAcResult {
    ac: AssocAC,
    ab: UIDRef,
    ts: TSRef,
}

struct Association {
    _id: usize,
    stream: TcpStream,
    host_ae: String,
    accept_aets: HashSet<String>,
    accept_abs: HashSet<UIDRef>,
    accept_ts: HashSet<TSRef>,
}

impl Association {
    fn write_pdu(
        &self,
        pdu: Pdu,
        mut bufwrite: &mut BufWriter<&TcpStream>,
    ) -> Result<(), AssocError> {
        pdu.write(&mut bufwrite).map_err(AssocError::error)?;
        bufwrite
            .flush()
            .map_err(|e| AssocError::error(DimseError::IOError(e)))?;
        Ok(())
    }

    fn start_association(&mut self) -> Result<(), DimseError> {
        let mut bufread = BufReader::new(&self.stream);
        let mut bufwrite = BufWriter::new(&self.stream);
        let remote_ip = self.stream.peer_addr()?.ip().to_string();

        let rq = Pdu::read(&mut bufread)
            .map_err(AssocError::ab_failure)
            .and_then(|rq| match rq {
                Pdu::AssocRQ(rq) => Ok(rq),
                pdu => Err(AssocError::ab_unexpected_pdu(DimseError::UnexpectedPDU(
                    pdu.pdu_type(),
                ))),
            });

        let rq = match rq {
            Ok(rq) => rq,
            Err(e) => {
                if let Some(rsp) = e.rsp() {
                    println!("[info ->]: {:?}", rsp.pdu_type());
                }
                return e.write(&mut bufwrite);
            }
        };

        // Gracefully decode the calling AE title for logging purposes. If the calling AE can't
        // properly be decoded it will be thrown up from handle_assoc_rq().
        let calling_ae =
            if let Ok(calling_ae) = CS.decode(rq.calling_ae()).map(|ae| ae.trim().to_owned()) {
                calling_ae
            } else {
                "[invalid ae]".to_owned()
            };
        let source = format!("{calling_ae} @ {remote_ip}");

        let assoc_ac = match self.validate_assoc_rq(&rq) {
            Ok(rq) => rq,
            Err(e) => {
                println!("[info <-]: {:?} {source}", rq.pdu_type());
                return e.write(&mut bufwrite);
            }
        };
        let (ab, ts) = (assoc_ac.ab, assoc_ac.ts);
        println!("[info <-]: {:?} {source}", rq.pdu_type());

        println!("[info ->]: {:?} {source}", assoc_ac.ac.pdu_type());
        self.write_pdu(Pdu::AssocAC(assoc_ac.ac), &mut bufwrite)
            .map_err(|e| e.into_err())?;

        if let Err(e) = self.pdu_loop(ab, ts, &mut bufread, &mut bufwrite) {
            if let Some(rsp) = e.rsp() {
                println!("[info ->]: {:?}", rsp.pdu_type());
            }
            e.write(&mut bufwrite)
        } else {
            Ok(())
        }
    }

    fn validate_assoc_rq(&self, rq: &AssocRQ) -> Result<AssocAcResult, AssocError> {
        let host_ae = self.host_ae.trim();

        let calling_ae = CS
            .decode(rq.calling_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::from(e)))?;
        if !self.accept_aets.is_empty() && !self.accept_aets.contains(&calling_ae) {
            return Err(AssocError::rj_calling_aet(DimseError::GeneralError(
                format!("Calling AE Title ({calling_ae}) not in accepted list"),
            )));
        }

        let called_ae = CS
            .decode(rq.called_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::CharsetError(e)))?;
        if called_ae != host_ae {
            return Err(AssocError::rj_called_aet(DimseError::GeneralError(
                format!("Called AE ({called_ae}) is not host AE ({host_ae})"),
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
            let ab = String::from_utf8(ab.clone()).unwrap_or_else(|_e| format!("{:?}", ab));
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

    /// Handles a PDU that is not a `PresentationDataItem`, after the association is negotiated. In
    /// this scenario the only valid PDUs are `ReleaseRQ` or `Abort`.
    fn handle_disconnect(
        &self,
        pdu: Pdu,
        bufwrite: &mut BufWriter<&TcpStream>,
    ) -> Result<(), AssocError> {
        match pdu {
            Pdu::ReleaseRQ(rq) => {
                println!("[info <-]: {:?}", rq.pdu_type());
                let rp = ReleaseRP::new();
                println!("[info ->]: {:?}", rp.pdu_type());
                self.write_pdu(Pdu::ReleaseRP(rp), bufwrite)?;
                Ok(())
            }
            Pdu::Abort(ab) => {
                println!("[info <-]: {:?}: {}", ab.pdu_type(), ab.get_reason_desc());
                Ok(())
            }
            pdu => {
                let ab = Abort::new(2u8, 2u8);
                println!("[info ->]: {:?}", ab.pdu_type());
                self.write_pdu(Pdu::Abort(ab), bufwrite)?;
                Err(AssocError::error(DimseError::UnexpectedPDU(pdu.pdu_type())))
            }
        }
    }

    fn pdu_loop(
        &self,
        _ab: UIDRef,
        ts: TSRef,
        mut bufread: &mut BufReader<&TcpStream>,
        bufwrite: &mut BufWriter<&TcpStream>,
    ) -> Result<(), AssocError> {
        loop {
            let mut cmd_type = CommandType::INVALID(0);
            let mut last_dcm_pdvh: Option<PresentationDataValueHeader> = None;
            let mut last_cmd_pdvh: Option<PresentationDataValueHeader> = None;
            let mut last_cmd: Option<CommandMessage> = None;

            let pdu_iter = PduIter::new(&mut bufread);
            for iter_item in pdu_iter {
                match iter_item {
                    Ok(PduIterItem::Pdu(pdu)) => {
                        return self.handle_disconnect(pdu, bufwrite);
                    }
                    Ok(PduIterItem::CmdMessage(pdvh, cmd)) => {
                        println!("[info <-]: P-DATA COMMAND");
                        let _ = cmd.message().dbg_dump();

                        cmd_type = CommandType::from(
                            cmd.message()
                                .get_value_as_by_tag(&CommandField, &US)
                                .and_then(|v| v.ushort())
                                .ok_or_else(|| {
                                    AssocError::rj_unsupported(DimseError::GeneralError(
                                        "failed to parse CommandField".to_string(),
                                    ))
                                })?,
                        );
                        last_cmd_pdvh = Some(pdvh);
                        last_cmd = Some(cmd);

                        // C-ECHO only requires a P-DATA command, and no DICOM dataset.
                        if cmd_type == CommandType::CEchoReq {
                            break;
                        }
                    }
                    Ok(PduIterItem::Dataset(pdvh)) => {
                        println!("[info <-]: P-DATA DICOM");
                        last_dcm_pdvh = Some(pdvh);
                        break;
                    }
                    Err(err) => {
                        return Err(AssocError::ab_failure(err));
                    }
                };
            }

            let Some(cmd_pdvh) = last_cmd_pdvh else {
                continue;
                /*
                return Err(AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                    err: anyhow!("No COMMAND PDVH after receiving DICOM dataset"),
                });
                */
            };
            let Some(cmd) = last_cmd else {
                continue;
                /*
                return Err(AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                    err: anyhow!("No COMMAND after receiving DICOM dataset"),
                });
                */
            };

            if cmd_type == CommandType::CEchoReq {
                let rsp = self.handle_c_echo(ts, cmd_pdvh, cmd)?;
                println!("[info ->]: {:?} COMMAND RSP", rsp.pdu_type());
                self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)?;
                continue;
            }

            // All services aside from C-ECHO require having received a DICOM dataset after the
            // command.
            let Some(dcm_pdvh) = last_dcm_pdvh else {
                return Err(AssocError::ab_failure(DimseError::GeneralError(
                    "No DICOM after receiving DICOM dataset".to_string(),
                )));
            };

            if cmd_type == CommandType::CFindReq {
                self.handle_c_find(ts, &cmd, &cmd_pdvh, &dcm_pdvh, bufread, bufwrite)?
            }
        }
    }

    fn handle_c_echo(
        &self,
        _ts: TSRef,
        pdvh: PresentationDataValueHeader,
        cmd: CommandMessage,
    ) -> Result<PresentationDataItem, AssocError> {
        let rsp = CommandMessage::c_echo_rsp_from_req(
            &ImplicitVRLittleEndian,
            cmd,
            &CommandStatus::Success(0),
        )
        .map_err(AssocError::ab_invalid_pdu)?;

        let mut writer = WriterBuilder::default()
            .state(WriterState::Element)
            .ts(&ImplicitVRLittleEndian)
            .build(Vec::<u8>::new());
        writer
            .write_dcmroot(rsp.message())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::WriteError(e)))?;
        let data = writer.into_dataset();
        let rsp = PresentationDataItem::new(vec![PresentationDataValue::new(
            pdvh.ctx_id(),
            P_DATA_CMD_LAST,
            data,
        )]);

        Ok(rsp)
    }

    fn handle_c_find(
        &self,
        ts: TSRef,
        cmd: &CommandMessage,
        cmd_pdvh: &PresentationDataValueHeader,
        dcm_pdvh: &PresentationDataValueHeader,
        bufread: &mut BufReader<&TcpStream>,
        bufwrite: &mut BufWriter<&TcpStream>,
    ) -> Result<(), AssocError> {
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(ts)
            .stop(ParseStop::AfterBytesRead(u64::from(
                dcm_pdvh.length_of_data(),
            )))
            .build(bufread, &STANDARD_DICOM_DICTIONARY);
        let query = DicomRoot::parse(&mut parser)
            .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
        let Some(query) = query else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(
                "No DICOM query after parsing query".to_string(),
            )));
        };

        let _ = query.dbg_dump();

        /* TODO: Execute Search on Query */
        let mut results = Vec::<DicomRoot>::new();
        for patient in [
            ("477-0101", "SNOW^JON"),
            ("477-0183", "STARK^ROB"),
            ("212-0309", "MARTELL^OBERYN"),
        ] {
            let pid = patient.0;
            let name = patient.1;
            let mut result = DicomRoot::new_empty(ts, CS);
            result.add_child_with_val(&PatientID, RawValue::of_string(pid));
            result.add_child_with_val(&PatientsName, RawValue::of_string(name));
            results.push(result);
        }

        for result in results {
            match self.create_c_find_cmd(cmd_pdvh, cmd, &CommandStatus::Pending(0xFF00)) {
                Ok(rsp) => {
                    println!("[info ->]: {:?} COMMAND", rsp.pdu_type());
                    self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)?;
                }
                Err(e) => return Err(e),
            }

            match self.create_c_find_result(dcm_pdvh, &result) {
                Ok(rsp) => {
                    println!("[info ->]: {:?} DICOM", rsp.pdu_type());
                    self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)?;
                }
                Err(e) => return Err(e),
            }
        }

        match self.create_c_find_cmd(cmd_pdvh, cmd, &CommandStatus::Success(0)) {
            Ok(rsp) => {
                println!("[info ->]: {:?} COMMAND", rsp.pdu_type());
                self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)
                    .map_err(AssocError::from)
            }
            Err(e) => Err(e),
        }
    }

    fn create_c_find_cmd(
        &self,
        pdvh: &PresentationDataValueHeader,
        cmd: &CommandMessage,
        status: &CommandStatus,
    ) -> Result<PresentationDataItem, AssocError> {
        let rsp = CommandMessage::c_find_rsp_from_req(&ImplicitVRLittleEndian, cmd, status)
            .map_err(AssocError::ab_invalid_pdu)?;

        let mut writer = WriterBuilder::default()
            .state(WriterState::Element)
            .ts(&ImplicitVRLittleEndian)
            .build(Vec::<u8>::new());
        writer
            .write_dcmroot(rsp.message())
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::WriteError(e)))?;
        let data = writer.into_dataset();
        let rsp = PresentationDataItem::new(vec![PresentationDataValue::new(
            pdvh.ctx_id(),
            P_DATA_CMD_LAST,
            data,
        )]);

        Ok(rsp)
    }

    fn create_c_find_result(
        &self,
        pdvh: &PresentationDataValueHeader,
        result: &DicomRoot,
    ) -> Result<PresentationDataItem, AssocError> {
        let mut writer = WriterBuilder::default()
            .state(WriterState::Element)
            .ts(&ImplicitVRLittleEndian)
            .build(Vec::<u8>::new());
        writer
            .write_dcmroot(result)
            .map_err(|e| AssocError::ab_invalid_pdu(DimseError::WriteError(e)))?;

        let result = writer.into_dataset();
        Ok(PresentationDataItem::new(vec![PresentationDataValue::new(
            pdvh.ctx_id(),
            P_DATA_DCM_DATASET_LAST,
            result,
        )]))
    }
}
