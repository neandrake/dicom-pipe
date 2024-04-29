//! DICOM Data Element (Tag)

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::core::dcmsqelem::SequenceElement;
use crate::core::read::ParseError;
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
    pub fn is_private_creator<T>(tag: T) -> bool
    where
        T: Into<u32>,
    {
        let tag: u32 = tag.into();
        if !Tag::is_private(tag) {
            return false;
        }
        let tag_elem: u32 = tag & 0x0000_FFFF;
        (0x0010..=0x00FF).contains(&tag_elem)
    }

    /// Detects if the given tag is a private group length. These tags are deprecated according to
    /// the dicom standard.
    pub fn is_private_group_length<T>(tag: T) -> bool
    where
        T: Into<u32>,
    {
        let tag: u32 = tag.into();
        if !Tag::is_private(tag) {
            return false;
        }
        Tag::is_group_length(tag)
    }

    /// Detects if the given tag is a group length tag, defined to have an element value of 0.
    pub fn is_group_length<T>(tag: T) -> bool
    where
        T: Into<u32>,
    {
        let tag: u32 = tag.into();
        (tag & 0x0000_FFFF) == 0
    }

    /// Detects if the given tag is a private tag. This is only a basic/rudimentary check and is
    /// not based on previously registered private creators.
    pub fn is_private<T>(tag: T) -> bool
    where
        T: Into<u32>,
    {
        let tag: u32 = tag.into();
        let tag_group: u32 = tag >> 16;
        // See Part 5, Section 7.1:
        // Private Data Elements have an odd Group Number that is not (0001,eeee), (0003,eeee),
        // (0005,eeee), (0007,eeee), or (FFFF,eeee).
        tag_group > 0x0008 && tag_group != 0xFFFF && tag_group % 2 == 1
    }

    /// Renders the tag number as `(gggg,eeee)`.
    pub fn format_tag_to_display<T>(tag: T) -> String
    where
        T: Into<u32>,
    {
        let tag: u32 = tag.into();
        let tag_group: u32 = tag >> 16;
        let tag_elem: u32 = tag & 0x0000_FFFF;
        format!("({:04X},{:04X})", tag_group, tag_elem)
    }

    /// Renders the tag number as `ggggeeee`.
    pub fn format_tag_to_path_display<T>(tag: T) -> String
    where
        T: Into<u32>,
    {
        let tag: u32 = tag.into();
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

/// Access the numeric tag, to allow functions to take `Into<u32>` rather than plain u32.
impl From<&Tag> for u32 {
    fn from(value: &Tag) -> Self {
        value.tag
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

    /// Parses a `TagNode` from the given string. The tag can be resolved by name if a dictionary
    /// is supplied, or by standard hexformat (parens are optional). For a `TagNode` which is in a
    /// sequence path, an index can be supplied which must be at the end and contained within
    /// square brackets. Supplying a dictionary is optional however must be supplied in order to
    /// resolve tags by name.
    ///
    /// The acceptable formats are:
    /// ```text
    /// "PatientID" => (0x0010_0020, None)
    /// "(0010,0020)" => (0x0010_0020, None)
    /// "0010,0020" => (0x0010_0020, None)
    /// "ReferencedFrameOfReferenceSequence[1]" => (0x3006_0010, Some(1))
    /// "(3006,0010)[1]" => (0x3006_0010, Some(1))
    /// ```
    pub fn from_str(value: &str, dict: Option<&dyn DicomDictionary>) -> Result<Self, ParseError> {
        let value = value.trim();
        let mut index = None;
        let mut tag_id = value;
        if let Some((name_part, index_part)) = value.rsplit_once('[') {
            if let Some((index_part, _bracket)) = index_part.rsplit_once(']') {
                if let Ok(parsed) = index_part.parse::<usize>() {
                    index = Some(parsed);
                }
            }
            tag_id = name_part;
        }

        let lookup = dict.and_then(|d| d.get_tag_by_name(tag_id));
        if let Some(tag) = lookup {
            Ok(TagNode::new(tag.tag, index))
        } else if let Some((group, tag)) = value.trim_matches(&['(', ')']).split_once(',') {
            let group = u16::from_str_radix(group, 16).map_err(|e| ParseError::InvalidTagPath {
                string_path: e.to_string(),
            })?;
            let tag = u16::from_str_radix(tag, 16).map_err(|e| ParseError::InvalidTagPath {
                string_path: e.to_string(),
            })?;
            let full_tag: u32 = ((group as u32) << 16) + ((tag as u32) & 0x0000_FFFF);
            Ok(TagNode::new(full_tag, index))
        } else {
            Err(ParseError::InvalidTagPath {
                string_path: value.to_string(),
            })
        }
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

impl From<(u32, Option<usize>)> for TagNode {
    fn from(tuple: (u32, Option<usize>)) -> Self {
        TagNode {
            tag: tuple.0,
            item: tuple.1,
        }
    }
}

impl From<(&Tag, Option<usize>)> for TagNode {
    fn from(tuple: (&Tag, Option<usize>)) -> Self {
        TagNode {
            tag: tuple.0.tag,
            item: tuple.1,
        }
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

impl From<&Tag> for TagNode {
    fn from(value: &Tag) -> Self {
        TagNode {
            tag: value.tag,
            item: None,
        }
    }
}

impl From<SequenceElement> for TagNode {
    fn from(value: SequenceElement) -> Self {
        value.get_node().clone()
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
pub struct TagPath {
    pub nodes: Vec<TagNode>,
}

impl TagPath {
    /// Creates a tag path with no nodes.
    pub fn empty() -> TagPath {
        TagPath {
            nodes: Vec::with_capacity(0),
        }
    }

    /// Return whether there are any nodes in this tag path.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Formats the tag path as readable text, optionally using the tag's display name where
    /// possible, otherwise tags will be displayed as `(gggg,eeee)`.
    pub fn format_tagpath_to_display(
        tagpath: &TagPath,
        dict: Option<&dyn DicomDictionary>,
    ) -> String {
        tagpath
            .nodes
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

    /// Parses `TagNodes` from the given string and converts to a `TagPath`. The `TagNode`s must be
    /// separated by the period character `.`. The format of `TagNode` is described in
    /// `TagNode::from_str()`.
    ///
    /// Example:
    /// ```text
    /// "ReferencedFrameOfReferenceSequence
    ///   .RTReferencedStudySequence
    ///   .RTReferencedSeriesSequence
    ///   .ContourImageSequence[11]
    ///   .ReferencedSOPInstanceUID"
    /// ```
    /// will parse as:
    /// ```text
    /// [(0x3006_0010, Some(1)),
    ///  (0x3006_0012, Some(1)),
    ///  (0x3006_0014, Some(1)),
    ///  (0x3006_0016, Some(11)),
    ///  (0x0004_1504, None)]
    /// ```
    pub fn from_str(
        value: &str,
        dict: Option<&dyn DicomDictionary>,
    ) -> Result<TagPath, ParseError> {
        let tags = value.split('.').collect::<Vec<&str>>();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(tags.len());
        for (i, tag) in tags.iter().enumerate() {
            let mut node = TagNode::from_str(tag, dict)?;
            // Assume item #1 for all but the last TagNode if no item is supplied.
            if i < tags.len() - 1 {
                node.get_item_mut().get_or_insert(1);
            }
            nodes.push(node);
        }
        Ok(nodes.into())
    }
}

impl Display for TagPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &TagPath::format_tagpath_to_display(self, None))
    }
}

impl<T: Into<TagNode>> From<T> for TagPath {
    fn from(value: T) -> Self {
        TagPath {
            nodes: vec![value.into()],
        }
    }
}

impl<T: Into<TagNode>> From<Vec<T>> for TagPath {
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(len);
        for (i, t) in value.into_iter().enumerate() {
            let tag_node: TagNode = t.into();
            // For convenience, when making a tag path from a list of things that can be converted
            // into TagNode, assume that many of those things got the default into implementation
            // which defaults with an index of `None`, however unless it's the last node it should
            // not have an index of `None`. Assume `Some(1)` is the appropriate item index.
            let item = if i < len - 1 {
                tag_node.get_item().or(Some(1))
            } else {
                tag_node.get_item()
            };
            nodes.push(TagNode {
                tag: tag_node.get_tag(),
                item,
            })
        }
        TagPath { nodes }
    }
}

impl From<&[u32]> for TagPath {
    fn from(tags: &[u32]) -> Self {
        let len = tags.len();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(len);
        for (i, tag) in tags.iter().enumerate() {
            let item = if i == len - 1 { None } else { Some(1) };
            nodes.push(TagNode::new(*tag, item));
        }
        TagPath { nodes }
    }
}

impl From<&[Tag]> for TagPath {
    fn from(tags: &[Tag]) -> Self {
        let len = tags.len();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(len);
        for (i, tag) in tags.iter().enumerate() {
            let item = if i == len - 1 { None } else { Some(1) };
            nodes.push(TagNode::new(tag.tag, item));
        }
        TagPath { nodes }
    }
}

impl From<&[&Tag]> for TagPath {
    fn from(tags: &[&Tag]) -> Self {
        let len = tags.len();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(len);
        for (i, tag) in tags.iter().enumerate() {
            let item = if i == len - 1 { None } else { Some(1) };
            nodes.push(TagNode::new(tag.tag, item));
        }
        TagPath { nodes }
    }
}

impl From<&[&SequenceElement]> for TagPath {
    fn from(elements: &[&SequenceElement]) -> Self {
        elements
            .iter()
            .map(|sq_el| sq_el.get_node().clone())
            .collect::<Vec<TagNode>>()
            .into()
    }
}

impl From<&[SequenceElement]> for TagPath {
    fn from(elements: &[SequenceElement]) -> Self {
        elements
            .iter()
            .map(|sq_el| sq_el.into())
            .collect::<Vec<TagNode>>()
            .into()
    }
}

impl From<&Vec<SequenceElement>> for TagPath {
    fn from(elements: &Vec<SequenceElement>) -> Self {
        elements
            .iter()
            .map(|sq_el| sq_el.into())
            .collect::<Vec<TagNode>>()
            .into()
    }
}
