use std::process;

use clap::Parser;

use crate::app::archiveapp::ArchiveApp;
use crate::app::editapp::EditApp;
use crate::app::indexapp::IndexApp;
use crate::app::printapp::PrintApp;
use crate::app::scanapp::ScanApp;
use crate::app::CommandApplication;
use crate::args::{Arguments, Command};

mod app;
mod args;

fn main() {
    let mut app: Box<dyn CommandApplication> = make_app();
    if let Err(e) = app.run() {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}

fn make_app() -> Box<dyn CommandApplication> {
    let args: Arguments = Arguments::parse();

    match args.command {
        Command::Print { file } => Box::new(PrintApp::new(file)),
        Command::Edit { file } => Box::new(EditApp::new(file)),
        Command::Parse { folder } => Box::new(ScanApp::new(folder)),
        Command::Index { db, cmd } => Box::new(IndexApp::new(db, cmd)),
        Command::Archive {
            source,
            destination,
        } => Box::new(ArchiveApp::new(source, destination)),
    }
}
