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
        charset::DEFAULT_CHARACTER_SET,
        dcmobject::DicomRoot,
        defn::{
            constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
            dcmdict::DicomDictionary,
            ts::TSRef,
        },
        read::{ParserBuilder, ParserState},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{AffectedSOPClassUID, MessageID, PatientID, PatientsName},
        uids::{
            CTImageStorage, MRImageStorage, ModalityWorklistInformationModelFIND,
            NuclearMedicineImageStorage, PatientRootQueryRetrieveInformationModelFIND,
            PatientStudyOnlyQueryRetrieveInformationModelFIND,
            PositronEmissionTomographyImageStorage, RTDoseStorage, RTPlanStorage,
            RTStructureSetStorage, SecondaryCaptureImageStorage,
            StudyRootQueryRetrieveInformationModelFIND, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{Association, AssociationBuilder, DimseMsg},
        commands::{messages::CommandMessage, CommandStatus, CommandType},
        error::{AssocError, DimseError},
        pdus::PduType,
        Syntax,
    },
};
use std::{
    collections::HashSet,
    io::{BufReader, BufWriter, Cursor, Read, Write},
    net::TcpListener,
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
            HashSet::<String>::with_capacity(0)
        };

        let accept_abs = HashSet::from([
            &VerificationSOPClass,
            &PatientRootQueryRetrieveInformationModelFIND,
            &StudyRootQueryRetrieveInformationModelFIND,
            &ModalityWorklistInformationModelFIND,
            &PatientStudyOnlyQueryRetrieveInformationModelFIND,
            &CTImageStorage,
            &MRImageStorage,
            &PositronEmissionTomographyImageStorage,
            &NuclearMedicineImageStorage,
            &SecondaryCaptureImageStorage,
            &RTStructureSetStorage,
            &RTDoseStorage,
            &RTPlanStorage,
        ]);
        let accept_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);

        for (stream_id, stream) in listener.incoming().enumerate() {
            let stream = stream?;
            let assoc = AssociationBuilder::new()
                .id(stream_id)
                .host_ae(self.args.aetitle.clone())
                .accept_aets(accept_aets.clone())
                .accept_abs(accept_abs.clone())
                .accept_ts(accept_ts.clone())
                .build();
            pool.execute(move || {
                let bufread = BufReader::new(&stream);
                let bufwrite = BufWriter::new(&stream);
                let mut assoc_dev = AssociationDevice::new(assoc, bufread, bufwrite);
                match assoc_dev.start() {
                    Ok(DimseMsg::ReleaseRQ) => println!("[info <-]: {:?}", PduType::ReleaseRQ),
                    Ok(DimseMsg::Abort(ab)) => println!("[warn <-]: {}", ab.get_reason_desc()),
                    Ok(other) => eprintln!("Unexpected ending: {other:?}"),
                    Err(e) => eprintln!("[ err ><]: {e}"),
                }
            })?;
        }
        Ok(())
    }
}

