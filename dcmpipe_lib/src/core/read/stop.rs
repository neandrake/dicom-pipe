use std::iter::once;

use crate::core::dcmsqelem::SequenceElement;
use crate::defn::tag::{TagNode, TagPath};

/// ParseStop specifies the stopping point at which parsing of a DICOM dataset should end.
#[derive(Clone)]
pub enum ParseStop {
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

impl ParseStop {
    /// Evaluates the given tagpath against the current sequence path and the last tag read, based
    /// on the given predicate `f`.
    ///
    /// `tagpath` - The tag path to evaluate against the current path
    /// `current_path` - The current sequence path to evaluate against `tagpath`
    /// `tag_last_read` - The last tag read by parser. This will be used along with `current_path`
    ///                   to build a comparative list of tags for comparing against `tagpath`. This
    ///                   tag should be for an element which is a child of the last element of
    ///                   `current_path`.
    /// `f` - The predicate for comparing the specified tagpath value against the corresponding
    ///       current tag value. This will be passed a tuple, `(to_check, current)` where `to_check`
    ///       is the tag from `tagpath` and `current` is the tag from `current_path` or
    ///       `tag_last_read`. If this predicate returns true the return value of this function will
    ///       be true.
    pub(crate) fn eval_tagpath<F>(
        tagpath: &TagPath,
        current_path: &[SequenceElement],
        tag_last_read: u32,
        f: F,
    ) -> bool
    where
        F: FnMut((u32, u32)) -> bool,
    {
        let full_current_path = current_path
            .iter()
            .map(SequenceElement::get_seq_tag)
            .chain(once(tag_last_read));

        // TODO: The TagNodes need to also be evaluated for their item number, so if a tag path to
        //       stop at indicates not to exceed a certain item number then that can be enforced.
        //       As it's currently implemented the entire sequence will be parsed regardless of
        //       desired item number stopping point.

        tagpath.0
            .iter()
            .map(TagNode::get_tag)
            .zip(full_current_path)
            .any(f)
    }
}
