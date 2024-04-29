//! DICOM Data Element (Tag)

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::core::dcmsqelem::SequenceElement;
use crate::defn::vm::VMRef;
use crate::defn::vr::VRRef;

use super::constants::tags;
use super::dcmdict::DicomDictionary;

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

    /// Detects if the given tag is a private creator, which is defined to be an odd-numbered group
    /// number with an element number between 0x0010-0x00FF.
    pub fn is_private_creator(tag: u32) -> bool {
        if !Tag::is_private(tag) {
            return false;
        }
        let tag_elem: u32 = tag & 0x0000_FFFF;
        (0x0010..=0x00FF).contains(&tag_elem)
    }

    /// Detects if the given tag is a private group length. These tags are deprecated according to
    /// the dicom standard.
    pub fn is_private_group_length(tag: u32) -> bool {
        if !Tag::is_private(tag) {
            return false;
        }
        Tag::is_group_length(tag)
    }

    /// Detects if the given tag is a group length tag, defined to have an element value of 0.
    pub fn is_group_length(tag: u32) -> bool {
        (tag & 0x0000_FFFF) == 0
    }

    /// Detects if the given tag is a private tag. This is only a basic/rudimentary check and is
    /// not based on previously registered private creators.
    pub fn is_private(tag: u32) -> bool {
        let tag_group: u32 = tag >> 16;
        // See Part 5, Section 7.1:
        // Private Data Elements have an odd Group Number that is not (0001,eeee), (0003,eeee),
        // (0005,eeee), (0007,eeee), or (FFFF,eeee).
        tag_group > 0x0008 && tag_group != 0xFFFF && tag_group % 2 == 1
    }

    /// Renders the tag number as `(gggg,eeee)`.
    pub fn format_tag_to_display(tag: u32) -> String {
        let tag_group: u32 = tag >> 16;
        let tag_elem: u32 = tag & 0x0000_FFFF;
        format!("({:04X},{:04X})", tag_group, tag_elem)
    }

    /// Renders the tag number as `ggggeeee`.
    pub fn format_tag_to_path_display(tag: u32) -> String {
        let tag_group: u32 = tag >> 16;
        let tag_elem: u32 = tag & 0x0000_FFFF;
        format!("{:04X}{:04X}", tag_group, tag_elem)
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

impl From<&SequenceElement> for TagNode {
    fn from(element: &SequenceElement) -> Self {
        TagNode {
            tag: element.get_seq_tag(),
            item: element.get_item_number(),
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
    /// Formats the tag path as readable text, optionally using the tag's display name where
    /// possible, otherwise tags will be displayed as `(gggg,eeee)`.
    pub fn format_tagpath_to_display(
        tagpath: &TagPath,
        dict: Option<&dyn DicomDictionary>,
    ) -> String {
        tagpath
            .0
            .iter()
            // Filter out tags related to items & delimiters as they are markers which are already
            // contextually conveyed by the item number indicators.
            .filter(|node| {
                node.tag != tags::ITEM
                    && node.tag != tags::ITEM_DELIMITATION_ITEM
                    && node.tag != tags::SEQUENCE_DELIMITATION_ITEM
            })
            .map(|node| {
                let tag: String = dict
                    .and_then(|d| d.get_tag_by_number(node.tag))
                    .map_or_else(
                        || Tag::format_tag_to_display(node.tag),
                        |t| t.ident.to_string(),
                    );
                match node.item {
                    None => tag,
                    Some(item_num) => format!("{}[{}]", tag, item_num),
                }
            })
            .collect::<Vec<String>>()
            .join(".")
    }
}

impl Display for TagPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &TagPath::format_tagpath_to_display(self, None))
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

impl From<&[&SequenceElement]> for TagPath {
    fn from(elements: &[&SequenceElement]) -> Self {
        let mut nodes: Vec<TagNode> = Vec::with_capacity(elements.len());
        for elem in elements {
            nodes.push((*elem).into());
        }
        nodes.into()
    }
}

impl From<&[SequenceElement]> for TagPath {
    fn from(elements: &[SequenceElement]) -> Self {
        let mut nodes: Vec<TagNode> = Vec::with_capacity(elements.len());
        for elem in elements {
            nodes.push(elem.into());
        }
        nodes.into()
    }
}

impl From<Vec<SequenceElement>> for TagPath {
    fn from(elements: Vec<SequenceElement>) -> Self {
        elements.as_slice().into()
    }
}

impl From<&Vec<SequenceElement>> for TagPath {
    fn from(elements: &Vec<SequenceElement>) -> Self {
        elements.as_slice().into()
    }
}
