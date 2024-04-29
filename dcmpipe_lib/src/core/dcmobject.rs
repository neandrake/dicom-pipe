//! DICOM Object, an in-memory tree-like structure representing a DICOM dataset.

use std::collections::btree_map;
use std::collections::BTreeMap;

use crate::core::charset::CSRef;
use crate::core::dcmelement::DicomElement;
use crate::defn::dcmdict::DicomDictionary;
use crate::defn::tag::{TagNode, TagPath};
use crate::defn::ts::TSRef;

use super::write::error::WriteError;

/// Trait for a dicom node which contains child elements.
pub trait DicomNode {
    /// Get the number of child node if this is a root or sequence-like node.
    fn get_child_count(&self) -> usize;

    /// Get a child node with the given tag.
    fn get_child_by_tag(&self, tag: u32) -> Option<&DicomObject>;

    /// Iterator over the child nodes, in tag-ascending order.
    fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject>;

    /// Get the number of item nodes if this is a sequence-like node.
    fn get_item_count(&self) -> usize;

    /// Get an item of the given index. Index is 1-based.
    fn get_item_by_index(&self, index: usize) -> Option<&DicomObject>;

    /// Iterator over the item nodes of this node, if it is sequence-like.
    fn iter_items(&self) -> std::slice::Iter<DicomObject>;

    /// Get a child node with the given `TagNode`.
    fn get_child_by_tagnode(&self, node: &TagNode) -> Option<&DicomObject> {
        self.get_child_by_tag(node.get_tag())
            .and_then(|o| match node.get_item() {
                None => Some(o),
                Some(item_num) => o.get_item_by_index(item_num),
            })
    }

    /// Get a child node with the given `TagPath`.
    fn get_child_by_tagpath(&self, tagpath: &TagPath) -> Option<&DicomObject> {
        let mut target = tagpath.nodes.get(0).map(|n| self.get_child_by_tagnode(n))?;
        for node in tagpath.nodes.iter().skip(1) {
            target = target.and_then(|parent| parent.get_child_by_tagnode(node));
        }
        target
    }

    /// Flattens this object into an ordered list of elements as they would appear in a dataset.
    fn flatten(&self) -> Result<Vec<&DicomElement>, WriteError> {
        // TODO: Can this instead return an iterator?

        let mut elements: Vec<&DicomElement> = Vec::new();

        // List items + contents first, as SQ objects will include both items for its contents as
        // well as the sequence delimiter as a child node.
        for dcmobj in self.iter_items() {
            elements.push(dcmobj.get_element());
            elements.append(&mut (dcmobj.flatten()?));
        }
        for (_tag, dcmobj) in self.iter_child_nodes() {
            elements.push(dcmobj.get_element());
            elements.append(&mut (dcmobj.flatten()?));
        }

        Ok(elements)
    }
}

/// A root node of a DICOM dataset. This is the root object returned after parsing a dataset. It
/// does not contain a `DicomElement` itself but will have either children or items.
pub struct DicomRoot<'dict> {
    ts: TSRef,
    cs: CSRef,
    dictionary: &'dict dyn DicomDictionary,
    child_nodes: BTreeMap<u32, DicomObject>,
    items: Vec<DicomObject>,
}

impl<'dict> DicomRoot<'dict> {
    pub fn new(
        ts: TSRef,
        cs: CSRef,
        dictionary: &dyn DicomDictionary,
        child_nodes: BTreeMap<u32, DicomObject>,
        items: Vec<DicomObject>,
    ) -> DicomRoot<'_> {
        DicomRoot {
            ts,
            cs,
            dictionary,
            child_nodes,
            items,
        }
    }

    /// Get the transfer syntax used to encode the dataset.
    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    /// Get the character set used to encode string values.
    pub fn get_cs(&self) -> CSRef {
        self.cs
    }

    /// Get the dictionary used to encode the dataset.
    pub fn get_dictionary(&self) -> &'dict dyn DicomDictionary {
        self.dictionary
    }

}

impl<'dict> DicomNode for DicomRoot<'dict> {
    fn get_child_count(&self) -> usize {
        self.child_nodes.len()
    }

    fn get_child_by_tag(&self, tag: u32) -> Option<&DicomObject> {
        self.child_nodes.get(&tag)
    }

    fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject> {
        self.child_nodes.iter()
    }

    fn get_item_count(&self) -> usize {
        self.items.len()
    }

    fn get_item_by_index(&self, index: usize) -> Option<&DicomObject> {
        // Index for items are provided 1-based, convert to 0-based for vec use.
        let index = index - 1;
        self.items.get(index)
    }

    fn iter_items(&self) -> std::slice::Iter<DicomObject> {
        self.items.iter()
    }
}

impl std::fmt::Debug for DicomRoot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<DicomRoot {} {}>",
            self.get_ts().get_uid().get_name(),
            self.cs.name()
        )
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

    /// Gets the underlying `DicomElement` for this `DicomObject`
    pub fn get_element(&self) -> &DicomElement {
        &self.element
    }
}

impl DicomNode for DicomObject {
    fn get_child_count(&self) -> usize {
        self.child_nodes.len()
    }

    fn get_child_by_tag(&self, tag: u32) -> Option<&DicomObject> {
        self.child_nodes.get(&tag)
    }

    fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject> {
        self.child_nodes.iter()
    }

    fn get_item_count(&self) -> usize {
        self.items.len()
    }

    fn get_item_by_index(&self, index: usize) -> Option<&DicomObject> {
        self.items.get(index - 1)
    }

    fn iter_items(&self) -> std::slice::Iter<DicomObject> {
        self.items.iter()
    }
}
