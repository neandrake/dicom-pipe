use std::io::{Read, Write};

use bson::{doc, Document};
use dcmpipe_lib::{
    core::{
        charset::{CSRef, DEFAULT_CHARACTER_SET},
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        defn::{dcmdict::DicomDictionary, tag::Tag, ts::TSRef, vr::UN},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{AffectedSOPClassUID, MessageID, PatientID, PatientsName, QueryRetrieveLevel},
    },
    dimse::{
        assoc::Association,
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
    },
};

use crate::app::{indexapp::IndexApp, scpapp::AssociationDevice};

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_find_req(
        &mut self,
        cmd: &CommandMessage,
        dcm: &DicomRoot,
    ) -> Result<(), AssocError> {
        let ctx_id = cmd.ctx_id();
        let msg_id = cmd.get_ushort(&MessageID).map_err(AssocError::ab_failure)?;
        let aff_sop_class = cmd
            .get_string(&AffectedSOPClassUID)
            .map_err(AssocError::ab_failure)?;

        let results = &self.query_c_find_results(dcm)?;

        for result in results {
            let res_rsp = Association::create_cfind_result(ctx_id, msg_id, &aff_sop_class, result)?;
            self.assoc.write_pdu(&res_rsp.0, &mut self.writer)?;
            self.assoc.write_pdu(&res_rsp.1, &mut self.writer)?;
        }

        let end_rsp = Association::create_cfind_end(ctx_id, msg_id, &aff_sop_class)?;
        self.assoc.write_pdu(&end_rsp, &mut self.writer)?;

        Ok(())
    }

    fn query_c_find_results(&self, query: &DicomRoot) -> Result<Vec<DicomRoot>, AssocError> {
        let Some(db) = &self.db else {
            return Ok(Self::create_dummy_results(query, query.ts()));
        };
        let coll = IndexApp::get_dicom_coll(db)
            .map_err(|e| AssocError::ab_failure(DimseError::OtherError(e.into())))?;
        let (mongo_query, include_keys) = Self::dcm_query_to_mongo_query(query)?;
        let query_results = IndexApp::query_docs(&coll, Some(mongo_query))
            .map_err(|e| AssocError::ab_failure(DimseError::OtherError(e.into())))?;

        let mut dcm_results: Vec<DicomRoot> = Vec::new();
        for result in query_results {
            let res_root =
                Self::mongo_doc_to_dcm_root(result.doc(), &include_keys, query.ts(), query.cs())?;
            if res_root.get_child_count() > 0 {
                dcm_results.push(res_root);
            }
        }
        Ok(dcm_results)
    }

    fn dcm_query_to_mongo_query(dcm: &DicomRoot) -> Result<(Document, Vec<u32>), AssocError> {
        let mut query = Document::new();
        let mut include_keys: Vec<u32> = Vec::new();
        for elem in dcm.flatten() {
            if elem.tag() == QueryRetrieveLevel.tag() {
                continue;
            }
            let elem_key = IndexApp::tag_to_key(elem.tag());
            include_keys.push(elem.tag());
            if !elem.is_empty() {
                let val = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                if let Some(string) = val.string() {
                    let string = string.replace(['*', '/', '\\', '^'], "");
                    let regex = doc! {
                            "$regex": string,
                            "$options": "i",
                    };
                    query.insert(elem_key, regex);
                }
            }
        }
        Ok((query, include_keys))
    }

    fn mongo_doc_to_dcm_root(
        doc: &Document,
        include_keys: &Vec<u32>,
        ts: TSRef,
        cs: CSRef,
    ) -> Result<DicomRoot, AssocError> {
        let mut res_root = DicomRoot::new_empty(ts, cs);
        for key in include_keys {
            let tag = *key;
            let key = IndexApp::tag_to_key(tag);

            let vr = STANDARD_DICOM_DICTIONARY
                .get_tag_by_number(tag)
                .and_then(Tag::implicit_vr)
                .unwrap_or(&UN);
            let mut res_elem = DicomElement::new_empty(tag, vr, ts);
            if let Some(value) = doc.get(key) {
                if let Some(string) = value.as_str() {
                    res_elem
                        .encode_val(RawValue::of_string(string))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                } else if let Some(int) = value.as_i32() {
                    res_elem
                        .encode_val(RawValue::of_int(int))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                } else if let Some(long) = value.as_i64() {
                    res_elem
                        .encode_val(RawValue::of_long(long))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                } else if let Some(double) = value.as_f64() {
                    res_elem
                        .encode_val(RawValue::of_double(double))
                        .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?;
                }
            }
            if !res_elem.is_empty() {
                res_root.add_element(res_elem);
            }
        }
        Ok(res_root)
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
