extern crate csv;
extern crate htmltbl2csv;
extern crate rustc_serialize;
extern crate walkdir;

use csv::Reader;

use htmltbl2csv::CsvTable;

use std::ascii::AsciiExt;
use std::fs::File;
use std::io::{Cursor, Error, ErrorKind, Write};
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

static DICOM_CONSTANTS_HTML_FOLDER: &'static str = "build/dicom_constants_html/";

fn main() {
    let files: Vec<DirEntry> = collect_files();

    print_rerun_if_changed(&files)
        .expect("Unable to build rerun-if-changed");
    
    process_html_files(&files)
        .expect("Unable to build code files");
}

/// Gets the list of all files in the `DICOM_CONSTANTS_HTML_FOLDER` location
fn collect_files() -> Vec<DirEntry> {
    let dirwalker: WalkDir = WalkDir::new(DICOM_CONSTANTS_HTML_FOLDER)
        .min_depth(1)
        .max_depth(1);
    
    let mut files: Vec<DirEntry> = Vec::new();
    for dir_entry in dirwalker.into_iter() {
        if let Ok(entry) = dir_entry {
            files.push(entry);
        }
    }

    files
}

/// In theory this instructs cargo that this build script doesn't need run
/// unless the files in the `DICOM_CONSTANTS_HTML_FOLDER` directory changed.
/// This does appear to work, however cargo is also "smart" and will ensure
/// this build script runs fresh if this script file has changed even if the
/// files in `DICOM_CONSTANTS_HTML_FOLDER` haven't.
fn print_rerun_if_changed(files: &[DirEntry]) -> Result<(), Error> {
    for entry in files {
        let path: &Path = entry.path();
        println!("cargo:rerun-if-changed=./{}", path.to_str().unwrap());
    }
    Ok(())
}

/// Processes the HTML files in the given directory into CSV
/// the converts the CSV format into code, writing files into src/core/
fn process_html_files(files: &[DirEntry]) -> Result<(), Error> {
    for entry in files {
        let path: &Path = entry.path();
        let path_str: &str = path.to_str().unwrap();

        let table_type: Option<TableType> = TableType::from_filename(path_str);
        if table_type.is_none() {
            continue;
        }
        let table_type: TableType = table_type.unwrap();

        let csv_tables: Vec<CsvTable> = htmltbl2csv::process_input(path_str)?;
        let ref csv_table: CsvTable = csv_tables[2];
        let mut reader: Reader<Cursor<Vec<u8>>> = csv_table.process_to_reader()?;

        let mut defns: String = String::new();
        if table_type.is_element() {
            let elements: Vec<DataElement> = reader.decode()
                .collect::<csv::Result<Vec<DataElement>>>()
                .map_err(|e: csv::Error| Error::new(ErrorKind::InvalidData, e))?;
            
            for element in elements {
                if let Some(code) = table_type.process_element(&element) {
                    defns.push_str(&code);
                }
            }
        } else {
            let uids: Vec<Uid> = reader.decode()
                .collect::<csv::Result<Vec<Uid>>>()
                .map_err(|e: csv::Error| Error::new(ErrorKind::InvalidData, e))?;

            for uid in uids {
                if let Some(code) = table_type.process_uid(&uid) {
                    defns.push_str(&code);
                }
            }
        }

        let mut out_rs_file: File = File::create(table_type.filename()?)?;
        let preamble: String = table_type.get_codefile_preamble();

        out_rs_file.write_all(&preamble.into_bytes())?;
        out_rs_file.write_all(&defns.into_bytes())?;
        out_rs_file.flush()?;
        out_rs_file.sync_all()?;
    }

    Ok(())
}

/// The different tables we parse out of the HTML/CSV
#[derive(Eq, PartialEq, Debug)]
enum TableType {
    DicomElements,
    FileMetaElements,
    DirStructureElements,
    Uids,
}

impl TableType {
    /// Determines the table type being used based on the filename that parsed.
    /// This is dependent on the filenames following a specific format:
    /// 
    /// - "Part6.Ch6" => DicomElements
    /// - "Part6.Ch7" => FileMetaElements
    /// - "Part6.Ch8" => DirStructureElements
    /// - "Part6.ChA" => Uids
    pub fn from_filename(path_str: &str) -> Option<TableType> {
        if path_str.contains("Part6.Ch6") { Some(TableType::DicomElements) }
        else if path_str.contains("Part6.Ch7") { Some(TableType::FileMetaElements) }
        else if path_str.contains("Part6.Ch8") { Some(TableType::DirStructureElements) }
        else if path_str.contains("Part6.ChA") { Some(TableType::Uids) }
        else { None }
    }

    /// Whether this type is parsing as an element (Tag) or uid (...UID)
    pub fn is_element(&self) -> bool {
        match *self {
            TableType::Uids => false,
            _ => true
        }
    }

