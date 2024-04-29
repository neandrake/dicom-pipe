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

#![allow(clippy::module_name_repetitions)]

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
        eprintln!("Error: {e:?}");
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
