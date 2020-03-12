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
    /// Dump a DICOM dataset
    ///
    /// This will display all elements of a dataset, printing to stdout
    Dump {
        #[structopt(short, long)]
        /// Process the dataset as a stream
        ///
        /// If not specified then the entire dataset will be loaded into memory prior to display
        stream: bool,

        /// The file to process as a DICOM dataset
        file: PathBuf,
    },
    /// Opens a DICOM dataset in a TUI for browsing and editing
    Edit {
        /// The file to process as a DICOM dataset
        file: PathBuf,
    },
}