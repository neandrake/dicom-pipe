use crate::defn::tag::TagRef;
use crate::defn::ts::TSRef;
use crate::defn::uid::UIDRef;

/// Trait for looking up DICOM component definitions
pub trait DicomDictionary {
    /// Look up a `TransferSyntax` definition by its UID
    fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef>;
    /// Look up a `TransferSyntax` definition by its name
    fn get_ts_by_name(&self, name: &str) -> Option<TSRef>;
    /// Look up a `Tag` definition by its tag number
    fn get_tag_by_number(&self, number: u32) -> Option<TagRef>;
    /// Look up a `Tag` definition by its name
    fn get_tag_by_name(&self, name: &str) -> Option<TagRef>;
    /// Look up a `UID` definition by its UID
    fn get_uid_by_uid(&self, uid: &str) -> Option<UIDRef>;
    /// Look up a `UID` definition by its name
    fn get_uid_by_name(&self, name: &str) -> Option<UIDRef>;
}
