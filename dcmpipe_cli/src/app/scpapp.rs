/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the \"License\");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an \"AS IS\" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use crate::{app::CommandApplication, args::SvcProviderArgs, threadpool::ThreadPool};
use anyhow::Result;
use dcmpipe_lib::{
    core::{
        charset::DEFAULT_CHARACTER_SET as CS,
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
        tags::{PatientID, PatientsName},
        uids::{
            ModalityWorklistInformationModelFIND, PatientRootQueryRetrieveInformationModelFIND,
            PatientStudyOnlyQueryRetrieveInformationModelFIND,
            StudyRootQueryRetrieveInformationModelFIND, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{Association, AssociationBuilder},
        commands::{messages::CommandMessage, CommandStatus, CommandType},
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                PresentationDataItem, PresentationDataValue, P_DATA_CMD_LAST,
                P_DATA_DCM_DATASET_LAST,
            },
            pduiter::DimseMsg,
            Pdu,
        },
    },
};
use std::{
    collections::HashSet,
    io::{BufReader, BufWriter, Read, Write},
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
            let stream = stream?;
            let assoc = AssociationBuilder::new()
                .id(stream_id)
                .host_ae(self.args.aetitle.clone())
                .accept_aets(accept_aets.clone())
                .accept_abs(accept_abs.clone())
                .accept_ts(accept_ts.clone())
                .handler(
                    CommandType::CEchoReq,
                    |_ts: TSRef, msg: &DimseMsg, _reader: &mut dyn Read, writer: &mut dyn Write| {
                        AssociationDevice::handle_c_echo(msg, writer)
                    },
                )
                .handler(
                    CommandType::CFindReq,
                    |ts: TSRef, msg: &DimseMsg, reader: &mut dyn Read, writer: &mut dyn Write| {
                        AssociationDevice::handle_c_find(ts, msg, reader, writer)
                    },
                )
                .build();
            pool.execute(move || {
                if let Err(e) = AssociationDevice::process(&stream, &assoc) {
                    eprintln!("[ err ><]: {e}");
                }
            })?;
        }
        Ok(())
    }
}

struct AssociationDevice;

impl AssociationDevice {
    pub fn process(stream: &TcpStream, assoc: &Association) -> Result<(), AssocError> {
        let bufread = BufReader::new(stream);
        let bufwrite = BufWriter::new(stream);
        assoc.start(bufread, bufwrite)
    }

    fn write_pdu(pdu: &Pdu, mut writer: &mut dyn Write) -> Result<(), AssocError> {
        pdu.write(&mut writer).map_err(AssocError::ab_failure)?;
        writer
            .flush()
            .map_err(|err| AssocError::ab_failure(DimseError::IOError(err)))
    }

    fn handle_c_echo(msg: &DimseMsg, writer: &mut dyn Write) -> Result<(), AssocError> {
        let rsp = CommandMessage::c_echo_rsp_from_req(
            &ImplicitVRLittleEndian,
            msg.cmd(),
            &CommandStatus::Success(0),
        )
        .map_err(AssocError::ab_invalid_pdu)?;

        let data = Association::serialize(rsp.message())?;
        let rsp = PresentationDataItem::new(vec![PresentationDataValue::new(
            msg.ctx_id(),
            P_DATA_CMD_LAST,
            data,
        )]);

        AssociationDevice::write_pdu(&Pdu::PresentationDataItem(rsp), writer)
    }

    fn handle_c_find(
        ts: TSRef,
        msg: &DimseMsg,
        reader: &mut dyn Read,
        writer: &mut dyn Write,
    ) -> Result<(), AssocError> {
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(ts)
            .stop(ParseStop::AfterBytesRead(u64::from(msg.dcm_len())))
            .build(reader, &STANDARD_DICOM_DICTIONARY);
        let query = DicomRoot::parse(&mut parser)
            .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
        let Some(query) = query else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(
                "No DICOM query after parsing query".to_string(),
            )));
        };

        let q_pid = query
            .get_value_by_tag(&PatientID)
            .and_then(|v| v.string().cloned())
            .unwrap_or_default();
        let q_name = query
            .get_value_by_tag(&PatientsName)
            .and_then(|v| v.string().cloned())
            .unwrap_or_default();

        /* TODO: Execute Search on Query */
        let mut results = Vec::<DicomRoot>::new();
        for patient in [
            ("477-0101", "SNOW^JON"),
            ("477-0183", "STARK^ROB"),
            ("212-0309", "MARTELL^OBERYN"),
        ] {
            let pid = patient.0;
            let name = patient.1;

            let pid_match = !pid.starts_with(&q_pid) && !pid.ends_with(&q_pid);
            let name_match = name
                .split('^')
                .any(|p| p.starts_with(&q_name) || p.ends_with(&q_name));
            if !pid_match && !name_match {
                continue;
            }

            let mut result = DicomRoot::new_empty(ts, CS);
            result.add_child_with_val(&PatientID, RawValue::of_string(pid));
            result.add_child_with_val(&PatientsName, RawValue::of_string(name));
            results.push(result);
        }

        for result in results {
            match AssociationDevice::create_c_find_cmd(msg, &CommandStatus::Pending(0xFF00)) {
                Ok(rsp) => {
                    AssociationDevice::write_pdu(&Pdu::PresentationDataItem(rsp), writer)?;
                }
                Err(e) => return Err(e),
            }

            match AssociationDevice::create_c_find_result(msg, &result) {
                Ok(rsp) => {
                    AssociationDevice::write_pdu(&Pdu::PresentationDataItem(rsp), writer)?;
                }
                Err(e) => return Err(e),
            }
        }

        match AssociationDevice::create_c_find_cmd(msg, &CommandStatus::Success(0)) {
            Ok(rsp) => AssociationDevice::write_pdu(&Pdu::PresentationDataItem(rsp), writer)
                .map_err(AssocError::from),
            Err(e) => Err(e),
        }
    }

    fn create_c_find_cmd(
        msg: &DimseMsg,
        status: &CommandStatus,
    ) -> Result<PresentationDataItem, AssocError> {
        let rsp = CommandMessage::c_find_rsp_from_req(&ImplicitVRLittleEndian, msg.cmd(), status)
            .map_err(AssocError::ab_invalid_pdu)?;

        let data = Association::serialize(rsp.message())?;
        let rsp = PresentationDataItem::new(vec![PresentationDataValue::new(
            msg.ctx_id(),
            P_DATA_CMD_LAST,
            data,
        )]);

        Ok(rsp)
    }

    fn create_c_find_result(
        msg: &DimseMsg,
        result: &DicomRoot,
    ) -> Result<PresentationDataItem, AssocError> {
        let data = Association::serialize(result)?;
        Ok(PresentationDataItem::new(vec![PresentationDataValue::new(
            msg.ctx_id(),
            P_DATA_DCM_DATASET_LAST,
            data,
        )]))
    }
}
