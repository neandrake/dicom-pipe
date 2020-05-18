use crate::defn::tag::{TagPath, TagNode};
use crate::core::dcmsqelem::SequenceElement;
use std::iter::once;

/// TagStop specifies the stopping point at which parsing of a DICOM dataset should end.
#[derive(Clone)]
pub enum TagStop {
    /// The entire dataset should be parsed.
    EndOfDataset,
    /// Read all tag elements up to (but not including) the specified tag. The tag value is
    /// interpreted only at the root of an object and not within sequences.
    BeforeTag(TagPath),
    /// Read all tag elements up to (and including) the specified tag. The tag value is interpreted
    /// only at the root of an object and not within sequences.
    AfterTag(TagPath),
    /// Read all tag elements up to specified number of bytes have been read. If the byte position
    /// is in the middle of an element then bytes from that dataset will continue to be read until
    /// the elment is fully parsed.
    AfterBytePos(u64),
}

impl TagStop {
    pub fn check_tagpath<F>(
        tagpath: &TagPath,
        current_path: &Vec<SequenceElement>,
        tag_last_read: u32,
        f: F,
    ) -> bool
        where F: FnMut(&(u32, u32)) -> bool {
        tagpath.0
            .iter()
            .map(TagNode::get_tag)
            .zip(current_path
                .iter()
                .map(SequenceElement::get_seq_tag)
                .chain(once(tag_last_read))
            )
            .find(f)
            .is_some()
    }
}