use crate::{app::CommandApplication, args::SvcProviderArgs, threadpool::ThreadPool};
use anyhow::{anyhow, Error, Result};
use dcmpipe_lib::{
    core::{
        charset::DEFAULT_CHARACTER_SET,
        dcmobject::DicomRoot,
        defn::constants::ts::ImplicitVRLittleEndian,
        read::{ParserBuilder, ParserState},
        write::{builder::WriterBuilder, writer::WriterState},
    },
    dict::{stdlookup::STANDARD_DICOM_DICTIONARY, uids::VerificationSOPClass},
    dimse::{
        commands::{messages::CommandMessage, CommandStatus},
        pdus::{
            Abort, AssocAC, AssocACPresentationContext, AssocRJ, AssocRQ, Pdu,
            PresentationDataItem, PresentationDataValue, ReleaseRP, TransferSyntaxItem,
        },
    },
};
use std::{
    fmt::Display,
    io::{BufReader, BufWriter, Cursor, Write},
    net::{TcpListener, TcpStream},
};

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
            Vec::<String>::with_capacity(0)
        };

        for stream in listener.incoming() {
            let stream = stream?;
            let host_ae = self.args.aetitle.clone();
            let accept_aets = accept_aets.clone();
            pool.execute(|| {
                let mut assoc = Association {
                    stream,
                    host_ae,
                    accept_aets,
                };
                if let Err(e) = assoc.handle_association() {
                    eprintln!("[ err ><]: {e:?}");
                }
            })?;
        }
        Ok(())
    }
}

struct Association {
    stream: TcpStream,
    host_ae: String,
    accept_aets: Vec<String>,
}

#[derive(Debug)]
enum AssocErrorRsp {
    RJ(AssocRJ),
    AB(Abort),
}

#[derive(Debug)]
struct AssocError {
    rsp: AssocErrorRsp,
    err: anyhow::Error,
}

impl Display for AssocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl Association {
    fn handle_association(&mut self) -> Result<()> {
        let mut bufread = BufReader::new(&self.stream);
        let mut bufwrite = BufWriter::new(&self.stream);
        let remote_ip = self.stream.peer_addr()?.ip().to_string();

        let pdu = Pdu::read_pdu(&mut bufread)?;
        let Pdu::AssocRQ(rq) = pdu else {
            let ab = Abort::new(0u8, 2u8);
            ab.write(&mut bufwrite)?;
            return Err(anyhow!("Unexpected PDU: {pdu:?}"));
        };

        println!("[info <-]: A-ASSOCIATE-RQ {remote_ip}");
        match self.handle_assoc_rq(rq) {
            Err(e) => {
                match e.rsp {
                    AssocErrorRsp::RJ(rj) => {
                        println!("[info ->]: A-ASSOCIATE-RJ {remote_ip}");
                        rj.write(&mut bufwrite)?;
                    }
                    AssocErrorRsp::AB(ab) => {
                        println!("[info ->]: A-ABORT {remote_ip}");
                        ab.write(&mut bufwrite)?;
                    }
                }
                bufwrite.flush()?;
                return Err(e.err);
            }
            Ok(rsp) => {
                println!("[info ->]: A-ASSOCIATE-AC {remote_ip}");
                rsp.write(&mut bufwrite)?;
                bufwrite.flush()?;
            }
        }

        loop {
            let pdu = Pdu::read_pdu(&mut bufread)?;
            match pdu {
                Pdu::PresentationDataItem(pdi) => {
                    println!("[info <-]: PresentationDataItem");
                    match self.handle_pres_data_item(pdi) {
                        Err(e) => {
                            match e.rsp {
                                AssocErrorRsp::RJ(rj) => {
                                    println!("[info ->]: A-ASSOCIATE-RJ {remote_ip}");
                                    rj.write(&mut bufwrite)?;
                                }
                                AssocErrorRsp::AB(ab) => {
                                    println!("[info ->]: A-ABORT {remote_ip}");
                                    ab.write(&mut bufwrite)?;
                                }
                            }
                            bufwrite.flush()?;
                            return Err(e.err);
                        }
                        Ok(responses) => {
                            for rsp in responses {
                                println!("[info ->]: PresentationDataItem");
                                rsp.write(&mut bufwrite)?;
                                bufwrite.flush()?;
                            }
                        }
                    }
                }
                Pdu::ReleaseRQ(_rq) => {
                    println!("[info <-]: A-RELEASE-RQ {remote_ip}");
                    println!("[info ->]: A-RELEASE-RP {remote_ip}");
                    let rsp = ReleaseRP::new();
                    rsp.write(&mut bufwrite)?;
                    bufwrite.flush()?;
                    return Ok(());
                }
                Pdu::Abort(ab) => {
                    let reason = match ab.reason() {
                        0 => "Not-specified".to_owned(),
                        1 => "Unrecognized PDU".to_owned(),
                        2 => "Unexpected PDU".to_owned(),
                        3 => "Unexpected session-service primitive".to_owned(),
                        4 => "Unrecognized PDU parameter".to_owned(),
                        5 => "Unexpected PDU parameter".to_owned(),
                        6 => "Invalid PDU parameter value".to_owned(),
                        c => format!("Non-standard reason {c}"),
                    };
                    println!("[info <-]: A-ABORT {remote_ip}: {reason}");
                    return Ok(());
                }
                pdu => {
                    println!("[info ->]: A-ABORT {remote_ip}");
                    let ab = Abort::new(0u8, 2u8);
                    ab.write(&mut bufwrite)?;
                    return Err(anyhow!("Unexpected PDU: {pdu:?}"));
                }
            }
        }
    }

