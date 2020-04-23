use std::process;

use structopt::StructOpt;

use crate::app::archiveapp::ArchiveApp;
use crate::app::args::{Arguments, Command};
use crate::app::cursiveapp::CursiveApp;
use crate::app::fullobjapp::FullObjApp;
use crate::app::indexapp::IndexApp;
use crate::app::lowmemapp::LowMemApp;
use crate::app::scanapp::ScanApp;
use crate::app::CommandApplication;

mod app;

fn main() {
    let mut app: Box<dyn CommandApplication> = make_app();
    if let Err(e) = app.run() {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}

fn make_app() -> Box<dyn CommandApplication> {
    let args: Arguments = Arguments::from_args();

    match args.command {
        Command::Dump { stream, file } => {
            if stream {
                Box::new(LowMemApp::new(file))
            } else {
                Box::new(FullObjApp::new(file))
            }
        }
        Command::Edit { file } => Box::new(CursiveApp::new(file)),
        Command::Scan { folder } => Box::new(ScanApp::new(folder)),
        Command::Index { mongo, folder } => Box::new(IndexApp::new(mongo, folder)),
        Command::Archive {
            source,
            destination,
        } => Box::new(ArchiveApp::new(source, destination)),
    }
}
