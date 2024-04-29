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

    /// Manage a database index of DICOM on disk.
    ///
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    #[cfg(feature = "index")]
    Index(IndexArgs),

    /// Archives DICOM datasets from a source folder into a destination folder.
    ///
    /// The source folder is assumed to be unstructured whereas the DICOM datasets will be copied
    /// into the destination folder in a consistent structure:
    ///   - One series per folder
    ///   - Each DICOM file will be named in the format `[SOP_UID].dcm`
    Archive(ArchiveArgs),

    /// Starts an SCP service.
    Scp(SvcProviderArgs),
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

#[derive(Args, Debug)]
pub struct SvcProviderArgs {
    #[arg(short, long)]
    /// The host/port to bind the service on.
    pub host: String,

    #[arg(short, long)]
    /// The AE Title to run as.
    pub aetitle: String,

    #[arg(short, long)]
    /// An allow-list of accepted AE Titles for associations.
    ///
    /// If not specified then all AE Titles are accepted.
    pub accept_aets: Option<String>,

    #[arg(short, long)]
    /// The maximum number of concurrent associations.
    pub max_assoc: usize,

    #[arg(short, long)]
    /// The database URL for resolving DIMSE queries.
    ///
    /// If not specified then Query/Retrieve will not return results, and Store will not persist
    /// received series.
    pub db: Option<String>,
}
