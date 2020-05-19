use dcmpipe_lib::defn::dcmdict::DicomDictionary;
use dcmpipe_lib::defn::tag::TagRef;
use dcmpipe_lib::defn::ts::TSRef;
use dcmpipe_lib::defn::uid::UIDRef;

use crate::dict::lookup::{
    TAG_BY_IDENT, TAG_BY_VALUE, TS_BY_IDENT, TS_BY_UID, UID_BY_IDENT, UID_BY_UID,
};

/// The Standard DICOM Dictionary
pub static STANDARD_DICOM_DICTIONARY: StandardDicomDictionary = StandardDicomDictionary {};

/// The Standard DICOM Dictionary
pub struct StandardDicomDictionary {}
impl DicomDictionary for StandardDicomDictionary {
    fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef> {
        TS_BY_UID.get(uid).copied()
    }

    fn get_ts_by_name(&self, name: &str) -> Option<TSRef> {
        TS_BY_IDENT.get(name).copied()
    }

    fn get_tag_by_number(&self, number: u32) -> Option<TagRef> {
        TAG_BY_VALUE.get(&number).copied()
    }

    fn get_tag_by_name(&self, name: &str) -> Option<TagRef> {
        TAG_BY_IDENT.get(name).copied()
    }

    fn get_uid_by_uid(&self, uid: &str) -> Option<UIDRef> {
        UID_BY_UID.get(uid).copied()
    }

    fn get_uid_by_name(&self, name: &str) -> Option<UIDRef> {
        UID_BY_IDENT.get(name).copied()
    }
}