struct AssociationDevice<R: Read, W: Write> {
    assoc: Association,
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> AssociationDevice<R, W> {
    fn new(assoc: Association, reader: R, writer: W) -> Self {
        Self {
            assoc,
            reader,
            writer,
        }
    }

    fn start(&mut self) -> Result<DimseMsg, AssocError> {
        self.assoc.accept(&mut self.reader, &mut self.writer)?;
        println!("[info <-]: {:?}", PduType::AssocRQ);
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

            if !cmd.has_dataset() {
                if cmd.cmd_type() == &CommandType::CEchoReq {
                    self.handle_c_echo_req(&cmd)?;
                    println!("[info ->]: {:?}", CommandType::CEchoRsp);
                }
                continue;
            }

            let mut buffer = Vec::<u8>::new();
            let mut all_read = false;
            while !all_read {
                let dcm_msg = self.assoc.next_msg(&mut self.reader, &mut self.writer)?;
                let DimseMsg::Dataset(pdv) = dcm_msg else {
                    return Err(AssocError::ab_failure(DimseError::GeneralError(
                        "Expected DICOM dataset".to_string(),
                    )));
                };

                all_read = pdv.is_last_fragment();
                buffer.append(&mut pdv.into_data());
            }

            let mut dcm_parser = ParserBuilder::default()
                .dataset_ts(ts)
                .state(ParserState::ReadElement)
                .build(Cursor::new(buffer), &STANDARD_DICOM_DICTIONARY);

            let dcm = DicomRoot::parse(&mut dcm_parser)
                .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                .ok_or_else(|| {
                    AssocError::ab_failure(DimseError::GeneralError(
                        "Expected DICOM dataset".to_string(),
                    ))
                })?;

            if cmd.cmd_type() == &CommandType::CFindReq {
                self.handle_c_find_req(&cmd, &dcm)?;
                println!("[info ->]: {:?}", CommandType::CFindRsp);
            } else if cmd.cmd_type() == &CommandType::CStoreReq {
                self.handle_c_store_req(&cmd, &dcm)?;
                println!("[info ->]: {:?}", CommandType::CStoreRsp);
            }
        }
    }

    fn handle_c_echo_req(&mut self, cmd: &CommandMessage) -> Result<(), AssocError> {
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;
        let end_rsp = Association::create_cecho_end(cmd.ctx_id(), cmd.msg_id(), &aff_sop_class)?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;
        Ok(())
    }

    fn handle_c_find_req(
        &mut self,
        cmd: &CommandMessage,
        dcm: &DicomRoot,
    ) -> Result<(), AssocError> {
        /* TODO: Execute Search on Query */
        let results = AssociationDevice::<R, W>::create_dummy_results(dcm, dcm.ts());

        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        for result in results {
            let res_rsp =
                Association::create_cfind_result(ctx_id, msg_id, &aff_sop_class, &result)?;
            self.assoc.write_pdu(&res_rsp.0, &mut self.writer)?;
            self.assoc.write_pdu(&res_rsp.1, &mut self.writer)?;
        }

        let end_rsp = Association::create_cfind_end(ctx_id, msg_id, &aff_sop_class)?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;

        Ok(())
    }

    fn create_dummy_results(query: &DicomRoot, ts: TSRef) -> Vec<DicomRoot> {
        let q_pid = query
            .get_value_by_tag(&PatientID)
            .and_then(|v| v.string().cloned())
            .unwrap_or_default();
        let q_name = query
            .get_value_by_tag(&PatientsName)
            .and_then(|v| v.string().cloned())
            .unwrap_or_default();

        let mut results = Vec::<DicomRoot>::new();
        for patient in [
            ("477-0101", "SNOW^JON"),
            ("477-0183", "STARK^ROB"),
            ("212-0309", "MARTELL^OBERYN"),
        ] {
            let pid = patient.0;
            let name = patient.1;

            let pid_match = if q_pid.is_empty() {
                false
            } else {
                pid.starts_with(&q_pid) || pid.ends_with(&q_pid)
            };
            let name_match = if q_name.is_empty() {
                false
            } else {
                name.split('^')
                    .any(|p| p.starts_with(&q_name) || p.ends_with(&q_name))
            };
            if !pid_match && !name_match {
                continue;
            }

            let mut result = DicomRoot::new_empty(ts, DEFAULT_CHARACTER_SET);
            result.add_child_with_val(&PatientID, RawValue::of_string(pid));
            result.add_child_with_val(&PatientsName, RawValue::of_string(name));
            results.push(result);
        }
        results
    }

    fn handle_c_store_req(
        &mut self,
        cmd: &CommandMessage,
        _dcm: &DicomRoot,
    ) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let end_rsp = Association::create_cstore_end(
            ctx_id,
            msg_id,
            &aff_sop_class,
            &CommandStatus::success(),
        )?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;

        Ok(())
    }
}
