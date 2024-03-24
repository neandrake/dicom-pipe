use crate::{app::CommandApplication, args::SvcProviderArgs, threadpool::ThreadPool};
use anyhow::{anyhow, Error, Result};
use dcmpipe_lib::{
    core::{
        charset::{CSRef, DEFAULT_CHARACTER_SET},
        dcmobject::DicomRoot,
        defn::{
            constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
            dcmdict::DicomDictionary,
            ts::TSRef,
            uid::UIDRef,
        },
        read::{stop::ParseStop, ParserBuilder, ParserState},
        write::{builder::WriterBuilder, writer::WriterState},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::PatientID,
        uids::{ModalityWorklistInformationModelFIND, VerificationSOPClass},
    },
    dimse::{
        commands::{messages::CommandMessage, CommandStatus},
        pduiter::{PduIter, PduIterItem},
        pdus::{
            Abort, AssocAC, AssocACPresentationContext, AssocRJ, AssocRQ, Pdu,
            PresentationDataItem, PresentationDataValue, PresentationDataValueHeader, ReleaseRP,
            TransferSyntaxItem, P_DATA_CMD_LAST, P_DATA_DCM_DATASET_LAST,
        },
    },
};
use std::{
    collections::HashSet,
    fmt::Display,
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

        let accept_abs =
            HashSet::from([&VerificationSOPClass, &ModalityWorklistInformationModelFIND]);
        let accept_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);

        for (stream_id, stream) in listener.incoming().enumerate() {
            let id = stream_id;
            let stream = stream?;
            let host_ae = self.args.aetitle.clone();
            let accept_aets = accept_aets.clone();
            let accept_abs = accept_abs.clone();
            let accept_ts = accept_ts.clone();
            pool.execute(move || {
                let mut assoc = Association {
                    id,
                    stream,
                    host_ae,
                    accept_aets,
                    accept_abs,
                    accept_ts,
                };
                if let Err(e) = assoc.start_association() {
                    eprintln!("[ err ><]: {e:?}");
                } else {
                    println!("[info ><]: Association Ended");
                }
            })?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum AssocErrorRsp {
    RJ(AssocRJ),
    AB(Abort),
    AlreadyHandled,
}

#[derive(Debug)]
struct AssocError {
    rsp: AssocErrorRsp,
    err: Error,
}

impl From<Error> for AssocError {
    fn from(value: Error) -> Self {
        Self {
            rsp: AssocErrorRsp::AlreadyHandled,
            err: value,
        }
    }
}

impl From<(Error, String)> for AssocError {
    fn from(value: (Error, String)) -> Self {
        Self {
            rsp: AssocErrorRsp::AlreadyHandled,
            err: value.0.context(value.1),
        }
    }
}

impl From<(std::io::Error, String)> for AssocError {
    fn from(value: (std::io::Error, String)) -> Self {
        Self {
            rsp: AssocErrorRsp::AlreadyHandled,
            err: Error::from(value.0).context(value.1),
        }
    }
}

impl Display for AssocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

struct AssocAcResult {
    ac: AssocAC,
    ab: UIDRef,
    ts: TSRef,
}

struct Association {
    id: usize,
    stream: TcpStream,
    host_ae: String,
    accept_aets: HashSet<String>,
    accept_abs: HashSet<UIDRef>,
    accept_ts: HashSet<TSRef>,
}

impl Association {
    fn write_pdu(&self, pdu: Pdu, mut bufwrite: &mut BufWriter<&TcpStream>) -> Result<()> {
        pdu.write(&mut bufwrite)?;
        bufwrite.flush()?;
        Ok(())
    }

    fn handle_error(
        &self,
        err: AssocError,
        source: &str,
        bufwrite: &mut BufWriter<&TcpStream>,
    ) -> Result<()> {
        match err.rsp {
            AssocErrorRsp::RJ(rj) => {
                println!("[info ->]: A-ASSOCIATE-RJ {} {source}", self.id);
                self.write_pdu(Pdu::AssocRJ(rj), bufwrite)
            }
            AssocErrorRsp::AB(ab) => {
                println!("[info ->]: A-ABORT {} {source}", self.id);
                self.write_pdu(Pdu::Abort(ab), bufwrite)
            }
            AssocErrorRsp::AlreadyHandled => Err(err.err),
        }
    }

    fn start_association(&mut self) -> Result<()> {
        let mut bufread = BufReader::new(&self.stream);
        let mut bufwrite = BufWriter::new(&self.stream);
        let remote_ip = self.stream.peer_addr()?.ip().to_string();

        let rq = Pdu::read_pdu(&mut bufread)
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                err: Error::new(e).context("failed reading initial PDU"),
            })
            .and_then(|rq| match rq {
                Pdu::AssocRQ(rq) => Ok(rq),
                pdu => Err(AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(2u8, 2u8)),
                    err: anyhow!("Unexpected PDU: {pdu:?}"),
                }),
            });

        let rq = match rq {
            Ok(rq) => rq,
            Err(e) => return self.handle_error(e, &remote_ip, &mut bufwrite),
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
        println!("[info <-]: A-ASSOCIATE-RQ {} {source}", self.id);

        let assoc_ac = match self.validate_assoc_rq(rq) {
            Ok(rq) => rq,
            Err(e) => return self.handle_error(e, &source, &mut bufwrite),
        };
        println!("[info ->]: A-ASSOCIATE-AC {} {source}", self.id);
        self.write_pdu(Pdu::AssocAC(assoc_ac.ac), &mut bufwrite)?;
        let (ab, ts) = (assoc_ac.ab, assoc_ac.ts);

        if let Err(e) = self.pdu_loop(&source, ab, ts, &mut bufread, &mut bufwrite) {
            self.handle_error(e, &source, &mut bufwrite)
        } else {
            Ok(())
        }
    }

    fn pdu_loop(
        &self,
        source: &str,
        ab: UIDRef,
        ts: TSRef,
        mut bufread: &mut BufReader<&TcpStream>,
        bufwrite: &mut BufWriter<&TcpStream>,
    ) -> Result<(), AssocError> {
        loop {
            let mut last_dcm_pdvh: Option<PresentationDataValueHeader> = None;
            let mut last_cmd_pdvh: Option<PresentationDataValueHeader> = None;
            let mut last_cmd: Option<CommandMessage> = None;

            let pdu_iter = PduIter::new(&mut bufread);
            for iter_item in pdu_iter {
                match iter_item {
                    Ok(PduIterItem::Pdu(pdu)) => {
                        return self.handle_disconnect(pdu, bufwrite, source);
                    }
                    Ok(PduIterItem::CmdMessage(pdvh, cmd)) => {
                        println!("[info <-]: P-DATA COMMAND");
                        if ab == &VerificationSOPClass {
                            let rsp = self.handle_c_echo(ts, pdvh, cmd)?;
                            println!("[info ->]: P-DATA COMMAND RSP");
                            self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)?;
                            last_cmd_pdvh = None;
                            last_cmd = None;
                        } else {
                            last_cmd_pdvh = Some(pdvh);
                            last_cmd = Some(cmd);
                        }
                    }
                    Ok(PduIterItem::Dataset(pdvh)) => {
                        println!("[info <-]: P-DATA DICOM");
                        last_dcm_pdvh = Some(pdvh);
                        break;
                    }
                    Err(err) => {
                        return self
                            .handle_error(
                                AssocError {
                                    rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                                    err: Error::new(err).context("failed parsing PDU stream"),
                                },
                                source,
                                bufwrite,
                            )
                            .map_err(|e| {
                                AssocError::from((e, "failed handling error".to_owned()))
                            });
                    }
                };
            }

            let Some(cmd_pdvh) = last_cmd_pdvh else {
                return Err(AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                    err: anyhow!("No COMMAND PDVH after receiving DICOM dataset"),
                });
            };
            let Some(cmd) = last_cmd else {
                return Err(AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                    err: anyhow!("No COMMAND after receiving DICOM dataset"),
                });
            };
            let Some(dcm_pdvh) = last_dcm_pdvh else {
                return Err(AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                    err: anyhow!("No DICOM after receiving DICOM dataset"),
                });
            };

            if ab == &ModalityWorklistInformationModelFIND {
                self.handle_c_find(source, ts, &cmd, &cmd_pdvh, &dcm_pdvh, bufread, bufwrite)?;
            }
        }
    }

    fn validate_assoc_rq(&self, rq: AssocRQ) -> Result<AssocAcResult, AssocError> {
        let host_ae = self.host_ae.trim();

        let calling_ae = CS
            .decode(rq.calling_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(2u8, 6u8)),
                err: Error::new(e).context(format!(
                    "Failed to decode calling_ae: {:?}",
                    rq.calling_ae()
                )),
            })?;
        if !self.accept_aets.is_empty() && !self.accept_aets.contains(&calling_ae) {
            return Err(AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 3u8)),
                err: anyhow!("Calling AE Title ({calling_ae}) not in accepted list"),
            });
        }

        let called_ae = CS
            .decode(rq.called_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(2u8, 6u8)),
                err: Error::new(e)
                    .context(format!("Failed to decode called_ae {:?}", rq.called_ae())),
            })?;
        if called_ae != host_ae {
            return Err(AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 7u8)),
                err: anyhow!("Called AE ({called_ae}) is not host AE ({host_ae})"),
            });
        }

        let pres_ctx = rq.pres_ctxs().first().ok_or_else(|| AssocError {
            rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 1u8)),
            err: anyhow!("No presentation context items defined"),
        })?;

        let ab = CS
            .decode(pres_ctx.abstract_syntax().abstract_syntax())
            .ok()
            .and_then(|ab| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(ab.trim()))
            .into_iter()
            .find(|ab| self.accept_abs.contains(ab))
            .ok_or_else(|| AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 1u8)),
                err: anyhow!(
                    "Unsupported abstract syntax: {:?}",
                    pres_ctx.abstract_syntax().abstract_syntax()
                ),
            })?;

        let ts = pres_ctx
            .transfer_syntaxes()
            .iter()
            .find_map(|ts| CS.decode(ts.transfer_syntaxes()).ok())
            .and_then(|ts| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(ts.trim()))
            .filter(|ts| self.accept_ts.contains(ts))
            .ok_or_else(|| AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 2u8)),
                err: anyhow!(
                    "No transfer syntax supported: {:?}",
                    pres_ctx.transfer_syntaxes()
                ),
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
        source: &str,
    ) -> Result<(), AssocError> {
        match pdu {
            Pdu::ReleaseRQ(_rq) => {
                println!("[info <-]: A-RELEASE-RQ {source}");
                println!("[info ->]: A-RELEASE-RP {source}");
                self.write_pdu(Pdu::ReleaseRP(ReleaseRP::new()), bufwrite)
                    .map_err(|e| AssocError {
                        rsp: AssocErrorRsp::AlreadyHandled,
                        err: e.context("error releasing association"),
                    })?;
                Ok(())
            }
            Pdu::Abort(ab) => {
                println!("[info <-]: A-ABORT {source}: {}", ab.get_reason_desc());
                Ok(())
            }
            pdu => {
                println!("[info ->]: A-ABORT {source}");
                self.write_pdu(Pdu::Abort(Abort::new(2u8, 2u8)), bufwrite)
                    .map_err(|e| AssocError {
                        rsp: AssocErrorRsp::AlreadyHandled,
                        err: e.context("error aborting association"),
                    })?;
                Err(AssocError {
                    rsp: AssocErrorRsp::AlreadyHandled,
                    err: anyhow!("Unexpected PDU: {pdu:?}"),
                })
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
        .map_err(|e| AssocError {
            rsp: AssocErrorRsp::AB(Abort::new(2u8, 6u8)),
            err: Error::new(e)
                .context("Failed converting C-ECHO command message into response".to_string()),
        })?;

        let mut writer = WriterBuilder::default()
            .state(WriterState::Element)
            .ts(&ImplicitVRLittleEndian)
            .build(Vec::<u8>::new());
        writer
            .write_dcmroot(rsp.message())
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(2u8, 6u8)),
                err: Error::new(e).context("Failed encoding C-ECHO response"),
            })?;
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
        source: &str,
        ts: TSRef,
        cmd: &CommandMessage,
        cmd_pdvh: &PresentationDataValueHeader,
        dcm_pdvh: &PresentationDataValueHeader,
        mut bufread: &mut BufReader<&TcpStream>,
        bufwrite: &mut BufWriter<&TcpStream>,
    ) -> Result<(), AssocError> {
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(ts)
            .stop(ParseStop::AfterBytesRead(u64::from(
                dcm_pdvh.length_of_data(),
            )))
            .build(&mut bufread, &STANDARD_DICOM_DICTIONARY);
        let query = DicomRoot::parse(&mut parser).map_err(|e| AssocError {
            rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
            err: Error::from(e).context("parsing dicom pdv"),
        })?;
        let Some(_query) = query else {
            return Err(AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(0u8, 0u8)),
                err: anyhow!("No DICOM query after parsing query"),
            });
        };

        /* TODO: Execute Search on Query */
        let mut results = Vec::<DicomRoot>::new();
        for name in ["SNOW^JON", "STARK^ROB", "MARTELL^OBERYN"] {
            let mut result = DicomRoot::new_empty(ts, CS);
            result.add_child_with_val(&PatientID, RawValue::string(name));
            results.push(result);
        }

        for result in results {
            match self.create_c_find_cmd(ts, cmd_pdvh, cmd, &CommandStatus::Pending(0xFF00)) {
                Ok(rsp) => {
                    println!("[info ->]: P-DATA COMMAND");
                    self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)?;
                }
                Err(e) => {
                    return self
                        .handle_error(e, source, bufwrite)
                        .map_err(AssocError::from)
                }
            }

            match self.create_c_find_result(dcm_pdvh, &result) {
                Ok(rsp) => {
                    println!("[info ->]: P-DATA DICOM");
                    self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)?;
                }
                Err(e) => {
                    return self
                        .handle_error(e, source, bufwrite)
                        .map_err(AssocError::from)
                }
            }
        }

        match self.create_c_find_cmd(ts, cmd_pdvh, cmd, &CommandStatus::Success(0)) {
            Ok(rsp) => {
                println!("[info ->]: P-DATA COMMAND");
                self.write_pdu(Pdu::PresentationDataItem(rsp), bufwrite)
                    .map_err(AssocError::from)
            }
            Err(e) => self
                .handle_error(e, source, bufwrite)
                .map_err(AssocError::from),
        }
    }

    fn create_c_find_cmd(
        &self,
        _ts: TSRef,
        pdvh: &PresentationDataValueHeader,
        cmd: &CommandMessage,
        status: &CommandStatus,
    ) -> Result<PresentationDataItem, AssocError> {
        let rsp = CommandMessage::c_find_rsp_from_req(&ImplicitVRLittleEndian, cmd, status)
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(2u8, 6u8)),
                err: Error::new(e)
                    .context("Failed converting C-ECHO command message into response".to_string()),
            })?;

        let mut writer = WriterBuilder::default()
            .state(WriterState::Element)
            .ts(&ImplicitVRLittleEndian)
            .build(Vec::<u8>::new());
        writer
            .write_dcmroot(rsp.message())
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(2u8, 6u8)),
                err: Error::new(e).context("Failed encoding C-ECHO response"),
            })?;
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
        writer.write_dcmroot(result).map_err(|e| AssocError {
            rsp: AssocErrorRsp::AB(Abort::new(2u8, 6u8)),
            err: Error::new(e).context("Failed encoding C-ECHO response"),
        })?;

        let result = writer.into_dataset();
        Ok(PresentationDataItem::new(vec![PresentationDataValue::new(
            pdvh.ctx_id(),
            P_DATA_DCM_DATASET_LAST,
            result,
        )]))
    }
}
