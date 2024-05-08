use std::{
    collections::HashSet,
    fs::File,
    io::{stdout, BufReader, BufWriter, Write},
    net::TcpStream,
    path::PathBuf,
};

use dcmpipe_lib::{
    core::{
        defn::{
            constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
            dcmdict::DicomDictionary,
            tag::{Tag, TagNode},
            vr::LT,
        },
        inspect::FormattedElement,
        read::{valdecode::StringAndVr, ParserBuilder},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::AffectedSOPClassUID,
        uids::{
            CTImageStorage, DeformableSpatialRegistrationStorage, MRImageStorage,
            ModalityWorklistInformationModelFIND, NuclearMedicineImageStorage,
            PatientRootQueryRetrieveInformationModelFIND,
            PatientRootQueryRetrieveInformationModelGET,
            PatientRootQueryRetrieveInformationModelMOVE, PositronEmissionTomographyImageStorage,
            RTDoseStorage, RTImageStorage, RTPlanStorage, RTStructureSetStorage, RawDataStorage,
            SecondaryCaptureImageStorage, SpatialRegistrationStorage,
            StudyRootQueryRetrieveInformationModelFIND, StudyRootQueryRetrieveInformationModelGET,
            StudyRootQueryRetrieveInformationModelMOVE, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{
            scu::{UserAssoc, UserAssocBuilder},
            DimseMsg, QueryLevel,
        },
        commands::{messages::CommandMessage, CommandStatus, CommandType, SubOpProgress},
        error::{AssocError, DimseError},
    },
};

use crate::{
    app::{handle_assoc_result, rename_file_to_sop},
    args::{SvcUserArgs, SvcUserCommand},
    CommandApplication,
};

pub struct SvcUserApp {
    args: SvcUserArgs,
}

impl SvcUserApp {
    pub fn new(args: SvcUserArgs) -> SvcUserApp {
        SvcUserApp { args }
    }

    fn issue_c_echo(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let msg_id = assoc.next_msg_id();
        assoc.common().c_echo_rq(&mut reader, &mut writer, msg_id)?;
        assoc.release_association(&mut reader, &mut writer)
    }

    fn issue_c_find(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        query_level: QueryLevel,
        query: &[(String, String)],
    ) -> Result<Option<DimseMsg>, AssocError> {
        let msg_id = assoc.next_msg_id();
        let query_vals_resolved = Self::resolve_cli_query(query)?;
        let results = assoc.common().c_find_req(
            &mut reader,
            &mut writer,
            msg_id,
            query_level,
            query_vals_resolved,
        )?;
        let mut stdout = stdout().lock();
        for (i, result) in results.enumerate() {
            let result = result.map_err(AssocError::ab_failure)?;
            if let Some(dcm) = result.1 {
                stdout
                    .write_all(format!("### Result {i}\n").as_ref())
                    .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                let elems = dcm
                    .flatten()
                    .iter()
                    .map(|o| FormattedElement::new(o))
                    .collect::<Vec<FormattedElement>>();
                for elem in elems {
                    stdout
                        .write_all(format!("{elem}\n").as_ref())
                        .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                }
            }
            if !result.0.status().is_pending() {
                stdout
                    .write_all(format!("### End Results: {:?}", result.0.status()).as_ref())
                    .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                break;
            }

            stdout
                .write_all("\n".to_owned().as_ref())
                .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
        }

        assoc.release_association(&mut reader, &mut writer)
    }

    fn issue_c_store(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        my_ae: &str,
        file: &[PathBuf],
    ) -> Result<Option<DimseMsg>, AssocError> {
        for f in file {
            let input = BufReader::with_capacity(
                1024 * 1024,
                File::open(f).map_err(|e| AssocError::ab_failure(DimseError::from(e)))?,
            );
            let parser = ParserBuilder::default().build(input, &STANDARD_DICOM_DICTIONARY);
            let store_msg_id = assoc.next_msg_id();
            let rsp = assoc.common().c_store_req(
                &mut reader,
                &mut writer,
                parser,
                store_msg_id,
                my_ae,
                0,
            )?;
            let Some(DimseMsg::Cmd(cmd)) = rsp else {
                return Ok(rsp);
            };

            if !cmd.status().is_success() {
                return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                    "Transfer of file failed with status: {:?}",
                    *cmd.status()
                ))));
            }
        }

        assoc.release_association(&mut reader, &mut writer)
    }

    fn issue_c_move(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        dest_ae: &str,
        query_level: QueryLevel,
        query: &[(String, String)],
    ) -> Result<Option<DimseMsg>, AssocError> {
        let msg_id = assoc.next_msg_id();
        let query_vals_resolved = Self::resolve_cli_query(query)?;
        let prog_iter = assoc.common().c_move_req(
            &mut reader,
            &mut writer,
            msg_id,
            dest_ae,
            query_level,
            query_vals_resolved,
        )?;

        for prog in prog_iter {
            let (prog, _dataset) = prog.map_err(AssocError::ab_failure)?;
            let prog = SubOpProgress::from(&prog);
            let total = prog.0 + prog.1 + prog.2 + prog.3;
            println!("C-MOVE Progress: {}/{}", prog.1, total);
        }

        assoc.release_association(&mut reader, &mut writer)
    }

    fn issue_c_get(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        query_level: QueryLevel,
        query: &[(String, String)],
    ) -> Result<Option<DimseMsg>, AssocError> {
        let msg_id = assoc.next_msg_id();
        let query_vals_resolved = Self::resolve_cli_query(query)?;

        assoc
            .common()
            .c_get_req(&mut writer, msg_id, query_level, query_vals_resolved)?;

        let mut sop_count = 0;
        let mut cstore_ctx_id: u8 = 0;
        let mut cstore_msg_id: u16 = 0;
        let mut cstore_aff_sop: String = String::default();
        let mut filename = format!("rcv_dcm_{sop_count}.tmp");
        let mut output = BufWriter::with_capacity(
            1024 * 1024,
            File::create(&filename).map_err(|e| AssocError::ab_failure(DimseError::from(e)))?,
        );
        loop {
            match assoc.common().next_msg(&mut reader, &mut writer)? {
                DimseMsg::Cmd(cmd) => {
                    // SCP will respond with individual C-STORE requests for each SOP.
                    if cmd.cmd_type() == &CommandType::CStoreReq {
                        cstore_ctx_id = cmd.ctx_id();
                        cstore_msg_id = cmd.msg_id();
                        cstore_aff_sop = cmd.get_string(&AffectedSOPClassUID).unwrap_or_default();

                        filename = format!("rcv_dcm_{sop_count}.tmp");
                        output = BufWriter::with_capacity(
                            1024 * 1024,
                            File::create(&filename)
                                .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?,
                        );
                        // TODO: Write FileMeta
                        continue;
                    }
                    // SCP may respond with status updates. As long as the status is "pending" this
                    // should continue to expect C-STORE requests.
                    if cmd.cmd_type() == &CommandType::CGetRsp && cmd.status().is_pending() {
                        let progress = SubOpProgress::from(&cmd);
                        println!(
                            "C-GET Progress: {}/{}",
                            progress.0,
                            progress.0 + progress.1 + progress.2 + progress.3
                        );
                        continue;
                    }
                    break;
                }
                DimseMsg::Dataset(pdv) => {
                    output
                        .write_all(pdv.data())
                        .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;

                    if pdv.is_last_fragment() {
                        output
                            .flush()
                            .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                        let file = output.into_inner().map_err(|e| {
                            AssocError::ab_failure(DimseError::OtherError(e.into()))
                        })?;
                        file.sync_all()
                            .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                        drop(file);

                        let (_pres_ctx, ts) = assoc.common().get_pres_ctx_and_ts(cstore_ctx_id)?;

                        rename_file_to_sop(&filename, ts).map_err(AssocError::ab_failure)?;

                        let cmd = CommandMessage::c_store_rsp(
                            cstore_ctx_id,
                            cstore_msg_id,
                            &cstore_aff_sop,
                            &CommandStatus::success(),
                        );
                        assoc.common().write_command(&cmd, &mut writer)?;

                        sop_count += 1;
                        filename = format!("rcv_dcm_{sop_count}.tmp");
                        output = BufWriter::with_capacity(
                            1024 * 1024,
                            File::create_new(&filename)
                                .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?,
                        );
                    }
                }
                other => {
                    eprintln!("Unexpected response: {other:?}");
                    break;
                }
            }
        }

        let _ = std::fs::remove_file(&filename);

        assoc.release_association(&mut reader, &mut writer)
    }

    fn resolve_cli_query<'e>(
        query: &'e [(String, String)],
    ) -> Result<Vec<(&'e Tag, RawValue<'e>)>, AssocError> {
        let mut query_vals_resolved: Vec<(&'e Tag, RawValue<'e>)> = Vec::with_capacity(query.len());
        for (tag, val) in query {
            let tag = TagNode::parse(tag, Some(&STANDARD_DICOM_DICTIONARY))
                .map_err(|e| AssocError::error(DimseError::from(e)))
                .map(|t| STANDARD_DICOM_DICTIONARY.get_tag_by_number(t.tag()))?
                .ok_or_else(|| {
                    AssocError::error(DimseError::GeneralError(format!(
                        "Unable resolve tag: {tag}"
                    )))
                })?;
            let val = RawValue::try_from(StringAndVr(val, tag.implicit_vr().unwrap_or(&LT)))
                .map_err(|e| AssocError::error(DimseError::from(e)))?;
            query_vals_resolved.push((tag, val.clone()));
        }
        Ok(query_vals_resolved)
    }
}

