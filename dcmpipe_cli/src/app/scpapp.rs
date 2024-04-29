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

use crate::{app::CommandApplication, args::SvcProviderArgs, threadpool::ThreadPool};
use anyhow::Result;
use dcmpipe_lib::{
    core::{
        dcmobject::DicomRoot,
        defn::{
            constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
            dcmdict::DicomDictionary,
        },
        read::{ParserBuilder, ParserState},
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        uids::{
            CTImageStorage, MRImageStorage, ModalityWorklistInformationModelFIND,
            NuclearMedicineImageStorage, PatientRootQueryRetrieveInformationModelFIND,
            PatientRootQueryRetrieveInformationModelGET,
            PatientRootQueryRetrieveInformationModelMOVE, PositronEmissionTomographyImageStorage,
            RTDoseStorage, RTPlanStorage, RTStructureSetStorage, SecondaryCaptureImageStorage,
            StudyRootQueryRetrieveInformationModelFIND, StudyRootQueryRetrieveInformationModelGET,
            StudyRootQueryRetrieveInformationModelMOVE, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{
            scp::{ServiceAssoc, ServiceAssocBuilder},
            DimseMsg,
        },
        commands::CommandType,
        error::{AssocError, AssocRsp, DimseError},
        pdus::PduType,
        Syntax,
    },
};
use std::{
    collections::HashSet,
    io::{BufReader, BufWriter, Cursor, Read, Write},
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

        let accept_aets = if let Some(aets) = &self.args.accept_aets {
            aets.split(',').map(str::to_owned).collect()
        } else {
            HashSet::<String>::with_capacity(0)
        };

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
            &PositronEmissionTomographyImageStorage,
            &NuclearMedicineImageStorage,
            &SecondaryCaptureImageStorage,
            &RTStructureSetStorage,
            &RTDoseStorage,
            &RTPlanStorage,
        ]);
        let supported_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);
        for (stream_id, stream) in listener.incoming().enumerate() {
            let stream = stream?;
            let db = self.args.db.clone();
            let assoc = ServiceAssocBuilder::new()
                .id(stream_id)
                .host_ae(self.args.aetitle.clone())
                .accept_aets(accept_aets.clone())
                .supported_abs(supported_abs.clone())
                .supported_ts(supported_ts.clone())
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
        match self.main_loop() {
            Ok(DimseMsg::ReleaseRQ) => println!("[info <-]: {:?}", PduType::ReleaseRQ),
            Ok(DimseMsg::Abort(ab)) => println!("[warn <-]: {}", ab.get_reason_desc()),
            Ok(other) => eprintln!("[ err xx]: Unexpected ending state: {other:?}"),
            Err(e) => {
                eprintln!("[ err ><]: {e}");
                match e.rsp() {
                    Some(AssocRsp::RJ(rj)) => println!("[info ->]: {:?}", rj.pdu_type()),
                    Some(AssocRsp::AB(ab)) => println!("[info ->]: {:?}", ab.pdu_type()),
                    None => {}
                }
                if let Err(inner) = e.write(&mut self.writer) {
                    eprintln!("[ err xx]: Failure writing error response: {inner}");
                }
            }
        }
    }

    fn main_loop(&mut self) -> Result<DimseMsg, AssocError> {
        println!("[info <-]: {:?}", PduType::AssocRQ);
        self.assoc.accept(&mut self.reader, &mut self.writer)?;
        println!("[info ->]: {:?}", PduType::AssocAC);

        loop {
            let msg = self.assoc.next_msg(&mut self.reader, &mut self.writer)?;
            let cmd = match msg {
                DimseMsg::Cmd(cmd) => cmd,
                DimseMsg::Dataset(_) => {
                    return Err(AssocError::ab_failure(DimseError::GeneralError(
                        "Received DICOM dataset without prior Command.".to_string(),
                    )));
                }
                DimseMsg::ReleaseRQ => return Ok(DimseMsg::ReleaseRQ),
                DimseMsg::Abort(ab) => return Ok(DimseMsg::Abort(ab)),
            };
            println!("[info <-]: {:?}", cmd.cmd_type());

            let Some(pres_ctx) = self.assoc.get_pres_ctx(cmd.ctx_id()) else {
                return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                    "Presentation Context not found: {}",
                    cmd.ctx_id()
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

            if cmd.cmd_type() == &CommandType::CCancelReq {
                // TODO: After implementing async PDU handling this should cancel in-flight
                // operations.
                println!("[warn <-]: {:?}", CommandType::CCancelReq);
                continue;
            }

            if !cmd.has_dataset() {
                if cmd.cmd_type() == &CommandType::CEchoReq {
                    self.handle_c_echo_req(&cmd)?;
                    println!("[info ->]: {:?}", CommandType::CEchoRsp);
                }
                continue;
            }

            // XXX: For C-STORE this will result in loading the entire received dataset into memory
            // at once.
            let mut buffer = Cursor::new(Vec::<u8>::new());
            self.read_dataset(&mut buffer)?;
            buffer.set_position(0);

            let mut dcm_parser = ParserBuilder::default()
                .dataset_ts(ts)
                .state(ParserState::ReadElement)
                .build(buffer, &STANDARD_DICOM_DICTIONARY);

            let dcm = DicomRoot::parse(&mut dcm_parser)
                .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                .ok_or_else(|| {
                    AssocError::ab_failure(DimseError::GeneralError(
                        "Parsing DICOM dataset failed".to_string(),
                    ))
                })?;

            if cmd.cmd_type() == &CommandType::CFindReq {
                self.handle_c_find_req(&cmd, &dcm)?;
                println!("[info ->]: {:?}", CommandType::CFindRsp);
            } else if cmd.cmd_type() == &CommandType::CStoreReq {
                self.handle_c_store_req(&cmd, &dcm)?;
                println!("[info ->]: {:?}", CommandType::CStoreRsp);
            } else if cmd.cmd_type() == &CommandType::CMoveReq {
                self.handle_c_move_req(&cmd, &dcm)?;
                println!("[info ->]: {:?}", CommandType::CMoveRsp);
            } else if cmd.cmd_type() == &CommandType::CGetReq {
                self.handle_c_get_req(&cmd, &dcm)?;
                println!("[info ->]: {:?}", CommandType::CGetRsp);
            }
        }
    }

    /// Continuously reads DICOM `PresentationDataValue` PDUs from the stream and writes the bytes
    /// to the given writer, stopping after processing the last fragment.
    ///
    /// # Errors
    /// I/O errors may occur reading from `self.reader`, writing a failure response to
    /// `self.writer`, or writing the DICOM PDV bytes to the given `writer`.
    pub(crate) fn read_dataset(&mut self, writer: &mut dyn Write) -> Result<(), AssocError> {
        let mut all_read = false;
        while !all_read {
            let dcm_msg = self.assoc.next_msg(&mut self.reader, &mut self.writer)?;
            let DimseMsg::Dataset(pdv) = dcm_msg else {
                return Err(AssocError::ab_failure(DimseError::GeneralError(
                    "Expected DICOM dataset".to_string(),
                )));
            };

            all_read = pdv.is_last_fragment();
            writer
                .write_all(pdv.data())
                .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))?;
        }

        Ok(())
    }
}
