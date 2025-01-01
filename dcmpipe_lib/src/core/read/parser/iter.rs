/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::io::Read;

use crate::core::{
    dcmelement::DicomElement,
    read::{
        error::ParseError,
        parser::{ParseResult, Parser},
    },
};

/// The implementation for `Parser` which is the core iteration loop.
impl<R: Read> Iterator for Parser<'_, R> {
    type Item = ParseResult<DicomElement>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // Once an error occurs, or the first `None` is returned then do not
        // continue trying to parse, and always return `None`.
        if self.iterator_ended {
            return None;
        }

        match self.iterate() {
            Ok(None) | Err(ParseError::ExpectedEOF) => {
                self.iterator_ended = true;
                None
            }
            // This function should be the only place that creates DetailedErrors, but as
            // precaution check and propagate without wrapping.
            Err(ParseError::DetailedError { source, detail }) => {
                self.iterator_ended = true;
                Some(Err(ParseError::DetailedError { source, detail }))
            }
            Err(e) => {
                self.iterator_ended = true;
                let detail = self.current_debug_str();
                Some(Err(ParseError::DetailedError {
                    source: Box::new(e),
                    detail,
                }))
            }
            Ok(Some(element)) => Some(Ok(element)),
        }
    }
}
