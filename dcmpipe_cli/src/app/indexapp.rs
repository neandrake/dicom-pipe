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

use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use bson::{doc, oid::ObjectId, spec::BinarySubtype, Array, Binary, Bson, Document};
use mongodb::sync::{Client, Collection, Cursor, Database};
use walkdir::WalkDir;

use anyhow::{anyhow, Context, Result};
use dcmpipe_lib::{
    core::{
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        read::{stop::ParseStop, Parser, ParserBuilder},
        RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{PixelData, SOPInstanceUID, SeriesInstanceUID},
    },
};

use crate::{
    app::CommandApplication,
    args::{IndexArgs, IndexCommand},
};

static SERIES_UID_KEY: &str = "0020000E";
static SOP_UID_KEY: &str = "00080018";

static DATABASE_NAME: &str = "dicom_database";
static COLLECTION_NAME: &str = "series";

static MONGO_ID_KEY: &str = "_id";

/// Tracks a dicom document scanned from disk or from the database. I was originally going to make
/// this an enum with variants `FromDisk` and `FromDb` and then try to merge so that the same
/// record is updated from disk contents rather than creating new records, however it was easier
/// for the moment to just make an optional `id` field -- which if filled in means that it will
/// correspond to an existing record otherwise it represents a brand new document.
pub struct DicomDoc {
    key: String,
    doc: Document,
    id: Option<ObjectId>,
}

impl DicomDoc {
    pub fn new(key: String) -> DicomDoc {
        DicomDoc {
            key,
            doc: Document::new(),
            id: None,
        }
    }

    #[must_use]
    pub fn doc(&self) -> &Document {
        &self.doc
    }
}

pub struct IndexApp {
    args: IndexArgs,
}

impl CommandApplication for IndexApp {
    fn run(&mut self) -> Result<()> {
        match &self.args.cmd {
            IndexCommand::Scan { folder } => {
                let folder = folder.clone();
                let uid_to_doc: HashMap<String, DicomDoc> = Self::scan_dir(folder)?;
                Self::upsert_records(&self.args.db, uid_to_doc)?;
            }
            IndexCommand::Verify {} => {
                Self::verify_records(&self.args.db)?;
            }
        }
        Ok(())
    }
}

impl IndexApp {
    pub fn new(args: IndexArgs) -> IndexApp {
        IndexApp { args }
    }

    pub fn get_dicom_coll(db: impl AsRef<str>) -> Result<Collection<Document>> {
        let client: Client = Client::with_uri_str(db.as_ref())
            .with_context(|| format!("Invalid database URI: {}", db.as_ref()))?;
        let database: Database = client.database(DATABASE_NAME);
        Ok(database.collection(COLLECTION_NAME))
    }

    /// Scans a directory and returns the map of all scanned documents
    fn scan_dir(folder: PathBuf) -> Result<HashMap<String, DicomDoc>> {
        let mut uid_to_doc: HashMap<String, DicomDoc> = HashMap::new();

        let walkdir = WalkDir::new(folder).into_iter().filter_map(Result::ok);

        let parser_builder = ParserBuilder::default().stop(ParseStop::before(&PixelData));
        for entry in walkdir {
            if !entry.metadata()?.file_type().is_file() {
                continue;
            }

            let file: File = File::open(entry.path())?;
            let mut parser: Parser<'_, File> =
                parser_builder.build(file, &STANDARD_DICOM_DICTIONARY);

            let dcm_root = DicomRoot::parse(&mut parser)?;
            let Some(dcm_root) = dcm_root else {
                continue;
            };

            let uid_key = dcm_root
                .get_value_by_tag(&SeriesInstanceUID)
                .or_else(|| dcm_root.get_value_by_tag(&SOPInstanceUID))
                .and_then(|v| v.string().cloned())
                .unwrap_or_default();

            let entry_key: String = uid_key.clone();
            let dicom_doc: &mut DicomDoc = uid_to_doc
                .entry(entry_key)
                .or_insert_with(|| DicomDoc::new(uid_key.clone()));

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
            metadata_doc.insert("serieskey", uid_key);

            for (_child_tag, child_obj) in dcm_root.iter_child_nodes() {
                let child_elem: &DicomElement = child_obj.element();
                if child_elem.is_sq_like() {
                    // TODO: handle sequences
                } else {
                    Self::insert_elem_entry(child_elem, &mut dicom_doc.doc)?;
                }
            }
        }

        Ok(uid_to_doc)
    }

