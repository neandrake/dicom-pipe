use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::path::PathBuf;

use bson::oid::ObjectId;
use bson::spec::BinarySubtype;
use bson::{doc, Array, Bson, Document};
use mongodb::sync::{Client, Collection, Cursor, Database};
use walkdir::WalkDir;

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_dict::dict::tags;
use dcmpipe_lib::core::dcmelement::{DicomElement, RawValue};
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::dcmparser::{Parser, ParserBuilder};
use dcmpipe_lib::core::dcmparser_util::parse_into_object;
use dcmpipe_lib::core::tagstop::TagStop;
use dcmpipe_lib::defn::tag::Tag;

use crate::app::CommandApplication;

/// Tracks a dicom document scanned from disk or from the database. I was originally going to make
/// this an enum with variants `FromDisk` and `FromDb` and then try to merge so that the same
/// record is updated from disk contents rather than creating new records, however it was easier
/// for the moment to just make an optional `id` field -- which if filled in means that it will
/// correspond to an existing record otherwise it represents a brand new document.
struct DicomDoc {
    doc: Document,
    id: Option<ObjectId>,
}

impl DicomDoc {
    pub fn new() -> DicomDoc {
        DicomDoc {
            doc: Document::new(),
            id: None,
        }
    }
}

pub struct IndexApp {
    mongo: String,
    folder: PathBuf,
    uid_to_doc: HashMap<String, DicomDoc>,
}

impl CommandApplication for IndexApp {
    fn run(&mut self) -> Result<()> {
        self.scan_dir()?;
        self.update_mongo()?;
        Ok(())
    }
}

impl IndexApp {
    pub fn new(mongo: String, folder: PathBuf) -> IndexApp {
        IndexApp {
            mongo,
            folder,
            uid_to_doc: HashMap::new(),
        }
    }

    /// Scans a directory and builds `DicomDoc` to be inserted into `self.uid_to_doc`
    fn scan_dir(&mut self) -> Result<()> {
        let walkdir = WalkDir::new(&self.folder)
            .into_iter()
            .filter_map(|e| e.ok());

        let parser_builder: ParserBuilder<'_> = ParserBuilder::default()
            .tagstop(TagStop::BeforeTag(tags::PixelData.tag))
            .dictionary(&STANDARD_DICOM_DICTIONARY);
        for entry in walkdir {
            if !entry.metadata()?.file_type().is_file() {
                continue;
            }

            let file: File = File::open(entry.path())?;
            let mut parser: Parser<'_, File> = parser_builder.build(file);

            let dcm_root: Option<DicomRoot<'_>> = parse_into_object(&mut parser)?;
            if dcm_root.is_none() {
                continue;
            }
            let dcm_root: DicomRoot<'_> = dcm_root.unwrap();

            let uid_obj: &DicomObject = dcm_root
                .get_child(tags::SeriesInstanceUID.tag)
                .or_else(|| dcm_root.get_child(tags::SOPInstanceUID.tag))
                .ok_or_else(|| {
                    anyhow!(
                        "DICOM file has no SeriesInstanceUID or SOPInstanceUID: {:?}",
                        entry.path().display()
                    )
                })?;
            let uid_key: String = uid_obj.as_element().try_into()?;
            let dicom_doc: &mut DicomDoc =
                self.uid_to_doc.entry(uid_key).or_insert_with(DicomDoc::new);

            let metadata_doc: &mut Document = dicom_doc
                .doc
                .entry("metadata".to_owned())
                .or_insert_with(|| Document::new().into())
                .as_document_mut()
                .ok_or_else(|| anyhow!("Field failure: metadata"))?;
            let files_field: &mut Array = metadata_doc
                .entry("files".to_owned())
                .or_insert_with(|| Vec::<String>::new().into())
                .as_array_mut()
                .ok_or_else(|| anyhow!("Field failure: metadata.files"))?;
            files_field.push(format!("{}", entry.path().display()).into());

            for (_child_tag, child_obj) in dcm_root.iter_child_nodes() {
                let child_elem: &DicomElement = child_obj.as_element();
                if child_elem.is_seq_like() {
                    // TODO: handle sequences
                } else {
                    insert_elem_entry(child_elem, &mut dicom_doc.doc)?;
                }
            }
        }

        Ok(())
    }

