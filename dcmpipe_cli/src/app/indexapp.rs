use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use bson::ordered::OrderedDocument;
use bson::spec::BinarySubtype;
use bson::{Array, Bson, Document};
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};
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

pub struct IndexApp {
    mongo: String,
    folder: PathBuf,
}

impl IndexApp {
    pub fn new(mongo: String, folder: PathBuf) -> IndexApp {
        IndexApp { mongo, folder }
    }

    fn get_dicom_collection(&self) -> Result<Collection, Error> {
        let mongo_opts: ClientOptions = ClientOptions::parse(&self.mongo).map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Invalid mongo: {}, {:?}", &self.mongo, e),
            )
        })?;
        let client: Client = Client::with_options(mongo_opts).map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Invalid mongo: {}, {:?}", &self.mongo, e),
            )
        })?;
        let metabase_db: Database = client.database("metabase_rs");
        metabase_db.drop(None).map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Invalid mongo: {}, {:?}", &self.mongo, e),
            )
        })?;
        let dicom_coll: Collection = metabase_db.collection("dicom");
        Ok(dicom_coll)
    }

    fn scan_dir(&self, dicom_coll: Collection) -> Result<(), Error> {
        let walkdir = WalkDir::new(&self.folder)
            .into_iter()
            .filter_map(|e| e.ok());

        let mut uid_to_doc: HashMap<String, OrderedDocument> = HashMap::new();
        let parser_builder: ParserBuilder = ParserBuilder::default()
            .tagstop(TagStop::BeforeTag(tags::PixelData.tag))
            .dictionary(&STANDARD_DICOM_DICTIONARY);
        for entry in walkdir {
            if !entry.metadata()?.file_type().is_file() {
                continue;
            }

            let file: File = File::open(entry.path())?;
            let mut parser: Parser<'_, File> = parser_builder.build(file);

            let dcm_root: Option<DicomRoot> = parse_into_object(&mut parser)?;
            if dcm_root.is_none() {
                continue;
            }
            let dcm_root: DicomRoot = dcm_root.unwrap();

            let uid_obj: &DicomObject = dcm_root
                .get_child(tags::SeriesInstanceUID.tag)
                .or_else(|| dcm_root.get_child(tags::SOPInstanceUID.tag))
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "DICOM file has no SeriesInstanceUID or SOPInstanceUID: {:?}",
                            entry.path()
                        ),
                    )
                })?;
            let uid_key: String = uid_obj.as_element().try_into()?;
            let mut dicom_doc: &mut OrderedDocument = uid_to_doc.entry(uid_key).or_default();

            let metadata_doc: &mut Document = dicom_doc
                .entry("metadata".to_owned())
                .or_insert_with(|| OrderedDocument::new().into())
                .as_document_mut()
                .ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Field failure: metadata")
                })?;
            let files_field: &mut Array = metadata_doc
                .entry("files".to_owned())
                .or_insert_with(|| Vec::<String>::new().into())
                .as_array_mut()
                .ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData,"Field failure: metadata.files")
                })?;
            files_field.push(format!("{}", entry.path().display()).into());

            for (_child_tag, child_obj) in dcm_root.iter_child_nodes() {
                let child_elem: &DicomElement = child_obj.as_element();
                if child_elem.is_seq_like() {
                    // TODO: handle sequences
                } else {
                    insert_elem_entry(child_elem, &mut dicom_doc)?;
                }
            }
        }

        let docs: Vec<OrderedDocument> = uid_to_doc
            .into_iter()
            .map(|(_series_uid, doc)| doc)
            .collect::<Vec<OrderedDocument>>();

        dicom_coll.insert_many(docs, None).map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Error inserting into mongo: , {:?}", e),
            )
        })?;

        Ok(())
    }
}

impl CommandApplication for IndexApp {
    fn run(&mut self) -> Result<(), Error> {
        let dicom_coll: Collection = self.get_dicom_collection()?;
        self.scan_dir(dicom_coll)?;

        Ok(())
    }
}

/// Inserts the dicom element entry into the given BSON document
fn insert_elem_entry(elem: &DicomElement, dicom_doc: &mut OrderedDocument) -> Result<(), Error> {
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