    /// Queries mongo for existing documents and updates `self.uid_to_doc` with a related id field
    /// if appropriate, or marks the document as missing on-disk and then deletes it.
    /// Performs all updates to mongo based on the scan results.
    fn upsert_records(db: &String, mut uid_to_doc: HashMap<String, DicomDoc>) -> Result<()> {
        let dicom_coll: Collection<Document> = Self::get_dicom_coll(db)?;

        let mut serieskeys: Vec<Bson> = Vec::new();
        for key in uid_to_doc.keys() {
            serieskeys.push(Bson::String(key.clone()));
        }
        let query: Document = doc! {
            "metadata.serieskey" : {
                "$in" : serieskeys
            }
        };

        for dicom_doc in Self::query_docs(&dicom_coll, Some(query))? {
            if let Some(existing) = uid_to_doc.get_mut(&dicom_doc.key) {
                existing.id = dicom_doc.id;
            }
        }

        let mut inserts: Vec<Document> = Vec::new();
        let mut updates: Vec<(ObjectId, Document)> = Vec::new();
        let keys: Vec<String> = uid_to_doc.keys().cloned().collect();
        for key in keys {
            if let Some(mut dicom_doc) = uid_to_doc.remove(&key) {
                match dicom_doc.id {
                    None => inserts.push(dicom_doc.doc),
                    Some(id) => {
                        dicom_doc.doc.insert(MONGO_ID_KEY, id);
                        updates.push((id, dicom_doc.doc));
                    }
                }
            }
        }

        if !inserts.is_empty() {
            println!("Inserting {} records", inserts.len());
            dicom_coll.insert_many(inserts, None)?;
        }

        // There's no API for mass replacing documents, so do one-by-one.
        println!("Updating {} records", updates.len());
        for (id, doc) in updates {
            let query: Document = doc! { MONGO_ID_KEY: id };
            dicom_coll.update_one(query, doc, None)?;
        }

        Ok(())
    }

    fn verify_records(db: &String) -> Result<()> {
        let dicom_coll: Collection<Document> = Self::get_dicom_coll(db)?;

        let mut record_count: usize = 0;
        let mut updated_records: Vec<Document> = Vec::new();
        let mut missing_records: Vec<Document> = Vec::new();
        for mut dicom_doc in Self::query_docs(&dicom_coll, None)? {
            record_count += 1;
            let metadata_doc = dicom_doc
                .doc
                .get_mut("metadata")
                .and_then(|md_doc| md_doc.as_document_mut());
            let Some(metadata_doc) = metadata_doc else {
                missing_records.push(dicom_doc.doc);
                continue;
            };

            let files_array = metadata_doc
                .get_mut("files")
                .and_then(|fd_doc| fd_doc.as_array_mut());
            let Some(files_array) = files_array else {
                missing_records.push(dicom_doc.doc);
                continue;
            };

            let num_files: usize = files_array.len();
            files_array.retain(|bson| match bson.as_str() {
                None => false,
                Some(path) => Path::new(path).is_file(),
            });

            match files_array.len() {
                0 => missing_records.push(dicom_doc.doc),
                x if x != num_files => updated_records.push(dicom_doc.doc),
                _ => {}
            }
        }

        println!("Verified {record_count} records");

        println!("Updating {} records", updated_records.len());
        if !updated_records.is_empty() {
            dicom_coll.insert_many(updated_records, None)?;
        }

        println!("Removing {} records", missing_records.len());
        if !missing_records.is_empty() {
            let ids: Vec<Bson> = missing_records
                .iter()
                .filter_map(|doc| doc.get_object_id(MONGO_ID_KEY).ok())
                .map(Bson::from)
                .collect::<Vec<Bson>>();

            let query = doc! {
                MONGO_ID_KEY : {
                    "$in": ids
                }
            };
            dicom_coll.delete_many(query, None)?;
        }

        Ok(())
    }

    /// Query for dicom records in the given collection and returns an iterator over `DicomDoc`.
    ///
    /// # Params
    /// `db` The database connection string, used for logging only.
    /// `dicom_coll` The collection to query.
    /// `query` The query to use. If `None`, then a blank query is issued, resulting in all records.
    ///
    /// # Errors
    /// I/O errors may occur communicating with mongodb.
    pub fn query_docs(
        dicom_coll: &Collection<Document>,
        query: Option<Document>,
    ) -> Result<impl Iterator<Item = DicomDoc>> {
        let all_dicom_docs: Cursor<Document> = dicom_coll
            .find(query, None)
            .with_context(|| format!("Invalid database: {dicom_coll:?}"))?;

        let doc_iter = all_dicom_docs.filter_map(|doc_res| {
            let doc: Document = match doc_res {
                Err(_e) => return None,
                Ok(d) => d,
            };

            let doc_id_res = doc.get_object_id(MONGO_ID_KEY);
            let doc_id: ObjectId = match doc_id_res {
                Err(_e) => return None,
                Ok(d) => d,
            };

            let doc_key_res = doc
                .get_str(SERIES_UID_KEY)
                .or_else(|_| doc.get_str(SOP_UID_KEY));
            let doc_key: String = match doc_key_res {
                Err(_e) => return None,
                Ok(d) => d.to_owned(),
            };

            Some(DicomDoc {
                key: doc_key,
                doc,
                id: Some(doc_id),
            })
        });

        Ok(doc_iter)
    }

    pub fn tag_to_key(tag: u32) -> String {
        let tag_group: u32 = tag >> 16;
        let tag_elem: u32 = tag & 0x0000_FFFF;
        format!("{tag_group:04X}{tag_elem:04X}")
    }

