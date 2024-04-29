use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
/// Explore DICOM
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Parses a single file and prints the DICOM elements to stdout.
    Print(PrintArgs),

    /// Browse a DICOM dataset in a text-based user interface.
    Browse(BrowseArgs),

    /// Recursively scans a folder of DICOM datasets and prints results of parsing.
    ///
    /// This is primarily useful for locating DICOM files which fail to parse.
    Scan(ScanArgs),

    /// Manage a database index of DICOM on disk.
    ///
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    Index(IndexArgs),

    /// Archives DICOM datasets from a source folder into a destination folder.
    ///
    /// The source folder is assumed to be unstructured whereas the DICOM datasets will be copied
    /// into the destination folder in a consistent structure:
    ///   - One series per folder
    ///   - Each DICOM file will be named in the format `[SOP_UID].dcm`
    Archive(ArchiveArgs),
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// The file to process as a DICOM dataset.
    pub file: PathBuf,
}

#[derive(Args, Debug)]
pub struct BrowseArgs {
    /// The file to process as a DICOM dataset.
    pub file: PathBuf,
}

#[derive(Args, Debug)]
pub struct ScanArgs {
    /// The folder to recursively scan for DICOM datasets.
    pub folder: PathBuf,
}

#[derive(Args, Debug)]
pub struct IndexArgs {
    #[arg(short, long)]
    /// The db URI of the index.
    pub db: String,

    #[clap(subcommand)]
    /// Index sub-command
    pub cmd: IndexCommand,
}

#[derive(Parser, Debug)]
pub enum IndexCommand {
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    Scan {
        /// The folder to scan for DICOM datasets.
        folder: PathBuf,
    },
    /// Verify records in the database reference valid files on-disk.
    Verify,
}

#[derive(Args, Debug)]
pub struct ArchiveArgs {
    /// The source folder of DICOM datasets to process.
    pub source: PathBuf,

    /// The destination folder to archive datasets into.
    pub destination: PathBuf,
}
