use crate::core::charset::CSRef;
use crate::core::dcmelement::DicomElement;
use crate::defn::dcmdict::DicomDictionary;
use crate::defn::ts::TSRef;
use std::collections::btree_map;
use std::collections::BTreeMap;

/// Trait for a dicom node which contains child elements.
pub trait DicomNode {
    /// Get the number of child node if this is a root or sequence-like node
    fn get_child_count(&self) -> usize;
    /// Get a child node with the given tag
    fn get_child(&self, tag: u32) -> Option<&DicomObject>;
    /// Iterator over the child nodes, in tag-ascending order
    fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject>;
    /// Inserts a child node
    fn insert_child(&mut self, object: DicomObject) -> Option<DicomObject>;
    /// Get the number of item nodes if this is a sequence-like node
    fn get_item_count(&self) -> usize;
    /// Get an item of the given index
    fn get_item(&self, index: usize) -> Option<&DicomObject>;
}

/// A root node of a DICOM dataset. It does not represent an element but contains child elements.
pub struct DicomRoot<'dict> {
    ts: TSRef,
    cs: CSRef,
    dictionary: &'dict dyn DicomDictionary,
    child_nodes: BTreeMap<u32, DicomObject>,
}
impl<'dict> DicomRoot<'dict> {
    pub fn new(
        ts: TSRef,
        cs: CSRef,
        dictionary: &dyn DicomDictionary,
        child_nodes: BTreeMap<u32, DicomObject>,
    ) -> DicomRoot<'_> {
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
impl<'dict> DicomNode for DicomRoot<'dict> {
    fn get_child_count(&self) -> usize {
        self.child_nodes.len()
    }

    fn get_child(&self, tag: u32) -> Option<&DicomObject> {
        self.child_nodes.get(&tag)
    }

    fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject> {
        self.child_nodes.iter()
    }

    fn insert_child(&mut self, object: DicomObject) -> Option<DicomObject> {
        let tag: u32 = object.as_element().tag;
        self.child_nodes.insert(tag, object)
    }

    fn get_item_count(&self) -> usize {
        0
    }

    fn get_item(&self, _index: usize) -> Option<&DicomObject> {
        None
    }
}

/// A DICOM object which represents a DICOM Element and may have child elements
pub struct DicomObject {
    /// The element of the current node. For sequence elements this will be the sequence element
    /// itself.
    element: DicomElement,
    /// Child nodes which may be elements of sub-sequences.
    child_nodes: BTreeMap<u32, DicomObject>,
    /// Item nodes don't go into the map since there can be multiple of them.
    items: Vec<DicomObject>,
}
impl DicomObject {
    pub fn new(element: DicomElement) -> DicomObject {
        DicomObject {
            element,
            child_nodes: BTreeMap::new(),
            items: Vec::new(),
        }
    }

    pub fn new_with_children(
        element: DicomElement,
        child_nodes: BTreeMap<u32, DicomObject>,
        items: Vec<DicomObject>,
    ) -> DicomObject {
        DicomObject {
            element,
            child_nodes,
            items,
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

    fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject> {
        self.child_nodes.iter()
    }

    fn insert_child(&mut self, object: DicomObject) -> Option<DicomObject> {
        let tag: u32 = object.as_element().tag;
        self.child_nodes.insert(tag, object)
    }

    fn get_item_count(&self) -> usize {
        self.items.len()
    }

    fn get_item(&self, index: usize) -> Option<&DicomObject> {
        self.items.get(index)
    }
}