    /// Queries mongo for existing documents and updates `self.uid_to_doc` with a related id field
    /// if appropriate, or marks the document as missing on-disk and then deletes it.
    /// Performs all updates to mongo based on the scan results.
    fn update_mongo(&mut self) -> Result<()> {
        let client: Client = Client::with_uri_str(&self.mongo)
            .with_context(|| format!("Invalid mongo: {}", &self.mongo))?;
        let metabase_db: Database = client.database("metabase_rs");
        let dicom_coll: Collection = metabase_db.collection("dicom");

        let series_uid_key: String = Tag::format_tag_to_path_display(tags::SeriesInstanceUID.tag);
        let sop_uid_key: String = Tag::format_tag_to_path_display(tags::SOPInstanceUID.tag);
        let all_dicom_docs: Cursor = dicom_coll
            .find(None, None)
            .with_context(|| format!("Invalid mongo: {}", &self.mongo))?;

        let mut missing_records: Vec<Document> = Vec::new();
        for doc in all_dicom_docs {
            if let Err(_e) = doc {
                // TODO: log
                continue;
            }
            let doc: Document = doc.unwrap();
            let doc_id = doc.get_object_id("_id");
            if let Err(_e) = doc_id {
                // TODO: log
                continue;
            }
            let doc_id: ObjectId = doc_id.unwrap().clone();

            let doc_key = doc
                .get_str(&series_uid_key)
                .or_else(|_| doc.get_str(&sop_uid_key));
            if let Err(_e) = doc_key {
                // TODO: log
                continue;
            }
            let doc_key: &str = doc_key.unwrap();
            match self.uid_to_doc.get_mut(doc_key) {
                Some(dicom_doc) => dicom_doc.id = Some(doc_id),
                None => missing_records.push(doc),
            }
        }

        // There's api for insert_many or update_many but there doesn't appear to be a way to do
        // something like upsert_many. The update_many does take an option to specify upsert
        // behavior but it's unclear how that should be used, as the query for which documents
        // to update would need to match all existing? For now just do individual updates.
        let keys: Vec<String> = self.uid_to_doc.keys().cloned().collect();
        for key in keys {
            if let Some(mut dicom_doc) = self.uid_to_doc.remove(&key) {
                match dicom_doc.id {
                    None => {
                        dicom_coll.insert_one(dicom_doc.doc, None)?;
                    }
                    Some(doc_id) => {
                        dicom_doc.doc.insert("_id", doc_id.clone());
                        let mut query: Document = Document::new();
                        query.insert("_id", doc_id);
                        dicom_coll.update_one(query, dicom_doc.doc, None)?;
                    }
                }
            }
        }

        if !missing_records.is_empty() {
            let ids: Vec<ObjectId> = missing_records
                .iter()
                .filter_map(|doc| doc.get_object_id("_id").ok())
                .cloned()
                .collect();
            let query = doc! {
                "$or": ids,
            };

            dicom_coll.delete_many(query, None)?;
        }

        Ok(())
    }
}

/// Builds a bson value from the given `DicomElement` and inserts it into the bson document
fn insert_elem_entry(elem: &DicomElement, dicom_doc: &mut Document) -> Result<()> {
    let key: String = Tag::format_tag_to_path_display(elem.tag);
    let raw_value: RawValue = elem.parse_value()?;
    match raw_value {
        RawValue::Attribute(attr) => {
            dicom_doc.insert(key, attr.0);
        }
        RawValue::Uid(uid) => {
            dicom_doc.insert(key, uid);
        }
        RawValue::Doubles(doubles) => {
            if !doubles.is_empty() {
                if doubles.len() == 1 {
                    dicom_doc.insert(key, doubles[0]);
                } else {
                    dicom_doc.insert(key, doubles);
                }
            }
        }
        RawValue::Shorts(shorts) => {
            if !shorts.is_empty() {
                // convert to i32 because Bson doesn't support i16
                let shorts: Vec<i32> = shorts.into_iter().map(i32::from).collect::<Vec<i32>>();
                if shorts.len() == 1 {
                    dicom_doc.insert(key, shorts[0]);
                } else {
                    dicom_doc.insert(key, shorts);
                }
            }
        }
        RawValue::Integers(ints) => {
            if !ints.is_empty() {
                if ints.len() == 1 {
                    dicom_doc.insert(key, ints[0]);
                } else {
                    dicom_doc.insert(key, ints);
                }
            }
        }
        RawValue::UnsignedIntegers(uints) => {
            if !uints.is_empty() {
                if uints.len() == 1 {
                    dicom_doc.insert(key, uints[0]);
                } else {
                    dicom_doc.insert(key, uints);
                }
            }
        }
        RawValue::Strings(strings) => {
            if !strings.is_empty() {
                if strings.len() == 1 {
                    dicom_doc.insert(key, strings[0].clone());
                } else {
                    dicom_doc.insert(key, strings);
                }
            }
        }
        RawValue::Bytes(bytes) => {
            let bytes: Vec<u8> = bytes.into_iter().take(16).collect::<Vec<u8>>();
            let binary = Bson::Binary(BinarySubtype::Generic, bytes);
            dicom_doc.insert(key, binary);
        }
    }

    Ok(())
}