    fn handle_assoc_rq(&self, rq: AssocRQ) -> Result<AssocAC, AssocError> {
        let cs = DEFAULT_CHARACTER_SET;
        let host_ae = self.host_ae.trim();

        let calling_ae = cs
            .decode(rq.calling_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(0u8, 6u8)),
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

        let called_ae = cs
            .decode(rq.called_ae())
            .map(|ae| ae.trim().to_owned())
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(0u8, 6u8)),
                err: Error::new(e)
                    .context(format!("Failed to decode called_ae {:?}", rq.called_ae())),
            })?;
        if host_ae != called_ae {
            return Err(AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 7u8)),
                err: anyhow!("Called AE ({called_ae}) is not host AE ({host_ae})"),
            });
        }

        if rq.pres_ctxs().is_empty() {
            return Err(AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 1u8)),
                err: anyhow!("No presentation context items defined"),
            });
        }
        let pres_ctx = &rq.pres_ctxs()[0];
        let abstract_syntax = cs
            .decode(pres_ctx.abstract_syntax().abstract_syntax())
            .map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(0, 6u8)),
                err: Error::new(e).context(format!(
                    "Failed to decode abstract syntax: {:?}",
                    pres_ctx.abstract_syntax().abstract_syntax()
                )),
            })?;
        let abstract_syntax = abstract_syntax.trim();

        if abstract_syntax != VerificationSOPClass.uid() {
            return Err(AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 2u8)),
                err: anyhow!("Invalid abstract syntax: {abstract_syntax}"),
            });
        }

        let ts = pres_ctx
            .transfer_syntaxes()
            .iter()
            .filter_map(|ts| cs.decode(ts.transfer_syntaxes()).ok())
            .find(|ts| ts == ImplicitVRLittleEndian.uid().uid());

        let Some(ts) = ts else {
            return Err(AssocError {
                rsp: AssocErrorRsp::RJ(AssocRJ::new(2u8, 1u8, 2u8)),
                err: anyhow!(
                    "Unsupported transfer syntax: {:?}",
                    pres_ctx.transfer_syntaxes()
                ),
            });
        };

        Ok(AssocAC::new(
            rq.called_ae().to_owned(),
            rq.calling_ae().to_owned(),
            rq.reserved_3().to_owned(),
            rq.app_ctx().to_owned(),
            vec![AssocACPresentationContext::new(
                pres_ctx.ctx_id(),
                0u8, // Acceptance
                TransferSyntaxItem::new(ts.into_bytes()),
            )],
            rq.user_info().to_owned(),
        ))
    }

    fn handle_pres_data_item(
        &self,
        pdi: PresentationDataItem,
    ) -> Result<Vec<PresentationDataItem>, AssocError> {
        let pb = ParserBuilder::default()
            .state(ParserState::Element)
            .dictionary(&STANDARD_DICOM_DICTIONARY)
            .dataset_ts(&ImplicitVRLittleEndian);

        let mut responses = Vec::<PresentationDataItem>::with_capacity(pdi.pres_data().len());
        for pdv in pdi.pres_data() {
            let mut parser = pb.build(Cursor::new(pdv.data()));
            let dicom_root = DicomRoot::parse(&mut parser).map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(0u8, 6u8)),
                err: Error::new(e).context("Failed parsing presentation data value".to_string()),
            })?;
            let Some(dicom_root) = dicom_root else {
                continue;
            };
            let command = CommandMessage::new(dicom_root);
            let rsp = CommandMessage::c_echo_rsp_from_req(command, &CommandStatus::Success(0))
                .map_err(|e| AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(0u8, 6u8)),
                    err: Error::new(e).context(
                        "Failed converting C-ECHO command message into response".to_string(),
                    ),
                })?;

            let mut writer = WriterBuilder::default()
                .state(WriterState::Element)
                .ts(&ImplicitVRLittleEndian)
                .build(Vec::<u8>::new());
            writer
                .write_dcmroot(rsp.message())
                .map_err(|e| AssocError {
                    rsp: AssocErrorRsp::AB(Abort::new(1u8, 6u8)),
                    err: Error::new(e).context("Failed encoding C-ECHO response"),
                })?;
            let data = writer.into_dataset().map_err(|e| AssocError {
                rsp: AssocErrorRsp::AB(Abort::new(1u8, 6u8)),
                err: Error::new(e).context("Failed encoding C-ECHO response"),
            })?;

            let rsp = PresentationDataItem::new(vec![PresentationDataValue::new(
                pdv.ctx_id(),
                pdv.msg_header(),
                data,
            )]);
            responses.push(rsp);
        }

        Ok(responses)
    }
}