    /// Builds a bson value from the given `DicomElement` and inserts it into the bson document
    #[allow(clippy::too_many_lines)]
    fn insert_elem_entry(elem: &DicomElement, dicom_doc: &mut Document) -> Result<()> {
        let key: String = Self::tag_to_key(elem.tag());
        let raw_value: RawValue = elem.parse_value()?;
        match raw_value {
            RawValue::Attributes(attrs) => {
                if attrs.len() == 1 {
                    dicom_doc.insert(key, attrs[0].0);
                } else if !attrs.is_empty() {
                    let attrs = attrs.into_iter().map(|a| a.0).collect::<Vec<u32>>();
                    dicom_doc.insert(key, attrs);
                }
            }
            RawValue::Uid(uid) => {
                dicom_doc.insert(key, uid);
            }
            RawValue::Floats(floats) => {
                if floats.len() == 1 {
                    dicom_doc.insert(key, floats[0]);
                } else if !floats.is_empty() {
                    dicom_doc.insert(key, floats);
                }
            }
            RawValue::Doubles(doubles) => {
                if doubles.len() == 1 {
                    dicom_doc.insert(key, doubles[0]);
                } else if !doubles.is_empty() {
                    dicom_doc.insert(key, doubles);
                }
            }
            RawValue::Shorts(shorts) => {
                // convert to i32 because Bson doesn't support i16
                let shorts: Vec<i32> = shorts.into_iter().map(i32::from).collect::<Vec<i32>>();
                if shorts.len() == 1 {
                    dicom_doc.insert(key, shorts[0]);
                } else if !shorts.is_empty() {
                    dicom_doc.insert(key, shorts);
                }
            }
            RawValue::UShorts(ushorts) => {
                if ushorts.len() == 1 {
                    dicom_doc.insert(key, u32::from(ushorts[0]));
                } else if !ushorts.is_empty() {
                    let uints = ushorts
                        .into_iter()
                        .map(|ushort: u16| u32::from(ushort))
                        .collect::<Vec<u32>>();
                    dicom_doc.insert(key, uints);
                }
            }
            RawValue::Ints(ints) => {
                if ints.len() == 1 {
                    dicom_doc.insert(key, ints[0]);
                } else if !ints.is_empty() {
                    dicom_doc.insert(key, ints);
                }
            }
            RawValue::UInts(uints) => {
                if uints.len() == 1 {
                    dicom_doc.insert(key, uints[0]);
                } else if !uints.is_empty() {
                    dicom_doc.insert(key, uints);
                }
            }
            RawValue::Strings(strings) => {
                if strings.len() == 1 {
                    dicom_doc.insert(key, strings[0].clone());
                } else if !strings.is_empty() {
                    dicom_doc.insert(key, strings);
                }
            }
            RawValue::Bytes(bytes) => {
                if !bytes.is_empty() {
                    let bytes: Vec<u8> = bytes.into_iter().take(16).collect::<Vec<u8>>();
                    let binary = Bson::Binary(Binary {
                        subtype: BinarySubtype::Generic,
                        bytes,
                    });
                    dicom_doc.insert(key, binary);
                }
            }
            RawValue::Longs(longs) => {
                if longs.len() == 1 {
                    dicom_doc.insert(key, longs[0]);
                } else if !longs.is_empty() {
                    dicom_doc.insert(key, longs);
                }
            }
            RawValue::ULongs(ulongs) => {
                let mut ulongs = ulongs
                    .into_iter()
                    .map(|u| u.to_string())
                    .collect::<Vec<String>>();
                if ulongs.len() == 1 {
                    dicom_doc.insert(key, ulongs.remove(0));
                } else if !ulongs.is_empty() {
                    dicom_doc.insert(key, ulongs);
                }
            }
            RawValue::Words(words) => {
                if words.len() == 1 {
                    dicom_doc.insert(key, u32::from(words[0]));
                } else if !words.is_empty() {
                    let words: Vec<u32> = words.into_iter().map(u32::from).collect::<Vec<u32>>();
                    dicom_doc.insert(key, words);
                }
            }
            RawValue::DWords(dwords) => {
                if dwords.len() == 1 {
                    dicom_doc.insert(key, dwords[0]);
                } else if !dwords.is_empty() {
                    dicom_doc.insert(key, dwords);
                }
            }
            RawValue::QWords(qwords) => {
                let mut qwords = qwords
                    .into_iter()
                    .map(|u| u.to_string())
                    .collect::<Vec<String>>();
                if qwords.len() == 1 {
                    dicom_doc.insert(key, qwords.remove(0));
                } else if !qwords.is_empty() {
                    dicom_doc.insert(key, qwords);
                }
            }
            RawValue::BytesView(bytes) => {
                if !bytes.is_empty() {
                    let bytes: Vec<u8> = bytes.iter().copied().take(16).collect::<Vec<u8>>();
                    let binary = Bson::Binary(Binary {
                        subtype: BinarySubtype::Generic,
                        bytes,
                    });
                    dicom_doc.insert(key, binary);
                }
            }
        }

        Ok(())
    }
}
