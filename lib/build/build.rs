extern crate dcmpipe_dict;

use dcmpipe_dict::defnwrite;
use std::fs::File;
use std::path::Path;

// This file was downloaded from
// http://dicom.nema.org/medical/dicom/current/source/docbook/part06/part06.xml
static DICOM_CONSTANTS_HTML_FILE: &'static str = "build/dicom_xml/part06.xml";

fn main() {
    // Causes this build script to re-run if the file is modified/changed
    println!("cargo:rerun-if-changed=./{}", DICOM_CONSTANTS_HTML_FILE);

    let file: File = File::open(DICOM_CONSTANTS_HTML_FILE)
        .expect("Unable to load XML file");
    let folder: &Path = Path::new("src/core/dict/");

    defnwrite::process_xml_file(file, folder)
        .expect("Failed to process XML file");
}
