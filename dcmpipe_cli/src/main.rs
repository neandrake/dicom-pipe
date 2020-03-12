mod app;

use app::cursiveapp::CursiveApp;
use app::fullobjapp::FullObjApp;
use app::lowmemapp::LowMemApp;
use std::io::{Error, ErrorKind};
use std::{env, process};

static APP_MODE: usize = 3;

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
        3 => CursiveApp::new(openpath).run(),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid app mode")),
    }
}
