extern crate crossterm;
extern crate cursive;
extern crate cursive_table_view;
extern crate dcmpipe_dict;
extern crate dcmpipe_lib;
extern crate tui;

mod app;

use crate::app::CursiveApp;
use app::{FullObjApp, LowMemApp, TuiApp};
use std::io::{Error, ErrorKind};
use std::{env, process};

static APP_MODE: usize = 4;

fn main() {
    if let Err(e) = runapp() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn runapp() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "first and only argument should be a file",
        ));
    }

    let openpath: String = args[1].to_owned();

    match APP_MODE {
        1 => LowMemApp::new(openpath).run(),
        2 => FullObjApp::new(openpath).run(),
        3 => TuiApp::new(openpath).run(),
        4 => CursiveApp::new(openpath).run(),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid app mode")),
    }
}
