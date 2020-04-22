use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// DICOM Archiver
pub struct Arguments {
    #[structopt(subcommand)]
    /// The DICOM Archiver command to run
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Scan a directory
    ///
    /// This will scan a folder for DICOM datasets, registering them in the database.
    Scan {
        #[structopt(short, long)]
        /// The mongo URI to insert dicom records into
        mongo: String,

        /// The folder to scan for DICOM datasets.
        file: PathBuf,
    },
    /// Imports DICOM datasets in the given folder, copying them into the archive location.
    Import {
        /// The file to process as a DICOM dataset.
        file: PathBuf,
    },
}
