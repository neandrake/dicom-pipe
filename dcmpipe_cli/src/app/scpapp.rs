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
            ts::TSRef,
        },
        read::{stop::ParseStop, ParserBuilder, ParserState},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{AffectedSOPClassUID, MessageID, PatientID, PatientsName},
        uids::{
            ModalityWorklistInformationModelFIND, PatientRootQueryRetrieveInformationModelFIND,
            PatientStudyOnlyQueryRetrieveInformationModelFIND,
            StudyRootQueryRetrieveInformationModelFIND, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{Association, AssociationBuilder},
        commands::CommandType,
        error::{AssocError, DimseError},
        pdus::pduiter::DimseMsg,
    },
};
use std::{
    collections::HashSet,
    io::{BufReader, BufWriter, Read, Write},
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
        ]);
        let accept_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);

        let assoc_builder = AssociationBuilder::new()
            .host_ae(self.args.aetitle.clone())
            .accept_aets(accept_aets.clone())
            .accept_abs(accept_abs.clone())
            .accept_ts(accept_ts.clone())
            .handler(
                CommandType::CEchoReq,
                |assoc: &Association,
                 _ts: TSRef,
                 msg: &DimseMsg,
                 _reader: &mut dyn Read,
                 mut writer: &mut dyn Write| {
                    AssociationDevice::handle_c_echo_req(assoc, msg, &mut writer)
                },
            )
            .handler(
                CommandType::CFindReq,
                |assoc: &Association,
                 ts: TSRef,
                 msg: &DimseMsg,
                 reader: &mut dyn Read,
                 mut writer: &mut dyn Write| {
                    AssociationDevice::handle_c_find_req(assoc, ts, msg, reader, &mut writer)
                },
            );

        for (stream_id, stream) in listener.incoming().enumerate() {
            let stream = stream?;
            let assoc = assoc_builder.clone().id(stream_id).build();
            pool.execute(move || {
                let bufread = BufReader::new(&stream);
                let bufwrite = BufWriter::new(&stream);
                if let Err(e) = assoc.start(bufread, bufwrite) {
                    eprintln!("[ err ><]: {e}");
                }
            })?;
        }
        Ok(())
    }
}

struct AssociationDevice;

impl AssociationDevice {
    fn handle_c_echo_req(
        assoc: &Association,
        msg: &DimseMsg,
        mut writer: &mut dyn Write,
    ) -> Result<(), AssocError> {
        let msg_id = msg
            .cmd()
            .get_ushort(&MessageID)
            .map_err(AssocError::ab_failure)?;
        let aff_sop_class = msg
            .cmd()
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;
        let end_rsp = Association::create_cecho_end(msg.ctx_id(), msg_id, &aff_sop_class)?;
        assoc.write_pdu(&end_rsp, &mut writer)
    }

    fn handle_c_find_req(
        assoc: &Association,
        ts: TSRef,
        req: &DimseMsg,
        reader: &mut dyn Read,
        mut writer: &mut dyn Write,
    ) -> Result<(), AssocError> {
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(ts)
            .stop(ParseStop::AfterBytesRead(u64::from(req.dcm_len())))
            .build(reader, &STANDARD_DICOM_DICTIONARY);
        let query = DicomRoot::parse(&mut parser)
            .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
        let Some(query) = query else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(
                "No DICOM query after parsing query".to_string(),
            )));
        };

        /* TODO: Execute Search on Query */
        let results = AssociationDevice::create_dummy_results(&query, ts);

        let ctx_id = req.ctx_id();
        let msg_id = req
            .cmd()
            .get_ushort(&MessageID)
            .map_err(AssocError::ab_failure)?;
        let aff_sop_class = req
            .cmd()
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        for result in results {
            let res_rsp =
                Association::create_cfind_result(ctx_id, msg_id, &aff_sop_class, &result)?;
            assoc.write_pdu(&res_rsp.0, &mut writer)?;
            assoc.write_pdu(&res_rsp.1, &mut writer)?;
        }

        let end_rsp = Association::create_cfind_end(ctx_id, msg_id, &aff_sop_class)?;
        assoc.write_pdu(&end_rsp, &mut writer)?;

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
}
