use std::io::Read;

use super::error::ParseError;
use super::parser::Parser;
use super::parser::Result;
use crate::core::dcmelement::DicomElement;

/// The implementation for `Parser` which is the core iteration loop.
impl<'dict, DatasetType: Read> Iterator for Parser<'dict, DatasetType> {
    type Item = Result<DicomElement>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // Once an error occurs, or the first `None` is returned then do not
        // continue trying to parse, and always return `None`.
        if self.iterator_ended {
            return None;
        }

        match self.iterate() {
            Err(ParseError::ExpectedEOF) => {
                self.iterator_ended = true;
                None
            }
            Err(e) => {
                self.iterator_ended = true;
                Some(Err(e))
            }
            Ok(None) => {
                self.iterator_ended = true;
                None
            }
            Ok(Some(element)) => Some(Ok(element)),
        }
    }
}
