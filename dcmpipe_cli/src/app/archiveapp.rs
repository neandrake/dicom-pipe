use std::io::Error;
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
    fn run(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}
