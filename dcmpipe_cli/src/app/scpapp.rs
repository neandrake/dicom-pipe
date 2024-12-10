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

use crate::{
    app::{handle_assoc_result, indexapp::DicomDoc, CommandApplication},
    args::SvcProviderArgs,
    threadpool::ThreadPool,
};
use anyhow::Result;
use dcmpipe_lib::{
    core::defn::constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
    dict::uids::{
        CTImageStorage, DeformableSpatialRegistrationStorage, MRImageStorage,
        ModalityWorklistInformationModelFIND, NuclearMedicineImageStorage,
        PatientRootQueryRetrieveInformationModelFIND, PatientRootQueryRetrieveInformationModelGET,
        PatientRootQueryRetrieveInformationModelMOVE, PositronEmissionTomographyImageStorage,
        RTDoseStorage, RTImageStorage, RTPlanStorage, RTStructureSetStorage, RawDataStorage,
        SecondaryCaptureImageStorage, SpatialRegistrationStorage,
        StudyRootQueryRetrieveInformationModelFIND, StudyRootQueryRetrieveInformationModelGET,
        StudyRootQueryRetrieveInformationModelMOVE, VerificationSOPClass,
    },
    dimse::{
        assoc::{
            scp::{ServiceAssoc, ServiceAssocBuilder},
            CloseMsg, CommonAssoc, DimseMsg,
        },
        commands::{messages::CommandMessage, CommandStatus, CommandType, SubOpProgress},
        error::{AssocError, DimseError},
        pdus::PduType,
        svcops::AssocSvcOp,
    },
};
use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, BufWriter, Read, Write},
    net::TcpListener,
    path::PathBuf,
};

mod cecho;
mod cfind;
mod cget;
mod cmove;
mod cstore;

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
        //      indicating that max associations has been reached.
        let pool = ThreadPool::new(self.args.max_assoc);
        let listener = TcpListener::bind(&self.args.host)?;
        println!(
            "[info <>]: Listening for associations on {}",
            self.args.host
        );

        let accept_aets: HashMap<String, String> = self.args.accept_aet.iter().cloned().collect();
        let supported_abs = HashSet::from([
            &VerificationSOPClass,
            &PatientRootQueryRetrieveInformationModelFIND,
            &StudyRootQueryRetrieveInformationModelFIND,
            &ModalityWorklistInformationModelFIND,
            &PatientRootQueryRetrieveInformationModelMOVE,
            &StudyRootQueryRetrieveInformationModelMOVE,
            &PatientRootQueryRetrieveInformationModelGET,
            &StudyRootQueryRetrieveInformationModelGET,
            &CTImageStorage,
            &MRImageStorage,
            &RTImageStorage,
            &PositronEmissionTomographyImageStorage,
            &NuclearMedicineImageStorage,
            &SecondaryCaptureImageStorage,
            &RTStructureSetStorage,
            &RTDoseStorage,
            &RTPlanStorage,
            &RawDataStorage,
            &SpatialRegistrationStorage,
            &DeformableSpatialRegistrationStorage,
        ]);
        let supported_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);
        let max_pdu_size = self
            .args
            .max_pdu_size
            .and_then(|s| u32::try_from(s).ok())
            .unwrap_or(0);
        for (stream_id, stream) in listener.incoming().enumerate() {
            let stream = stream?;
            let db = self.args.db.clone();
            let assoc = ServiceAssocBuilder::new()
                .id(stream_id)
                .host_ae(self.args.aetitle.clone())
                .accept_aets(accept_aets.clone())
                .supported_abs(supported_abs.clone())
                .supported_ts(supported_ts.clone())
                .pdu_rcv_max_len(max_pdu_size)
                .build();
            pool.execute(move || {
                let reader = BufReader::new(&stream);
                let writer = BufWriter::new(&stream);
                let mut assoc_dev = AssociationDevice {
                    assoc,
                    reader,
                    writer,
                    db,
                };
                assoc_dev.start();
            })?;
        }
        Ok(())
    }
}

/// Convenience to shorten `CommandStatus`.
pub(crate) type Stat = CommandStatus;

/// Convenience to create `Err(AssocError::ab_failure(DimseError::GeneralError(msg)))`.
pub(crate) fn fail(msg: &str) -> AssocError {
    AssocError::ab_failure(DimseError::ApplicationError(msg.into()))
}

/// Convenience to create a `SubOpProgress`.
pub(crate) fn prog(remaining: u16, completed: u16, failed: u16, warning: u16) -> SubOpProgress {
    SubOpProgress(remaining, completed, failed, warning)
}

/// Assists in producing progress/status responses to an initial request, particularly for C-MOVE
/// and C-GET.
pub(crate) struct StatusMsgBuilder {
    for_cmove: bool,
    ctx_id: u8,
    msg_id: u16,
    aff_sop_class: String,
}

impl StatusMsgBuilder {
    pub(crate) fn new(for_cmove: bool, ctx_id: u8, msg_id: u16, aff_sop_class: String) -> Self {
        Self {
            for_cmove,
            ctx_id,
            msg_id,
            aff_sop_class,
        }
    }

    pub(crate) fn msg(&self, status: &Stat, progress: &SubOpProgress) -> CommandMessage {
        // TODO: Also populate &ErrorComment and &OffendingElement.
        if self.for_cmove {
            self.mv_msg(status, progress)
        } else {
            self.get_msg(status, progress)
        }
    }

    fn mv_msg(&self, status: &Stat, progress: &SubOpProgress) -> CommandMessage {
        CommandMessage::c_move_rsp(
            self.ctx_id,
            self.msg_id,
            &self.aff_sop_class,
            status,
            progress,
        )
    }

