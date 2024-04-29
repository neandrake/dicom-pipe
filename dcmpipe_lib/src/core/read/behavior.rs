//! Specify behavior while parsing.

use super::stop::ParseStop;

#[derive(Clone, Debug)]
pub struct ParseBehavior {
    /// Specifies when the parser should stop parsing the dataset.
    ///
    /// Refer to documentation on `ParseStop`.
    ///
    /// Default: `ParseStop::EndOfDataset`.
    stop: ParseStop,

    /// Specifies how the parser should handle an error it encounters in the dataset while
    /// populating a `DicomObject` with elements it parses.
    ///
    /// If `true`, a partially-populated `DicomObject` will be returned instead of a `Result::Err`.
    ///
    /// Default: `false`.
    allow_partial_object: bool,
}

impl ParseBehavior {
    /// Specifies when the parser should stop parsing the dataset.
    ///
    /// Refer to documentation on `ParseStop`.
    ///
    /// Default: `ParseStop::EndOfDataset`.
    #[must_use]
    pub fn stop(&self) -> &ParseStop {
        &self.stop
    }

    /// Specifies how the parser should handle an error it encounters in the dataset while
    /// populating a `DicomObject` with elements it parses.
    ///
    /// If `true`, a partially-populated `DicomObject` will be returned instead of a `Result::Err`.
    ///
    /// Default: `false`.
    #[must_use]
    pub fn allow_partial_object(&self) -> bool {
        self.allow_partial_object
    }

    /// Specify when the parser should stop parsing the dataset.
    ///
    /// Refer to documentation on `ParseStop`.
    pub fn set_stop(&mut self, stop: ParseStop) {
        self.stop = stop;
    }

    /// Specify how the parser should handle an error in encounters in the dataset while populating
    /// a `DicomObject` with elements it parses.
    ///
    /// If `true`, a partially-populated `DicomObject` will be returned instead of a `Result::Err`.
    pub fn set_allow_partial_object(&mut self, allow_partial_object: bool) {
        self.allow_partial_object = allow_partial_object;
    }
}

impl Default for ParseBehavior {
    fn default() -> Self {
        Self {
            stop: ParseStop::EndOfDataset,
            allow_partial_object: false,
        }
    }
}
