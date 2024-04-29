use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Explore DICOM
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Parses a single file and prints the DICOM elements to stdout.
    Dump {
        #[structopt(short, long)]
        /// Process the dataset as a stream.
        ///
        /// If specified, instead of reading the entire dataset into memory will print elements as
        /// they are parsed.
        stream: bool,

        /// The file to process as a DICOM dataset.
        file: PathBuf,
    },
    /// Opens a DICOM dataset in a TUI for browsing and editing.
    Edit {
        /// The file to process as a DICOM dataset.
        file: PathBuf,
    },
    /// Scans a folder recursively for DICOM datasets and prints results of found DICOM.
    Scan {
        /// The folder to recursively scan for DICOM datasets.
        folder: PathBuf,
    },
}