    /// The rust output code file that should be written to.
    /// The path will be relative to the working directory, which for this
    /// build script is the root of the project
    pub fn filename(&self) -> Result<&Path, Error> {
        match *self {
            TableType::DicomElements => Ok(Path::new("src/core/dict/dicom_elements.rs")),
            TableType::FileMetaElements => Ok(Path::new("src/core/dict/file_meta_elements.rs")),
            TableType::DirStructureElements => Ok(Path::new("src/core/dict/dir_structure_elements.rs")),
            TableType::Uids => Ok(Path::new("src/core/dict/uids.rs")),
        }
    }

    /// Cleans up a string to remove/replace characters found in the HTML
    /// so the variable name will be valid rust
    /// Some things straight up doesn't have a name aside from "(Retired)"~
    /// Such as: 1.2.840.10008.5.1.4.1.​1.​40 and (0018,0061)
    /// So check whether the returned string is empty
    fn sanitize_var_name(var_name: &String) -> String {
        let sanitized: String = var_name
            .replace(" ", "")
            .replace("(Retired)", "_Retired")
            .replace("(Trial)", "_Trial")
            .replace("-", "")
            .replace(",", "")
            .replace("(", "")
            .replace(")", "")
            .replace(".", "")
            .replace("/", "")
            .replace("[", "")
            .replace("]", "")
            .replace(":", "_")
            .replace("&", "_");
        
        if sanitized == "_Retired" || sanitized == "_Trial" {
            return String::new();
        }
        
        if let Some(first_char) = sanitized.chars().next() {
            if !first_char.is_ascii() || !first_char.is_alphabetic() {
                return format!("Tag_{}", sanitized);
            }
        }
        sanitized
    }

    /// Gets the preamble for the file -- things like `use` statements needed
    pub fn get_codefile_preamble(&self) -> String {
        if self.is_element() {
"//! This is an auto-generated file. Do not make modifications here.

use core::tag::Tag;
use core::vm::VM;
use core::vr;
".to_owned()
        } else {
"//! This is an auto-generated file. Do not make modifications here.

use core::uid::UID;
".to_owned()
        }
    }

    /// Processes a UID entry from CSV and returns the bit of code for it
    pub fn process_uid(&self, uid: &Uid) -> Option<String> {
        if self.is_element() {
            return None;
        }

        let var_name: String = TableType::sanitize_var_name(&uid.name);
        if var_name.is_empty() {
            return None;
        }
        
        let code: String = format!(
"/// {}
/// 
/// - **UID:** {}
/// - **UID Type:** {}
pub static {}: UID = UID {{
    ident: \"{}\",
    uid: \"{}\",
    name: \"{}\",
}};

",
            // comment
            uid.name, uid.value, uid.uid_type,
            // definitions
            var_name, var_name, uid.value, uid.name);

        Some(code)
    }

    /// Processes an element entry from CSV and returns the bit of code for it
    pub fn process_element(&self, element: &DataElement) -> Option<String> {
        if !self.is_element() {
            return None;
        }

        let var_name: String = TableType::sanitize_var_name(&element.keyword);
        if var_name.is_empty() {
            return None;
        }

        let vr: &str = element.vr.split_whitespace().next().unwrap();
        let vr_value: String =
            if vr == "See" { "None".to_owned() }
            else { format!("Some(&vr::{})", vr) };
        
        let vm: String =
            if element.vm == "1-n or 1" { "&VM::OneOrMore".to_owned() }
            else if let Ok(vm_val) = element.vm.parse::<u32>() { format!("&VM::Distinct({})", vm_val) }
            else {
                let parts: Vec<&str> = element.vm.split('-').collect::<Vec<&str>>();
                let start: u32 = parts[0].parse::<u32>().expect(format!("Missing start to VM: {}", element.vm).as_ref());
                let end: &str = parts[1];

                if end == "n" { format!("&VM::AtLeast({})", start) }
                else if let Ok(end_val) = end.parse::<u32>() { format!("&VM::AtMost({})", end_val) }
                else { format!("&VM::MultipleOf({})", start) }
            };

        let code: String = format!(
"/// {}
/// 
/// - **Tag:** {}
/// - **VR:** {}
/// - **VM:** {}
pub static {}: Tag = Tag {{
    ident: \"{}\",
    tag: \"{}\",
    implicit_vr: {},
    vm: {},
    desc: \"{}\",
}};

",
            // comment
            element.name, element.tag, vr, element.vm,
            // definitions
            var_name, var_name, element.tag, vr_value, vm, element.name);

        Some(code)
    }
}

// The following decodable structs are used by the CSV reader to
// convert values from CSV into an struct/object. Each field will map
// directly to its ordinal column, so order of defined field is important

#[derive(RustcDecodable)]
struct DataElement {
    tag: String,
    name: String,
    keyword: String,
    vr: String,
    vm: String,

    #[allow(dead_code)]
    misc: String,
}

#[derive(RustcDecodable)]
struct Uid {
    value: String,
    name: String,
    uid_type: String,

    #[allow(dead_code)]
    part: String,
}