impl CommandApplication for SvcUserApp {
    fn run(&mut self) -> anyhow::Result<()> {
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
        let mut assoc = UserAssocBuilder::default()
            .id(0)
            .my_ae(self.args.my_ae.clone())
            .service_ae(self.args.host_ae.clone())
            .supported_abs(supported_abs)
            .supported_ts(supported_ts)
            .pdu_rcv_max_len(max_pdu_size)
            .build();

        let stream = TcpStream::connect(&self.args.host)?;
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let result = self.start(reader, &mut writer, &mut assoc);
        match result {
            Ok(None) => {}
            Ok(Some(res)) => {
                let output = handle_assoc_result(Ok(res), &mut writer);
                for line in output {
                    match line {
                        Ok(line) => println!("{line}"),
                        Err(line) => eprintln!("{line}"),
                    }
                }
            }
            Err(e) => {
                let _ = e.write(&mut writer);
                eprintln!("Error: {e:?}");
            }
        }
        Ok(())
    }
}

impl SvcUserApp {
    fn start(
        &self,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        assoc: &mut UserAssoc,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let rsp = assoc.request_association(&mut reader, &mut writer)?;
        if let Some(rsp) = rsp {
            return Ok(Some(rsp));
        }

        match &self.args.cmd {
            SvcUserCommand::Echo => Self::issue_c_echo(assoc, reader, writer),
            SvcUserCommand::Find { query_level, query } => {
                Self::issue_c_find(assoc, reader, writer, *query_level, query)
            }
            SvcUserCommand::Store { file } => {
                Self::issue_c_store(assoc, reader, writer, &self.args.my_ae, file)
            }
            SvcUserCommand::Move {
                dest_ae,
                query_level,
                query,
            } => Self::issue_c_move(
                assoc,
                reader,
                writer,
                dest_ae,
                query_level.unwrap_or(QueryLevel::Study),
                query,
            ),
            SvcUserCommand::Get { query_level, query } => Self::issue_c_get(
                assoc,
                reader,
                writer,
                query_level.unwrap_or(QueryLevel::Study),
                query,
            ),
        }
    }
}
