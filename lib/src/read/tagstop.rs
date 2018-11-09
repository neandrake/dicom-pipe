

/// TagStop specifies the stopping point at which parsing of a DICOM stream
/// should end.
pub enum TagStop {
    /// The entire stream should be parsed.
    EndOfStream,
    /// Read all tag elements up to (but not including) the specified tag.
    BeforeTag(u32),
    /// Read all tag elements up to (and including) the specified tag.
    AfterTag(u32),
    /// Read all tag elements up to specified number of bytes have been read.
    /// If the byte position is in the middle of an element then bytes from
    /// that stream will continue to be read until the elment is fully parsed.
    AfterBytePos(u64),
}
