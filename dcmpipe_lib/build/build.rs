use dcmpipe_dict_builder::defnwrite;

use std::fs::File;
use std::path::Path;

// This file was downloaded from
// http://dicom.nema.org/medical/dicom/current/source/docbook/part06/part06.xml
static PART_06_DATA_DICTIONARY_XML_FILE: &str = "build/dicom_xml/part06.xml";
static PART_07_DIMSE_COMMANDS_XML_FILE: &str = "build/dicom_xml/part07.xml";

fn main() {
    // Causes this build script to re-run if the file is modified/changed
    println!(
        "cargo:rerun-if-changed=./{}",
        PART_06_DATA_DICTIONARY_XML_FILE
    );
    println!(
        "cargo:rerun-if-changed=./{}",
        PART_07_DIMSE_COMMANDS_XML_FILE
    );

    let err_msg = format!(
        "Unable to load XML file: {}",
        PART_06_DATA_DICTIONARY_XML_FILE
    );
    let part6: File = File::open(PART_06_DATA_DICTIONARY_XML_FILE).expect(&err_msg);

    let err_msg = format!(
        "Unable to load XML file: {}",
        PART_07_DIMSE_COMMANDS_XML_FILE
    );
    let part7: File = File::open(PART_07_DIMSE_COMMANDS_XML_FILE).expect(&err_msg);
    let dest_folder: &Path = Path::new("src/dict/");

    let files = vec![part6, part7];
    defnwrite::process_xml_files(files, dest_folder).expect("Failed to process XML file");
}
