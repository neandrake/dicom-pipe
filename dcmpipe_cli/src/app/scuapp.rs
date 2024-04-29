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
        error::{AssocError, DimseError},
    },
};

use crate::{
    app::handle_assoc_result,
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
        assoc.c_echo_rq(&mut reader, &mut writer)?;
        assoc.release_association(&mut reader, &mut writer)
    }

    fn issue_c_find(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        query_level: QueryLevel,
        query: &Vec<(String, String)>,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let mut query_vals_resolved: Vec<(&Tag, RawValue)> = Vec::with_capacity(query.len());
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

        let results =
            assoc.c_find_req(&mut reader, &mut writer, query_level, query_vals_resolved)?;

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
            let f = File::open(f).map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
            let parser = ParserBuilder::default().build(f, &STANDARD_DICOM_DICTIONARY);
            let rsp = assoc.c_store_req(&mut reader, &mut writer, parser, my_ae, 0)?;
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

    #[allow(unused_variables, unused_mut)]
    fn issue_c_move(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        dest_ae: &str,
        query_level: QueryLevel,
        query: &[(String, String)],
    ) -> Result<Option<DimseMsg>, AssocError> {
        assoc.release_association(&mut reader, &mut writer)
    }

    #[allow(unused_variables, unused_mut)]
    fn issue_c_get(
        assoc: &mut UserAssoc,
        mut reader: BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        query_level: QueryLevel,
        query: &[(String, String)],
    ) -> Result<Option<DimseMsg>, AssocError> {
        assoc.release_association(&mut reader, &mut writer)
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
