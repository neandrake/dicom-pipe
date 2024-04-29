/*
   Copyright 2024 Christopher Speck

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

use anyhow::{anyhow, Result};

use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use dcmpipe_lib::{
    core::read::{Parser, ParserBuilder},
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    dimse::{
        assoc::DimseMsg,
        error::{AssocError, AssocRsp},
        pdus::PduType,
    },
};

pub(crate) mod archiveapp;
pub(crate) mod browseapp;
#[cfg(feature = "index")]
pub(crate) mod indexapp;
pub(crate) mod printapp;
pub(crate) mod scpapp;
pub(crate) mod scuapp;

pub(crate) trait CommandApplication {
    fn run(&mut self) -> Result<()>;
}

fn parse_file(path: &Path, allow_partial_object: bool) -> Result<Parser<'_, BufReader<File>>> {
    if !path.is_file() {
        return Err(anyhow!("invalid file: {}", path.display()));
    }

    let file = File::open(path)?;
    let dataset = BufReader::new(file);
    let mut parser = ParserBuilder::default()
        .allow_partial_object(allow_partial_object)
        .build(dataset, &STANDARD_DICOM_DICTIONARY);

    let mut peeker = parser.by_ref().peekable();

    let first = peeker.peek();
    if let Some(Err(_)) = first {
        return Err(anyhow!("file is not dicom: {}", path.display()));
    } else if first.is_none() {
        return Err(anyhow!("file is empty: {}", path.display()));
    }

    Ok(parser)
}

/// Returns a log statement appropriate for the result of an association.
fn handle_assoc_result<W: Write>(
    result: Result<DimseMsg, AssocError>,
    writer: W,
) -> Vec<Result<String, String>> {
    match result {
        Ok(DimseMsg::ReleaseRQ) => vec![Ok(format!("[info <-]: {:?}", PduType::ReleaseRQ))],
        Ok(DimseMsg::ReleaseRP) => vec![Ok(format!("[info <-]: {:?}", PduType::ReleaseRP))],
        Ok(DimseMsg::Reject(rj)) => vec![Ok(format!("[warn <-]: {}", rj.get_reason_desc()))],
        Ok(DimseMsg::Abort(ab)) => vec![Ok(format!("[warn <-]: {}", ab.get_reason_desc()))],
        Ok(other) => vec![Err(format!(
            "[ err xx]: Unexpected ending state: {other:?}"
        ))],
        Err(e) => {
            let mut output: Vec<Result<String, String>> = Vec::with_capacity(3);
            output.push(Err(format!("[ err ><]: {e}")));
            match e.rsp() {
                Some(AssocRsp::RJ(rj)) => {
                    output.push(Ok(format!("[info ->]: {:?}", rj.pdu_type())));
                }
                Some(AssocRsp::AB(ab)) => {
                    output.push(Ok(format!("[info ->]: {:?}", ab.pdu_type())));
                }
                None => {}
            }
            if let Err(inner) = e.write(writer) {
                output.push(Err(format!(
                    "[ err xx]: Failure writing error response: {inner}"
                )));
            }
            output
        }
    }
}
