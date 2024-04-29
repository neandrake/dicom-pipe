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
