/// TagStop specifies the stopping point at which parsing of a DICOM dataset should end.
#[derive(Clone)]
pub enum TagStop {
    /// The entire dataset should be parsed.
    EndOfDataset,
    /// Read all tag elements up to (but not including) the specified tag. The tag value is
    /// interpreted only at the root of an object and not within sequences.
    BeforeTag(u32),
    /// Read all tag elements up to (and including) the specified tag. The tag value is interpreted
    /// only at the root of an object and not within sequences.
    AfterTag(u32),
    /// Read all tag elements up to specified number of bytes have been read. If the byte position
    /// is in the middle of an element then bytes from that dataset will continue to be read until
    /// the elment is fully parsed.
    AfterBytePos(u64),
}
