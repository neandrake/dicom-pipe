/*
   Copyright 2024 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the \"License\");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an \"AS IS\" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use anyhow::{anyhow, Result};

use crate::{app::CommandApplication, args::ArchiveArgs};

pub struct ArchiveApp {
    args: ArchiveArgs,
}

impl ArchiveApp {
    pub fn new(args: ArchiveArgs) -> ArchiveApp {
        ArchiveApp { args }
    }
}

impl CommandApplication for ArchiveApp {
    fn run(&mut self) -> Result<()> {
        Err(anyhow!(
            "Archive is not implemented for source: {:?}, destination: {:?}",
            self.args.source,
            self.args.destination
        ))
    }
}
