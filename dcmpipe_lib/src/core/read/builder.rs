//! Configurable builder to create a DICOM parser.

use std::io::Read;

use super::parser::ParseState;
use super::parser::Parser;
use crate::core::charset::DEFAULT_CHARACTER_SET;
use crate::core::read::ds::dataset::Dataset;
use crate::core::read::stop::ParseStop;
use crate::defn::constants::lookup::MINIMAL_DICOM_DICTIONARY;
use crate::defn::constants::ts;
use crate::defn::dcmdict::DicomDictionary;
use crate::defn::ts::TSRef;

/// A builder for constructing `Parser` with common default states.
pub struct ParserBuilder<'dict> {
    /// Initial parse state. Default is `ParseState::DetectTransferSyntax`.
    state: Option<ParseState>,

    /// When to stop parsing the dataset. Default is `ParseStop::EndOfDataset`.
    stop: Option<ParseStop>,

    /// The transfer syntax of the dataset, if known. Defaults to `None` expecting that the initial
    /// parse state is `ParseState::DetectTransferSyntax`.
    dataset_ts: Option<TSRef>,

    /// The `DicomDictionary` to be used when parsing elements. Default is `MinimalDicomDictionary`.
    dictionary: &'dict dyn DicomDictionary,

    /// The dataset will be wrapped in a `BufReader`, this lets the buffere size be set.
    buffsize: usize,
}

impl<'dict> ParserBuilder<'dict> {
    /// Sets the initial `ParseState` indicating how to start parsing the dataset.
    pub fn state(mut self, state: ParseState) -> Self {
        self.state = Some(state);
        self
    }

    /// Sets the `ParseStop` for when to stop parsing the dataset.
    pub fn stop(mut self, stop: ParseStop) -> Self {
        self.stop = Some(stop);
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

    /// Constructs the parser from this builder.
    pub fn build<DatasetType: Read>(&self, dataset: DatasetType) -> Parser<'dict, DatasetType> {
        Parser {
            dataset: Dataset::new(dataset, self.buffsize),
            stop: self.stop.clone().unwrap_or(ParseStop::EndOfDataset),
            dictionary: self.dictionary,
            state: self.state.unwrap_or(ParseState::DetectTransferSyntax),

            bytes_read: 0,
            file_preamble: None,
            dicom_prefix: None,
            fmi_start: 0,
            fmi_grouplength: 0,
            tag_last_read: 0,
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
            stop: None,
            dataset_ts: None,
            dictionary: &MINIMAL_DICOM_DICTIONARY,
            // BufReader's current default buffer size is 8k
            buffsize: 8 * 1024,
        }
    }
}
