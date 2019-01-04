use crate::core::dcmelement::DicomElement;
use std::collections::hash_map::HashMap;
use std::io::{Error, ErrorKind};

pub struct DicomObject {
    /// For root dataset this will be `None`, for sequence elements this will be the sequence
    /// element itself.
    element: Option<DicomElement>,
    /// Child nodes which may be elements of sub-sequences
    child_nodes: HashMap<u32, DicomObject>,
}

impl DicomObject {
    pub fn new_root() -> DicomObject {
        DicomObject {
            element: None,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_with_element(element: DicomElement) -> DicomObject {
        DicomObject {
            element: Some(element),
            child_nodes: HashMap::new(),
        }
    }

    pub fn as_element(&mut self) -> Option<&mut DicomElement> {
        self.element.as_mut()
    }

    pub fn get_object(&mut self, tag: u32) -> Option<&mut DicomObject> {
        self.child_nodes.get_mut(&tag)
    }

    pub fn put_object(&mut self, mut object: DicomObject) -> Result<Option<DicomObject>, Error> {
        let tag: u32;
        if let Some(element) = object.as_element() {
            tag = element.tag;
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Attempting to insert root node as child"));
        }
        Ok(self.child_nodes.insert(tag, object))
    }
}
