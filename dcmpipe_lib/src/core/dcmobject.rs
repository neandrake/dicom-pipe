use crate::core::charset::CSRef;
use crate::core::dcmelement::DicomElement;
use crate::defn::ts::TSRef;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use crate::defn::dcmdict::DicomDictionary;

/// Trait for a dicom node which contains child elements.
pub trait DicomNode {
    /// Get the number of child node if this is a root or sequence-like node
    fn get_child_count(&self) -> usize;
    /// Get a child node with the given tag
    fn get_child(&self, tag: u32) -> Option<&DicomObject>;
    /// Iterator over the child nodes, in tag-ascending order
    fn iter(&self) -> Iter<u32, DicomObject>;
    /// Inserts a child node
    fn insert_child(&mut self, object: DicomObject) -> Option<DicomObject>;
}

/// A root node of a DICOM dataset. It does not represent an element but contains child elements.
pub struct DicomRoot<'dict> {
    ts: TSRef,
    cs: CSRef,
    dictionary: &'dict dyn DicomDictionary,
    child_nodes: BTreeMap<u32, DicomObject>,
}
impl <'dict> DicomRoot<'dict> {
    pub fn new(
        ts: TSRef,
        cs: CSRef,
        dictionary: &dyn DicomDictionary,
        child_nodes: BTreeMap<u32, DicomObject>,
    ) -> DicomRoot {
        DicomRoot {
            ts,
            cs,
            dictionary,
            child_nodes,
        }
    }

    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    pub fn get_cs(&self) -> CSRef {
        self.cs
    }

    pub fn get_dictionary(&self) -> &'dict dyn DicomDictionary {
        self.dictionary
    }
}
impl <'dict> DicomNode for DicomRoot<'dict> {
    fn get_child_count(&self) -> usize {
        self.child_nodes.len()
    }

    fn get_child(&self, tag: u32) -> Option<&DicomObject> {
        self.child_nodes.get(&tag)
    }

    fn iter(&self) -> Iter<u32, DicomObject> {
        self.child_nodes.iter()
    }

    fn insert_child(&mut self, object: DicomObject) -> Option<DicomObject> {
        let tag: u32 = object.as_element().tag;
        self.child_nodes.insert(tag, object)
    }
}

/// A DICOM object which represents a DICOM Element and may have child elements
pub struct DicomObject {
    /// The element of the current node. For sequence elements this will be the sequence element
    /// itself.
    element: DicomElement,
    /// Child nodes which may be elements of sub-sequences.
    child_nodes: BTreeMap<u32, DicomObject>,
}
impl DicomObject {
    pub fn new(element: DicomElement) -> DicomObject {
        DicomObject {
            element,
            child_nodes: BTreeMap::new(),
        }
    }

    pub fn new_with_children(element: DicomElement, child_nodes: BTreeMap<u32, DicomObject>) -> DicomObject {
        DicomObject {
            element,
            child_nodes,
        }
    }

    pub fn as_element(&self) -> &DicomElement {
        &self.element
    }
}
impl DicomNode for DicomObject {
    fn get_child_count(&self) -> usize {
        self.child_nodes.len()
    }

    fn get_child(&self, tag: u32) -> Option<&DicomObject> {
        self.child_nodes.get(&tag)
    }

    fn iter(&self) -> Iter<u32, DicomObject> {
        self.child_nodes.iter()
    }

    fn insert_child(&mut self, object: DicomObject) -> Option<DicomObject> {
        let tag: u32 = object.as_element().tag;
        self.child_nodes.insert(tag, object)
    }
}
