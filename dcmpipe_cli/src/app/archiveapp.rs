use anyhow::Result;

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
        // use fields to stop getting dead code warnings
        format!("{:?}, {:?}", self.args.source, self.args.destination);
        unimplemented!()
    }
}
