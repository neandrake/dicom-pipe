use std::fmt::{Debug, Display, Formatter};

use crate::core::{
    charset::CSRef,
    defn::{tag::TagNode, vl::ValueLength, vr::VRRef},
};

/// Represents the sequence/item position of an element.
/// For elements to track which sequence they are a part of. When an SQ element is parsed the parser
/// adds a new `SequenceElement` to its current path which subsequent elements will clone for
/// themselves. This allows elements to know how they exist within a dicom object.
#[derive(Clone)]
pub struct SequenceElement {
    /// See Part 5 Section 7.5
    /// Items present in an SQ Data Element shall be an ordered set where each Item may be
    /// referenced by its ordinal position. Each Item shall be implicitly assigned an ordinal
    /// position starting with the value 1 for the first Item in the Sequence, and incremented by 1
    /// with each subsequent Item. The last Item in the Sequence shall have an ordinal position
    /// equal to the number of Items in the Sequence.
    ///
    /// This is initialized/incremented whenever an Item tag is parsed. Sequences are not required
    /// to have their contents encoded within items so this cannot be used as an index into a
    /// sequence's total listing of top-level children.
    node: TagNode,

    /// The byte position where the parent sequence ends. This value is set as
    /// `bytes_read + value_length` during parsing. If the sequence has undefined length this is set
    /// to None.
    seq_end_pos: Option<u64>,

    /// The VR of the sequence/item. This will typically be SQ for sequence element and UN for Item
    /// elements. For private sequence elements this should be UN.
    vr: VRRef,

    /// The Value Length of the sequence/item.
    vl: ValueLength,

    /// See Part 5 Section 7.5.3
    /// If an encapsulated Data Set includes the Specific Character Set Attribute, it shall apply
    /// only to the encapsulated Data Set. If the Attribute Specific Character Set is not explicitly
    /// included in an encapsulated Data Set, then the Specific Character Set value of the
    /// encapsulating Data Set applies.
    cs: CSRef,
}

impl SequenceElement {
    pub fn new(
        seq_tag: u32,
        seq_end_pos: Option<u64>,
        vr: VRRef,
        vl: ValueLength,
        cs: CSRef,
    ) -> SequenceElement {
        SequenceElement {
            node: TagNode::new(seq_tag, None),
            seq_end_pos,
            vr,
            vl,
            cs,
        }
    }

    pub fn node(&self) -> &TagNode {
        &self.node
    }

    pub fn seq_tag(&self) -> u32 {
        self.node.tag()
    }

    pub fn item(&self) -> Option<usize> {
        self.node.item()
    }

    pub fn seq_end_pos(&self) -> Option<u64> {
        self.seq_end_pos
    }

    pub fn vr(&self) -> VRRef {
        self.vr
    }

    pub fn vl(&self) -> ValueLength {
        self.vl
    }

    pub fn cs(&self) -> CSRef {
        self.cs
    }

    pub fn set_cs(&mut self, cs: CSRef) {
        self.cs = cs;
    }

    pub fn increment_item(&mut self) {
        match self.node.item() {
            None => {
                // Item numbers start at 1.
                self.node.item_mut().replace(1);
            }
            Some(val) => {
                self.node.item_mut().replace(val + 1);
            }
        }
    }

    pub fn decrement_item(&mut self) {
        match self.node.item() {
            None => {}
            Some(val) if val > 1 => {
                self.node.item_mut().replace(val - 1);
            }
            Some(_) => {
                self.node.item_mut().take();
            }
        }
    }
}

impl Display for SequenceElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

impl Debug for SequenceElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "node: {:?}, end: {:?}", self.node, self.seq_end_pos)
    }
}
