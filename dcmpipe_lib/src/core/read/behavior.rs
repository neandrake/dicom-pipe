//! Specify behavior while parsing.

use super::stop::ParseStop;

#[derive(Clone, Debug)]
pub struct ParseBehavior {
    /// Specifies when the parser should stop parsing the dataset.
    ///
    /// Refer to documentatoin on `ParseStop`.
    ///
    /// Default: `ParseStop::EndOfDataset`.
    pub stop: ParseStop,

    /// Specifies how the parser should handle an error it encounters in the dataset while
    /// populating a `DicomObject` with elements it parses.
    ///
    /// If `true`, a partially-populated `DicomObject` will be returned instead of a `Result::Err`.
    ///
    /// Default: `false`.
    pub allow_partial_object: bool,
}

impl Default for ParseBehavior {
    fn default() -> Self {
        Self {
            stop: ParseStop::EndOfDataset,
            allow_partial_object: false,
        }
    }
}
