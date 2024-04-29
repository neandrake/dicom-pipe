use std::process;

use clap::Parser;

#[cfg(feature = "index")]
use crate::app::indexapp::IndexApp;
use crate::{
    app::{
        archiveapp::ArchiveApp, browseapp::BrowseApp, printapp::PrintApp, scanapp::ScanApp,
        scpapp::SvcProviderApp, CommandApplication,
    },
    args::{Arguments, Command},
};

mod app;
mod args;
mod threadpool;

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
        Command::Print(args) => Box::new(PrintApp::new(args)),
        Command::Browse(args) => Box::new(BrowseApp::new(args)),
        Command::Scan(args) => Box::new(ScanApp::new(args)),
        #[cfg(feature = "index")]
        Command::Index(args) => Box::new(IndexApp::new(args)),
        Command::Archive(args) => Box::new(ArchiveApp::new(args)),
        Command::Scp(args) => Box::new(SvcProviderApp::new(args)),
    }
}
