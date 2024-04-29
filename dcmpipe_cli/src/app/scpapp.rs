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
    app::handle_assoc_result, app::CommandApplication, args::SvcProviderArgs,
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
            DimseMsg,
        },
        commands::CommandType,
        error::AssocError,
        pdus::PduType,
    },
};
use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, BufWriter, Read, Write},
    net::TcpListener,
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
        //      indicating that max assocations has been reached.
        let pool = ThreadPool::new(self.args.max_assoc);
        let listener = TcpListener::bind(&self.args.host)?;
        println!(
            "[info <>]: Listening for associations on {}",
            self.args.host
        );

        let accept_aets: HashMap<String, String> = self.args.accept_aets.iter().cloned().collect();
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

struct AssociationDevice<R: Read, W: Write> {
    assoc: ServiceAssoc,
    reader: R,
    writer: W,
    db: Option<String>,
}

impl<R: Read, W: Write> AssociationDevice<R, W> {
    fn start(&mut self) {
        let result = self.main_loop();
        let wrote_release_rp = matches!(result, Ok(DimseMsg::ReleaseRQ));
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
            let msg = self
                .assoc
                .common()
                .next_msg(&mut self.reader, &mut self.writer)?;
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

            if cmd.cmd_type() == &CommandType::CEchoReq {
                self.handle_c_echo_req(&cmd)?;
                println!("[info ->]: {:?}", CommandType::CEchoRsp);
            } else if cmd.cmd_type() == &CommandType::CFindReq {
                self.handle_c_find_req(&cmd)?;
                println!("[info ->]: {:?}", CommandType::CFindRsp);
            } else if cmd.cmd_type() == &CommandType::CStoreReq {
                self.handle_c_store_req(&cmd)?;
                println!("[info ->]: {:?}", CommandType::CStoreRsp);
            } else if cmd.cmd_type() == &CommandType::CMoveReq {
                self.handle_c_move_req(&cmd)?;
                println!("[info ->]: {:?}", CommandType::CMoveRsp);
            } else if cmd.cmd_type() == &CommandType::CGetReq {
                self.handle_c_get_req(&cmd)?;
                println!("[info ->]: {:?}", CommandType::CGetRsp);
            }
        }
    }
}
