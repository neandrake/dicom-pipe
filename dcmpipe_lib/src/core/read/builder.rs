//! Configurable builder to create a DICOM parser.

use std::io::Read;

use super::{
    behavior::ParseBehavior,
    parser::{Parser, ParserState},
};

use crate::core::{
    charset::DEFAULT_CHARACTER_SET,
    defn::{
        constants::{lookup::MINIMAL_DICOM_DICTIONARY, ts},
        dcmdict::DicomDictionary,
        ts::TSRef,
    },
    read::{ds::dataset::Dataset, stop::ParseStop},
};

/// A builder for constructing a `Parser`.
#[derive(Debug)]
pub struct ParserBuilder<'dict> {
    /// Initial parse state. Default is `ParseState::DetectTransferSyntax`.
    state: Option<ParserState>,

    // Configure behavior during parsing.
    behavior: ParseBehavior,

    /// The transfer syntax of the dataset, if known. Defaults to `None` expecting that the initial
    /// parse state is `ParseState::DetectTransferSyntax`.
    dataset_ts: Option<TSRef>,

    /// The `DicomDictionary` to be used when parsing elements. Default is `MinimalDicomDictionary`.
    dictionary: &'dict dyn DicomDictionary,

    /// The dataset will be wrapped in a `BufReader`, this lets the buffer size be set.
    buffsize: usize,
}

impl<'dict> ParserBuilder<'dict> {
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

    /// Sets the DICOM dictionary. The parser uses `get_ts_by_uid` to identify transfer syntax for
    /// parsing through the stream, and `get_tag_by_number` for resolving VR of parsed elements. The
    /// VR is not strictly necessary for parsing elements however there is potential for sequences
    /// to not have their sub-elements parsed properly without this.
    pub fn dictionary(mut self, dictionary: &'dict dyn DicomDictionary) -> Self {
        self.dictionary = dictionary;
        self
    }

    /// Set the buffer size to use when parsing the dataset.
    pub fn buffsize(mut self, buffsize: usize) -> Self {
        self.buffsize = buffsize;
        self
    }

    /// Constructs a `Parser` from this builder.
    pub fn build<DatasetType: Read>(&self, dataset: DatasetType) -> Parser<'dict, DatasetType> {
        Parser {
            dataset: Dataset::new(dataset, self.buffsize),
            behavior: self.behavior.clone(),
            dictionary: self.dictionary,
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
            detected_ts: &ts::ExplicitVRLittleEndian,
            dataset_ts: self.dataset_ts,
            cs: DEFAULT_CHARACTER_SET,
            current_path: Vec::new(),
            iterator_ended: false,
        }
    }
}

impl<'dict> Default for ParserBuilder<'dict> {
    fn default() -> Self {
        ParserBuilder {
            state: None,
            behavior: ParseBehavior::default(),
            dataset_ts: None,
            dictionary: &MINIMAL_DICOM_DICTIONARY,
            // BufReader's current default buffer size is 8k.
            buffsize: 8 * 1024,
        }
    }
}
