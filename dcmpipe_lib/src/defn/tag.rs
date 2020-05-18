//! DICOM Data Elements

use crate::defn::vm::VMRef;
use crate::defn::vr::VRRef;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter};

pub type TagRef = &'static Tag;

/// DICOM Data Element (Tag)
#[derive(Debug, Eq)]
pub struct Tag {
    /// Some identifier or name, useful for lookup (no spaces - matches definition/name in code)
    pub ident: &'static str,
    /// The tag value
    pub tag: u32,
    /// The default VR which should be used to read this tag -- used for ImplicitVR
    /// This should maybe be a Vec<&VR> instead as a handful of tags can use several
    pub implicit_vr: Option<VRRef>,
    /// The value multiplicity -- how many values this tag can have
    pub vm: VMRef,
    /// Long name or description
    pub desc: &'static str,
}

impl Tag {
    pub fn get_ident(&self) -> &'static str {
        self.ident
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_implicit_vr(&self) -> Option<VRRef> {
        self.implicit_vr
    }

    pub fn get_vm(&self) -> VMRef {
        self.vm
    }

    /// Renders the tag number as `(gggg,eeee)`
    pub fn format_tag_to_display(tag: u32) -> String {
        let tag_group: u32 = tag >> 16;
        let tag_element: u32 = tag & 0x0000_FFFF;
        format!("({:04X},{:04X})", tag_group, tag_element)
    }

    /// Renders the tag number as `ggggeeee`
    pub fn format_tag_to_path_display(tag: u32) -> String {
        let tag_group: u32 = tag >> 16;
        let tag_element: u32 = tag & 0x0000_FFFF;
        format!("{:04X}{:04X}", tag_group, tag_element)
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.tag.eq(&other.tag)
    }
}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TagNode {
    tag: u32,
    item: Option<usize>,
}

impl TagNode {
    pub fn new(tag: u32, item: Option<usize>) -> TagNode {
        TagNode { tag, item }
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_item(&self) -> Option<usize> {
        self.item
    }

    pub fn get_item_mut(&mut self) -> &mut Option<usize> {
        &mut self.item
    }
}

impl Debug for TagNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.item {
            None => write!(f, "{}", Tag::format_tag_to_display(self.tag)),
            Some(item) => write!(f, "{}[{}]", Tag::format_tag_to_display(self.tag), item)
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

impl From<Vec<TagNode>> for TagPath {
    fn from(nodes: Vec<TagNode>) -> Self {
        TagPath(nodes)
    }
}

impl From<Vec<u32>> for TagPath {
    fn from(tags: Vec<u32>) -> Self {
        tags.into_iter()
            .map(|t| t.into())
            .collect::<Vec<TagNode>>()
            .into()
    }
}

impl From<&[u32]> for TagPath {
    fn from(tags: &[u32]) -> Self {
        tags.iter()
            .map(|t| t.into())
            .collect::<Vec<TagNode>>()
            .into()
    }
}

impl From<u32> for TagPath {
    fn from(tag: u32) -> Self {
        vec![TagNode::from(tag)].into()
    }
}
