//! DICOM Object, an in-memory tree-like structure representing a DICOM dataset.

use std::collections::btree_map;
use std::collections::BTreeMap;
use std::io::Read;

use crate::core::charset::CSRef;
use crate::core::dcmelement::DicomElement;
use crate::defn::constants::tags;
use crate::defn::dcmdict::DicomDictionary;
use crate::defn::tag::{TagNode, TagPath};
use crate::defn::ts::TSRef;
use crate::defn::vl::ValueLength;

use super::read::ParseError;
use super::read::Parser;
use super::write::error::WriteError;

/// Trait for a dicom node which contains child elements.
pub trait DicomNode {
    /// Get the element for this node, if present. This will be `None` for `DicomRoot` but will
    /// always be `Some()` for all `DicomObject`.
    fn get_element(&self) -> Option<&DicomElement>;

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
        let mut target = tagpath.nodes.first().map(|n| self.get_child_by_tagnode(n))?;
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
            elements.push(dcmobj.as_element());
            elements.append(&mut (dcmobj.flatten()?));
        }
        for (_tag, dcmobj) in self.iter_child_nodes() {
            elements.push(dcmobj.as_element());
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

    /// Parses elements to build a `DicomObject` to represent the parsed dataset as an in-memory tree.
    /// Returns `None` if the parser's first element fails to parse properly, assumed to be a non-DICOM
    /// dataset. Any errors after a successful first element being parsed are returned as `Result::Err`.
    pub fn parse<DatasetType: Read>(
        parser: &mut Parser<'dict, DatasetType>,
    ) -> Result<Option<DicomRoot<'dict>>, ParseError> {
        let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
        let mut items: Vec<DicomObject> = Vec::new();

        let parse_result: Option<Result<DicomElement, ParseError>> =
            DicomRoot::parse_recurse(parser, &mut child_nodes, &mut items, true);

        if !parser.get_behavior().allow_partial_object {
            if let Some(Err(e)) = parse_result {
                return Err(e);
            }
        }

        // If no child nodes and no items were parsed then this isn't valid dicom.
        if child_nodes.is_empty() && items.is_empty() {
            return Ok(None);
        }

        // Copy the parser state only after having parsed elements, to get appropriate transfer syntax
        // and specific character set.
        let root: DicomRoot<'_> = DicomRoot::new(
            parser.get_ts(),
            parser.get_cs(),
            parser.get_dictionary(),
            child_nodes,
            items,
        );
        Ok(Some(root))
    }

    /// Iterates through the parser populating values into the given `child_nodes` map. Elements which
    /// are sequence-like (contain sub-elements) will be recursed into so child elements are added to
    /// their node. The sequence path length is used to determine when parsing an element whether it
    /// escapes the current level a of recursion, and how far back up it should go (the end of a
    /// sequence can be the end of multiple sequences).
    ///
    /// `parser` The parser elements are being read from
    /// `child_nodes` The map of child nodes which elements should be parsed into
    /// `items` The list of nodes which item elements should be parsed into
    /// `is_root_level` Whether the root level is being parsed, or within child nodes
    fn parse_recurse<DatasetType: Read>(
        parser: &mut Parser<'_, DatasetType>,
        child_nodes: &mut BTreeMap<u32, DicomObject>,
        items: &mut Vec<DicomObject>,
        is_root_level: bool,
    ) -> Option<Result<DicomElement, ParseError>> {
        let mut prev_seq_path_len: usize = 0;
        let mut next_element: Option<Result<DicomElement, ParseError>> = parser.next();

        // If the first element at the root level is an error then this is probably not valid dicom.
        if is_root_level {
            if let Some(Err(_)) = next_element {
                return None;
            }
        }

        while let Some(Ok(element)) = next_element {
            let tag: u32 = element.get_tag();
            let cur_seq_path_len: usize = element.get_sequence_path().len() + 1;

            if prev_seq_path_len == 0 {
                prev_seq_path_len = cur_seq_path_len;
            }

            if cur_seq_path_len < prev_seq_path_len {
                // If the next element has a shorter path than the previous one it should not be added
                // to the given node but returned so it can be added to a parent node.
                return Some(Ok(element));
            }

            let mut possible_next_elem: Option<Result<DicomElement, ParseError>> = None;
            // Checking sequence or item tag should match dcmparser.read_dicom_element() which
            // does not read a value for those elements but lets the parser read its value as
            // separate elements which we're considering child elements.
            let dcmobj: DicomObject = if element.is_seq_like()
                || (tag == tags::ITEM && element.get_vl() != ValueLength::Explicit(0))
            {
                let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
                let mut items: Vec<DicomObject> = Vec::new();
                possible_next_elem =
                    DicomRoot::parse_recurse(parser, &mut child_nodes, &mut items, false);
                DicomObject::new_with_children(element, child_nodes, items)
            } else {
                DicomObject::new(element)
            };
            if tag == tags::ITEM {
                items.push(dcmobj);
            } else {
                child_nodes.insert(tag, dcmobj);
            }

            prev_seq_path_len = cur_seq_path_len;

            // If an element was returned from the recursive call that means the recursive call iterated
            // into an element which is not a child of the node we passed into the recursive call and
            // it should instead be added elsewhere up the tree.
            if let Some(Ok(next_elem)) = possible_next_elem {
                let next_seq_path_len: usize = next_elem.get_sequence_path().len() + 1;
                match next_seq_path_len {
                    val if val < cur_seq_path_len => {
                        // If that element is still shorter than the current then it needs passed up further.
                        return Some(Ok(next_elem));
                    }
                    val if val == cur_seq_path_len => {
                        // If it matches the same level as the current node then it should be "inserted"
                        // into the element iteration so it will be checked for being sequence-like and
                        // then added into the current node.
                        next_element = Some(Ok(next_elem));
                        continue;
                    }
                    _ => {}
                }
            } else if let Some(Err(_)) = possible_next_elem {
                // Errors need propagated all the way back up.
                return possible_next_elem;
            }

            // Parse the next element from the dataset.
            next_element = parser.next();
        }

        // Return the last value from the parser which will either be None or Error.
        next_element
    }
}

impl<'dict> DicomNode for DicomRoot<'dict> {
    fn get_element(&self) -> Option<&DicomElement> {
        None
    }

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

    /// Convenience function to get this object's underlying element, to avoid the `Option`
    /// wrapping of `get_element()`.
    pub fn as_element(&self) -> &DicomElement {
        &self.element
    }
}

impl DicomNode for DicomObject {
    fn get_element(&self) -> Option<&DicomElement> {
        Some(&self.element)
    }

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
