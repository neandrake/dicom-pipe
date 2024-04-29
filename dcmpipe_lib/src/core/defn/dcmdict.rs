use super::{tag::TagRef, ts::TSRef, uid::UIDRef};

/// A DICOM dictionary enables looking up transer syntaxes, tags, and uids by
/// their name or number (as defined in the standard), or by their UID.
pub trait DicomDictionary {
    /// Look up a `TransferSyntax` definition by its name
    fn get_ts_by_name(&self, name: &str) -> Option<TSRef>;
    /// Look up a `TransferSyntax` definition by its UID
    fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef>;

    /// Look up a `Tag` definition by its name
    fn get_tag_by_name(&self, name: &str) -> Option<TagRef>;
    /// Look up a `Tag` definition by its tag number
    fn get_tag_by_number(&self, number: u32) -> Option<TagRef>;

    /// Look up a `UID` definition by its name
    fn get_uid_by_name(&self, name: &str) -> Option<UIDRef>;
    /// Look up a `UID` definition by its UID
    fn get_uid_by_uid(&self, uid: &str) -> Option<UIDRef>;
}

impl core::fmt::Debug for dyn DicomDictionary + '_ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<DicomDictionary>")
    }
}

/// Wraps a list of DICOM dictionaries to be used a single dictionary. The order of the
/// dictionaries affects the resulting lookup if the search key exists in multiple dictionaries.
pub struct MultiDicomDictionary<'d> {
    dicts: Vec<&'d dyn DicomDictionary>,
}

impl<'d> MultiDicomDictionary<'d> {
    pub fn new(dicts: Vec<&'d dyn DicomDictionary>) -> Self {
        Self { dicts }
    }
}

impl<'d> DicomDictionary for MultiDicomDictionary<'d> {
    fn get_ts_by_name(&self, name: &str) -> Option<TSRef> {
        self.dicts
            .iter()
            .flat_map(|d| d.get_ts_by_name(name))
            .nth(0)
    }

    fn get_ts_by_uid(&self, uid: &str) -> Option<TSRef> {
        self.dicts.iter().flat_map(|d| d.get_ts_by_uid(uid)).nth(0)
    }

    fn get_tag_by_name(&self, name: &str) -> Option<TagRef> {
        self.dicts
            .iter()
            .flat_map(|d| d.get_tag_by_name(name))
            .nth(0)
    }

    fn get_tag_by_number(&self, number: u32) -> Option<TagRef> {
        self.dicts
            .iter()
            .flat_map(|d| d.get_tag_by_number(number))
            .nth(0)
    }

    fn get_uid_by_name(&self, name: &str) -> Option<UIDRef> {
        self.dicts
            .iter()
            .flat_map(|d| d.get_uid_by_name(name))
            .nth(0)
    }

    fn get_uid_by_uid(&self, uid: &str) -> Option<UIDRef> {
        self.dicts.iter().flat_map(|d| d.get_uid_by_uid(uid)).nth(0)
    }
}
