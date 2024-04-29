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

use std::str::FromStr;

use crate::{
    core::read::ParseError,
    dimse::{
        commands::messages::CommandMessage,
        pdus::mainpdus::{Abort, AssocRJ, PresentationDataValue},
    },
};

pub mod scp;
pub mod scu;

#[derive(Debug)]
pub enum DimseMsg {
    Cmd(CommandMessage),
    Dataset(PresentationDataValue),
    ReleaseRQ,
    ReleaseRP,
    Reject(AssocRJ),
    Abort(Abort),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QueryLevel {
    Patient,
    Study,
    Series,
    Image,
}

impl QueryLevel {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match &self {
            Self::Patient => "PATIENT",
            Self::Study => "STUDY",
            Self::Series => "SERIES",
            Self::Image => "IMAGE",
        }
    }

    #[must_use]
    pub fn include_patient_tags(&self) -> bool {
        true
    }

    #[must_use]
    pub fn include_study_tags(&self) -> bool {
        self != &QueryLevel::Patient
    }
}

impl std::fmt::Display for QueryLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for QueryLevel {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "PATIENT" => Ok(Self::Patient),
            "STUDY" => Ok(Self::Study),
            "SERIES" => Ok(Self::Series),
            "IMAGE" => Ok(Self::Image),
            other => Err(ParseError::GeneralDecodeError(format!(
                "Invalid Query Level: {other}"
            ))),
        }
    }
}
