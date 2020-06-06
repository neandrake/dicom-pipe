use std::io::Read;

use super::error::ParseError;
use super::parser::Parser;
use super::parser::Result;
use crate::core::dcmelement::DicomElement;

impl<'dict, DatasetType: Read> Iterator for Parser<'dict, DatasetType> {
    type Item = Result<DicomElement>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
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
