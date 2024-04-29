//! DICOM Data Element (Tag)

use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use crate::defn::vm::VMRef;
use crate::defn::vr::VRRef;

pub type TagRef = &'static Tag;

/// DICOM Data Element (Tag) Definition
#[derive(Debug, Eq)]
pub struct Tag {
    /// Identifier or name which can be used with a `DicomDictionary`.
    pub ident: &'static str,

    /// The tag number.
    pub tag: u32,

    /// The default value representation which should be used to read this tag when parsing
    /// ImplicitVR transfer syntaxes. Some tags may support multiple possible implicit VRs,
    /// however this is not currently supported.
    pub implicit_vr: Option<VRRef>,

    /// The value multiplicity indicating how many values might be encoded in the element.
    pub vm: VMRef,

    /// A longer name or description of the tag.
    pub desc: &'static str,
}

impl Tag {
    /// Get the tag's identifier.
    pub fn get_ident(&self) -> &'static str {
        self.ident
    }

    /// Get the tag's number.
    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    /// Get the tag's implicit value representation, if it has one.
    pub fn get_implicit_vr(&self) -> Option<VRRef> {
        self.implicit_vr
    }

    /// Get the tag's value multiplicity.
    pub fn get_vm(&self) -> VMRef {
        self.vm
    }

    /// Renders the tag number as `(gggg,eeee)`.
    pub fn format_tag_to_display(tag: u32) -> String {
        let tag_group: u32 = tag >> 16;
        let tag_element: u32 = tag & 0x0000_FFFF;
        format!("({:04X},{:04X})", tag_group, tag_element)
    }

    /// Renders the tag number as `ggggeeee`.
    pub fn format_tag_to_path_display(tag: u32) -> String {
        let tag_group: u32 = tag >> 16;
        let tag_element: u32 = tag & 0x0000_FFFF;
        format!("{:04X}{:04X}", tag_group, tag_element)
    }
}

/// `PartialEq` to allow for ordering of tags based on their number.
impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.tag.eq(&other.tag)
    }
}

/// `Hash` to allow for hashing based on the tag number.
impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

/// A `TagNode` represents a single entry/node within a `TagPath`. All leaf nodes have an
/// associated tag number. Non-leaf nodes are sequence tags, and will also contain an `item`
/// specifying which 1-based item child node is being referenced in the path.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TagNode {
    tag: u32,
    item: Option<usize>,
}

impl TagNode {
    /// Create a new tag node.
    pub fn new(tag: u32, item: Option<usize>) -> TagNode {
        TagNode { tag, item }
    }

    /// Get the tag number for this node.
    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    /// Get the 1-based item number this node references if this is a non-leaf node.
    pub fn get_item(&self) -> Option<usize> {
        self.item
    }

    /// Get the modifiable 1-based item number this node references if this is a non-leaf node.
    pub fn get_item_mut(&mut self) -> &mut Option<usize> {
        &mut self.item
    }
}

impl Debug for TagNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.item {
            None => write!(f, "{}", Tag::format_tag_to_display(self.tag)),
            Some(item) => write!(f, "{}[{}]", Tag::format_tag_to_display(self.tag), item),
        }
    }
}

impl From<u32> for TagNode {
    fn from(tag: u32) -> Self {
        TagNode { tag, item: None }
    }
}

impl From<&u32> for TagNode {
    fn from(tag: &u32) -> Self {
        TagNode {
            tag: *tag,
            item: None,
        }
    }
}

/// A `TagPath` is an ordered collection of `TagNode`s. The path specifies a unique traversal into
/// a DICOM dataset referencing a single DICOM tag/element. Example:
///
/// ```text
/// ReferencedFrameOfReferenceSequence[1]
///   RTReferencedStudySequence[1]
///     RTReferencedSeriesSequence[1]
///       ContourImageSequence[11]
///         ReferencedSOPInstanceUID
/// ```
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TagPath(pub Vec<TagNode>);

impl TagPath {
    pub fn display(&self) -> String {
        self.0
            .iter()
            .map(|node| match node.item {
                None => Tag::format_tag_to_path_display(node.tag),
                Some(item_num) => format!(
                    "{}[{}]",
                    Tag::format_tag_to_path_display(node.tag),
                    item_num
                ),
            })
            .collect::<Vec<String>>()
            .join(".")
    }
}

/// Allows for easily creating a path to a root-level element.
impl From<u32> for TagPath {
    fn from(tag: u32) -> Self {
        vec![TagNode::from(tag)].into()
    }
}

impl From<Vec<TagNode>> for TagPath {
    fn from(nodes: Vec<TagNode>) -> Self {
        TagPath(nodes)
    }
}

/// Convenience to create a `TagPath` from a collection of tag numbers. All tag numbers will be
/// given an item of `1` except the last tag number which will be given an item of `None`.
impl From<Vec<u32>> for TagPath {
    fn from(tags: Vec<u32>) -> Self {
        let mut nodes: Vec<TagNode> = Vec::with_capacity(tags.len());

        for i in 0..tags.len() {
            let item = if i < tags.len() - 1 { Some(1) } else { None };
            let tag: u32 = unsafe { *tags.get_unchecked(i) };
            nodes.push(TagNode::new(tag, item));
        }
        nodes.into()
    }
}

/// Convenience to create a `TagPath` from a collection of tag numbers. All tag numbers will be
/// given an item of `1` except the last tag number which will be given an item of `None`.
impl From<&[u32]> for TagPath {
    fn from(tags: &[u32]) -> Self {
        let mut nodes: Vec<TagNode> = Vec::with_capacity(tags.len());

        for i in 0..tags.len() {
            let item = if i < tags.len() - 1 { Some(1) } else { None };
            let tag: u32 = unsafe { *tags.get_unchecked(i) };
            nodes.push(TagNode::new(tag, item));
        }
        nodes.into()
    }
}
