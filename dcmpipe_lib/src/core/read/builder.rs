use std::io::Read;

use super::parser::ParseState;
use super::parser::Parser;
use crate::core::charset::DEFAULT_CHARACTER_SET;
use crate::core::read::ds::dataset::Dataset;
use crate::core::tagstop::TagStop;
use crate::defn::constants::lookup::MINIMAL_DICOM_DICTIONARY;
use crate::defn::constants::ts;
use crate::defn::dcmdict::DicomDictionary;

/// A builder for constructing `Parser` with common default states.
pub struct ParserBuilder<'dict> {
    /// Initial parse state. Default is `ParseState::DetectState`.
    state: Option<ParseState>,
    /// When to stop parsing the dataset. Default is `TagStop::EndOfDataset`.
    tagstop: Option<TagStop>,
    /// The `DicomDictionary` to be used when parsing elements. Default is `MinimalDicomDictionary`.
    dictionary: &'dict dyn DicomDictionary,
    /// The dataset will be wrapped in a `BufReader`, this lets the buffere size be set.
    buffsize: usize,
}

impl<'dict> ParserBuilder<'dict> {
    /// Sets the `TagStop` for when to stop parsing the dataset.
    pub fn tagstop(mut self, tagstop: TagStop) -> Self {
        self.tagstop = Some(tagstop);
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
            tagstop: self.tagstop.clone().unwrap_or(TagStop::EndOfDataset),
            dictionary: self.dictionary,
            state: self
                .state
                .clone()
                .unwrap_or(ParseState::DetectTransferSyntax),

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
            dataset_ts: None,
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
            tagstop: None,
            dictionary: &MINIMAL_DICOM_DICTIONARY,
            // BufReader's current default buffer size is 8k
            buffsize: 8 * 1024,
        }
    }
}