    fn get_msg(&self, status: &Stat, progress: &SubOpProgress) -> CommandMessage {
        CommandMessage::c_get_rsp(
            self.ctx_id,
            self.msg_id,
            &self.aff_sop_class,
            status,
            progress,
        )
    }
}

struct AssociationDevice<R: Read, W: Write> {
    assoc: ServiceAssoc,
    reader: R,
    writer: W,
    db: Option<String>,
}

impl<R: Read, W: Write> AssociationDevice<R, W> {
    fn start(&mut self) {
        let result = self.main_loop();
        let wrote_release_rp = matches!(result, Ok(DimseMsg::CloseMsg(CloseMsg::ReleaseRQ)));
        let output = handle_assoc_result(result, &mut self.writer);
        for line in output {
            match line {
                Ok(line) => println!("{line}"),
                Err(line) => eprintln!("{line}"),
            }
        }
        if wrote_release_rp {
            println!("[info ->]: {:?}", PduType::ReleaseRP);
        }
    }

    fn main_loop(&mut self) -> Result<DimseMsg, AssocError> {
        println!("[info <-]: {:?}", PduType::AssocRQ);
        self.assoc.accept(&mut self.reader, &mut self.writer)?;
        println!("[info ->]: {:?}", PduType::AssocAC);

        loop {
            let msg = CommonAssoc::next_msg(
                &mut self.reader,
                &mut self.writer,
                self.assoc.common().get_pdu_max_rcv_size(),
            )?;
            let cmd = match msg {
                DimseMsg::Cmd(cmd) => cmd,
                DimseMsg::Dataset(_) => {
                    println!("[warn <-]: Received dataset out of request handler, discarding.");
                    continue;
                }
                other => return Ok(other),
            };
            println!("[info <-]: {:?}", cmd.cmd_type());

            if cmd.cmd_type() == &CommandType::CCancelReq {
                // TODO: After implementing async PDU handling this should cancel in-flight
                // operations.
                println!("[warn <-]: {:?}", CommandType::CCancelReq);
                continue;
            }

            let Some(op) = CommonAssoc::recv_req(&cmd)? else {
                continue;
            };

            match op {
                AssocSvcOp::Echo(op) => {
                    self.handle_c_echo_req(op, &cmd)?;
                    println!("[info ->]: {:?}", CommandType::CEchoRsp);
                }
                AssocSvcOp::Find(op) => {
                    self.handle_c_find_req(op, &cmd)?;
                    println!("[info ->]: {:?}", CommandType::CFindRsp);
                }
                AssocSvcOp::Get(op) => {
                    self.handle_c_get_req(op, &cmd)?;
                    println!("[info ->]: {:?}", CommandType::CGetRsp);
                }
                AssocSvcOp::Move(op) => {
                    self.handle_c_move_req(op, &cmd)?;
                    println!("[info ->]: {:?}", CommandType::CMoveRsp);
                }
                AssocSvcOp::Store(op) => {
                    self.handle_c_store_req(op, &cmd)?;
                    println!("[info ->]: {:?}", CommandType::CStoreRsp);
                }
                AssocSvcOp::Cancel(op) => {
                    self.assoc.common_mut().remove_svc_op(op.msg_id());
                }
            }
        }
    }

    /// Flat-maps the search results from UID/Key -> Series to UID/Key -> Files.
    pub(crate) fn resolve_to_paths(
        group_map: HashMap<String, Vec<DicomDoc>>,
    ) -> HashMap<String, Vec<PathBuf>> {
        let mut path_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for (group_id, docs) in group_map {
            let paths = docs
                .iter()
                .filter_map(|d| {
                    d.doc()
                        .get_document("metadata")
                        .and_then(|m| m.get_array("files"))
                        .ok()
                })
                .flatten()
                .filter_map(|b| b.as_str())
                .map(PathBuf::from)
                .collect::<Vec<PathBuf>>();
            path_map.insert(group_id, paths);
        }
        path_map
    }

    /// Inspects the response of the C-STORE for errors, reporting error progress to C-MOVE
    /// initiator and returning a `Result::Err`.
    pub(crate) fn interpret_cstore_rsp(
        &mut self,
        store_rsp: Result<DimseMsg, AssocError>,
        stat_rpt: &StatusMsgBuilder,
        progress: &SubOpProgress,
    ) -> Result<CommandMessage, AssocError> {
        let cmd_rsp = match store_rsp {
            Ok(DimseMsg::Cmd(cmd)) => cmd,
            Ok(rp) => {
                CommonAssoc::write_command(
                    &stat_rpt.msg(&Stat::fail(), progress),
                    &mut self.writer,
                    self.assoc.common().get_pdu_max_snd_size(),
                )?;
                return Err(fail(&format!("Sub-operation C-STORE failed: {rp:?}")));
            }
            Err(e) => {
                CommonAssoc::write_command(
                    &stat_rpt.msg(&Stat::fail(), progress),
                    &mut self.writer,
                    self.assoc.common().get_pdu_max_snd_size(),
                )?;
                return Err(e);
            }
        };

        if !cmd_rsp.status().is_success() {
            CommonAssoc::write_command(
                &stat_rpt.msg(&Stat::fail(), progress),
                &mut self.writer,
                self.assoc.common().get_pdu_max_snd_size(),
            )?;
            return Err(fail(&format!("Sub-operation C-STORE failed: {cmd_rsp:?}")));
        }

        Ok(cmd_rsp)
    }
}
