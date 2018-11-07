extern crate dcmpipe_dict;

use dcmpipe_dict::defnwrite;
use std::fs::File;
use std::path::Path;

static DICOM_CONSTANTS_HTML_FILE: &'static str = "build/dicom_xml/part06.xml";
static DEFAULT_LOCATION: &'static str = "http://dicom.nema.org/medical/dicom/current/source/docbook/part06/part06.xml";

fn main() {
    /// In theory this instructs cargo that this build script doesn't need run
    /// unless the file `DICOM_CONSTANTS_HTML_FILE` has changed.
    /// This does appear to work, however cargo is also "smart" and will ensure
    /// this build script runs fresh if this script file has changed even if the
    /// file hasn't.
    println!("cargo:rerun-if-changed=./{}", DICOM_CONSTANTS_HTML_FILE);

    let file: File = File::open(DICOM_CONSTANTS_HTML_FILE)
        .expect("Unable to load XML file");
    let folder: &Path = Path::new("src/core/dict/");

    defnwrite::process_xml_file(file, folder)
        .expect("Failed to process XML file");
}
