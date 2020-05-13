use anyhow::Result;
use std::path::PathBuf;

use crate::app::CommandApplication;

pub struct ArchiveApp {
    source: PathBuf,
    destination: PathBuf,
}

impl ArchiveApp {
    pub fn new(source: PathBuf, destination: PathBuf) -> ArchiveApp {
        ArchiveApp {
            source,
            destination,
        }
    }
}

impl CommandApplication for ArchiveApp {
    fn run(&mut self) -> Result<()> {
        // use fields to stop getting dead code warnings
        format!("{:?}, {:?}", self.source, self.destination);
        unimplemented!()
    }
}
