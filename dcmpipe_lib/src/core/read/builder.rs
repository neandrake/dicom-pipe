//! Configurable builder to create a DICOM parser.

use std::io::Read;

use super::{
    behavior::ParseBehavior,
    parser::{Parser, ParserState},
};

use crate::core::{
    charset::DEFAULT_CHARACTER_SET,
    defn::{constants::ts::ExplicitVRLittleEndian, dcmdict::DicomDictionary, ts::TSRef},
    read::{ds::dataset::Dataset, stop::ParseStop},
};

/// A builder for constructing a `Parser`.
#[derive(Debug, Default)]
pub struct ParserBuilder {
    /// Initial parse state. Default is `ParseState::DetectTransferSyntax`.
    state: Option<ParserState>,

    // Configure behavior during parsing.
    behavior: ParseBehavior,

    /// The transfer syntax of the dataset, if known. Defaults to `None` expecting that the initial
    /// parse state is `ParseState::DetectTransferSyntax`.
    dataset_ts: Option<TSRef>,
}

impl ParserBuilder {
    /// Sets the initial `ParserState` indicating how to start parsing the dataset.
    pub fn state(mut self, state: ParserState) -> Self {
        self.state = Some(state);
        self
    }

    /// Sets the `ParseStop` for when to stop parsing the dataset.
    pub fn stop(mut self, stop: ParseStop) -> Self {
        self.behavior.set_stop(stop);
        self
    }

    /// Specify whether or not to return a partial `DicomObject` if the parser encounters an error
    /// in the dataset.
    pub fn allow_partial_object(mut self, allow_partial_object: bool) -> Self {
        self.behavior.set_allow_partial_object(allow_partial_object);
        self
    }

    /// Sets the transfer syntax of the dataset, if known.
    pub fn dataset_ts(mut self, dataset_ts: TSRef) -> Self {
        self.dataset_ts = Some(dataset_ts);
        self
    }

    /// Constructs a `Parser` from this builder.
    ///
    /// `dictionary` - The DICOM dictionary to use during parsing. The parser uses `get_ts_by_uid`
    /// to identify transfer syntax for parsing through the stream, and `get_tag_by_number` for
    /// resolving VR of parsed elements. The VR is not strictly necessary for parsing elements
    /// however there is potential for sequences to not have their sub-elements parsed properly
    /// without this.
    pub fn build<'d, R: Read>(
        &self,
        dataset: R,
        dictionary: &'d dyn DicomDictionary,
    ) -> Parser<'d, R> {
        Parser {
            dataset: Dataset::new(dataset),
            behavior: self.behavior.clone(),
            dictionary,
            state: self.state.unwrap_or(ParserState::DetectTransferSyntax),

            bytes_read: 0,
            file_preamble: None,
            dicom_prefix: None,
            fmi_start: 0,
            fmi_grouplength: 0,
            tag_last_read: 0,
            vr_last_used: None,
            vl_last_used: None,
            ts_last_used: None,
            partial_tag: None,
            partial_vr: None,
            partial_vl: None,
            detected_ts: &ExplicitVRLittleEndian,
            dataset_ts: self.dataset_ts,
            cs: DEFAULT_CHARACTER_SET,
            current_path: Vec::new(),
            iterator_ended: false,
        }
    }
}
