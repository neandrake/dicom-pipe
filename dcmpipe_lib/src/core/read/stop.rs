use crate::defn::tag::{TagNode, TagPath};

/// ParseStop specifies the stopping point at which parsing of a DICOM dataset should end.
#[derive(Clone)]
pub enum ParseStop {
    /// The entire dataset should be parsed, until the dataset stream/end is encountered.
    EndOfDataset,

    /// Read all elements until encountering the given tag, to avoid parsing the given tag's value
    /// field. This can be used to read everything up to e.g. `PixelData` but avoids parsing the
    /// `PixelData`s contents.
    ///
    /// If the tag does not exist in the dataset then this stops
    BeforeTagValue(TagPath),

    /// Read all elements until the given tag and its value contents have been parsed.
    AfterTagValue(TagPath),

    /// Read all tag elements up to specified number of bytes have been read. If the byte position
    /// is in the middle of an element then bytes from that dataset will continue to be read until
    /// the elment is fully parsed.
    AfterBytePos(u64),
}

impl ParseStop {
    /// Evaluates the given `TagPath` against this `ParseStop`'s defined stopping point, assuming
    /// this is `ParseStop::BeforeTagValue` or `ParseStop::AfterTagValue`. If this is neither
    /// `BeforeTagValue` nor `AfterTagValue` then this returns false.
    pub fn evaluate(&self, current: &TagPath) -> bool {
        match self {
            ParseStop::BeforeTagValue(target) => target
                .nodes
                .iter()
                .zip(current.nodes.iter())
                .any(ParseStop::is_before_tag_value),
            ParseStop::AfterTagValue(target) => target
                .nodes
                .iter()
                .zip(current.nodes.iter())
                .any(ParseStop::is_after_tag_value),
            _ => false,
        }
    }

    fn is_before_tag_value((target, current): (&TagNode, &TagNode)) -> bool {
        if current.get_tag() < target.get_tag() {
            // The target tag has not yet been encountered, do not stop parsing.
            false
        } else if current.get_tag() == target.get_tag() {
            // The target tag is encountered, compare target index.
            if let Some(cur_idx) = current.get_item() {
                match target.get_item() {
                    // If at or past (shouldn't occur) the target index then stop parsing.
                    Some(target_idx) => cur_idx >= target_idx,

                    // Stop parsing if no index was specified for the target.
                    None => true,
                }
            } else {
                // The target tag matches but there's no index to compare.
                true
            }
        } else {
            // The current tag has surpassed the target, the target is not present in the dataset.
            true
        }
    }

    fn is_after_tag_value((target, current): (&TagNode, &TagNode)) -> bool {
        if current.get_tag() <= target.get_tag() {
            // The target tag has not yet been encountered, do not stop parsing.
            false
        } else if current.get_tag() == target.get_tag() {
            // The target tag is encountered, compare target index.
            if let Some(cur_idx) = current.get_item() {
                match target.get_item() {
                    // If past the target index then stop parsing.
                    Some(target_idx) => cur_idx > target_idx,

                    // Do not stop parsing if no index was specified for target, assuming the
                    // entire set of items should then be parsed.
                    None => false,
                }
            } else {
                // The target tag matches but there's no index for comparison. Do not stop parsing
                // so all contents/items are parsed.
                false
            }
        } else {
            // The current tag has surpassed the target, the target and its contents have been
            // parsed (or target was not in dataset).
            true
        }
    }
}
