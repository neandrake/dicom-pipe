//! This is a build script that parses a few of the DICOM standard HTML pages
//! into Rust code for DICOM constants. It specifies to cargo that it should
//! only be run if the HTML files in build/dicom_constants_html change. This
//! script automatically runs as part of `cargo build` due to it being
//! specified in `Cargo.toml`
//!
//! I'm not really clear at what point this runs during the build process. According
//! to the documentation of build scripts it seems that this is only supposed to be
//! putting generated code in an environment variable `OUT_DIR` which is in the target
//! directory and not part of the repository. This script places the files it generates
//! into src/core/dict instead and the resulting files may or may not be compiled
//! after this script. It seems that sometimes I have to build twice to get changes made
//! from this file (but not every time).
//!
//! This build script depends on htmltbl2csv which is not a published crate. In order
//! for this to build properly the htmltbl2csv repo should be checked out next to dcmpipe:
//! ```text
//! ./dcmpipe/
//! ./htmltbl2csv
//! ```
//!
//! This also currently relies on a naming scheme of the HTML files in order to
//! interpret which part of the DICOM standard definitions are being parsed.
//! See `TableType::from_filename()`

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
    let mut tag_lookup_content: String = String::new();
    let mut uid_lookup_content: String = String::new();
    let mut transfer_syntax_lookup_content: String = String::new();
    let mut transfer_syntaxes: String = String::new();
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
                if let Some(code) = table_type.process_element_lookup(&element) {
                    tag_lookup_content.push_str(&code);
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
                if let Some(code) = table_type.process_uid_lookup(&uid) {
                    uid_lookup_content.push_str(&code);
                }

                if uid.uid_type == "Transfer Syntax" {
                    if let Some(code) = table_type.process_transfer_syntax(&uid) {
                        transfer_syntaxes.push_str(&code);
                    }
                    if let Some(code) = table_type.process_transfer_syntax_lookup(&uid) {
                        transfer_syntax_lookup_content.push_str(&code);
                    }
                }
            }
        }

        save_codefile(&table_type, &defns)?;
    }

    if !transfer_syntaxes.is_empty() {
        save_codefile(&TableType::TransferSyntaxes, &transfer_syntaxes)?;
    }

    if !tag_lookup_content.is_empty() {
        let code: String = get_element_lookup(&tag_lookup_content);
        let mut out_rs_file: File = File::create("src/core/dict/tag_lookup.rs")?;
        out_rs_file.write_all(code.as_bytes())?;
        out_rs_file.flush()?;
        out_rs_file.sync_all()?;
    }

    if !uid_lookup_content.is_empty() {
        let code: String = get_uid_lookup(&uid_lookup_content);
        let mut out_rs_file: File = File::create("src/core/dict/uid_lookup.rs")?;
        out_rs_file.write_all(code.as_bytes())?;
        out_rs_file.flush()?;
        out_rs_file.sync_all()?;
    }

    if !transfer_syntax_lookup_content.is_empty() {
        let code: String = get_transfer_syntax_lookup(&transfer_syntax_lookup_content);
        let mut out_rs_file: File = File::create("src/core/dict/ts_lookup.rs")?;
        out_rs_file.write_all(code.as_bytes())?;
        out_rs_file.flush()?;
        out_rs_file.sync_all()?;
    }

    Ok(())
}

fn save_codefile(table_type: &TableType, code: &str) -> Result<(), Error> {
    let mut out_rs_file: File = File::create(table_type.filename()?)?;
    let preamble: String = table_type.get_codefile_preamble();

    out_rs_file.write_all(&preamble.into_bytes())?;
    out_rs_file.write_all(code.as_bytes())?;
    out_rs_file.flush()?;
    out_rs_file.sync_all()?;
    Ok(())
}

fn get_element_lookup(content: &str) -> String {
    format!("//! This is an auto-generated file. Do not make modifications here.

use core::dict::dicom_elements as tags;
use core::dict::dir_structure_elements as dse;
use core::dict::file_meta_elements as fme;
use core::tag::Tag;

use std::collections::hash_map::HashMap;

pub struct TagLookup {{
\tident_to_elem: HashMap<&'static str, &'static Tag>,
\ttag_to_elem: HashMap<u32, &'static Tag>,
}}

impl TagLookup {{
\tpub fn by_ident(&self, ident: &str) -> Option<&'static Tag> {{
\t\tself.ident_to_elem.get(ident).map(|tag| *tag)
\t}}

\tpub fn by_tag(&self, tag: &u32) -> Option<&'static Tag> {{
\t\tself.tag_to_elem.get(tag).map(|tag| *tag)
\t}}

\tpub fn new() -> TagLookup {{
\t\tlet mut ident_to_elem: HashMap<&'static str, &'static Tag> = HashMap::new();
\t\tlet mut tag_to_elem: HashMap<u32, &'static Tag> = HashMap::new();
{}

\t\tTagLookup {{
\t\t\tident_to_elem: ident_to_elem,
\t\t\ttag_to_elem: tag_to_elem,
\t\t}}
\t}}
}}

",
    content)
}

fn get_uid_lookup(content: &str) -> String {
    format!("//! This is an auto-generated file. Do not make modifications here.

use core::dict::uids;
use core::uid::UID;

use std::collections::hash_map::HashMap;

pub struct UidLookup {{
\tident_to_uid: HashMap<&'static str, &'static UID>,
\tid_to_uid: HashMap<&'static str, &'static UID>,
}}

impl UidLookup {{
\tpub fn by_ident(&self, ident: &str) -> Option<&'static UID> {{
\t\tself.ident_to_uid.get(ident).map(|uid| *uid)
\t}}

\tpub fn by_id(&self, id: &str) -> Option<&'static UID> {{
\t\tself.id_to_uid.get(id).map(|uid| *uid)
\t}}

\tpub fn new() -> UidLookup {{
\t\tlet mut ident_to_uid: HashMap<&'static str, &'static UID> = HashMap::new();
\t\tlet mut id_to_uid: HashMap<&'static str, &'static UID> = HashMap::new();
{}

\t\tUidLookup {{
\t\t\tident_to_uid: ident_to_uid,
\t\t\tid_to_uid: id_to_uid,
\t\t}}
\t}}
}}
",
    content)
}

fn get_transfer_syntax_lookup(content: &str) -> String {
    format!("//! This is an auto-generated file. Do not make modifications here.

use core::dict::transfer_syntaxes as ts;
use core::dict::uids;
use core::ts::TransferSyntax;
use core::uid::UID;

use std::collections::hash_map::HashMap;

pub struct TransferSyntaxLookup {{
\tident_to_ts: HashMap<&'static str, &'static TransferSyntax>,
\tid_to_ts: HashMap<&'static str, &'static TransferSyntax>,
\tuid_to_ts: HashMap<&'static UID, &'static TransferSyntax>,
}}

impl TransferSyntaxLookup {{
\tpub fn by_ident(&self, ident: &str) -> Option<&'static TransferSyntax> {{
\t\tself.ident_to_ts.get(ident).map(|ts| *ts)
\t}}

\tpub fn by_id(&self, id: &str) -> Option<&'static TransferSyntax> {{
\t\tself.id_to_ts.get(id).map(|ts| *ts)
\t}}

\tpub fn by_uid(&self, uid: &UID) -> Option<&'static TransferSyntax> {{
\t\tself.uid_to_ts.get(uid).map(|ts| *ts)
\t}}

\tpub fn new() -> TransferSyntaxLookup {{
\t\tlet mut ident_to_ts: HashMap<&'static str, &'static TransferSyntax> = HashMap::new();
\t\tlet mut id_to_ts: HashMap<&'static str, &'static TransferSyntax> = HashMap::new();
\t\tlet mut uid_to_ts: HashMap<&'static UID, &'static TransferSyntax> = HashMap::new();
{}

\t\tTransferSyntaxLookup {{
\t\t\tident_to_ts: ident_to_ts,
\t\t\tid_to_ts: id_to_ts,
\t\t\tuid_to_ts: uid_to_ts,
\t\t}}
\t}}
}}
",
    content)
}

/// The different tables we parse out of the HTML/CSV
#[derive(Eq, PartialEq, Debug)]
enum TableType {
    DicomElements,
    FileMetaElements,
    DirStructureElements,
    Uids,
    TransferSyntaxes,
}

impl TableType {
    /// Determines the table type being used based on the filename that parsed.
    /// This is dependent on the filenames following a specific format:
    /// 
    /// - "Part6.Ch6" => DicomElements
    /// - "Part6.Ch7" => FileMetaElements
    /// - "Part6.Ch8" => DirStructureElements
    /// - "Part6.ChA" => Uids
    /// - Transfer Syntaxes are defined within the Uids table
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
            TableType::Uids | TableType::TransferSyntaxes => false,
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
            TableType::TransferSyntaxes => Ok(Path::new("src/core/dict/transfer_syntaxes.rs")),
        }
    }

    /// Cleans up a string to remove/replace characters found in the HTML
    /// so the variable name will be valid rust
    /// Some things straight up doesn't have a name aside from "(Retired)"~
    /// Such as: 1.2.840.10008.5.1.4.1.​1.​40 and (0018,0061)
    /// So check whether the returned string is empty
    fn sanitize_var_name(var_name: &String) -> String {
        let is_retired: bool = var_name.contains("Retired");

        let mut sanitized: String = var_name
            .replace(" ", "")
            .replace("(Retired)", "")
            .replace("(Trial)", "")
            .replace("-", "")
            .replace(",", "")
            .replace("(", "")
            .replace(")", "")
            .replace(".", "")
            .replace("/", "")
            .replace("[", "")
            .replace("]", "")
            .replace("&", "_and_")
            .split(":")
            .next()
            .unwrap()
            .to_owned();
        
        if sanitized.is_empty() {
            return sanitized;
        }

        // I think in general having "Retired" in the tag name is undesired
        // (see Explicit VR Big Endian (Retired)..)
        // However if we just remove "(Retired)" then it results in a
        // few duplicate definitions so we'll add back "_Retired"
        if is_retired &&
            (sanitized == "UltrasoundMultiframeImageStorage" ||
            sanitized == "UltrasoundImageStorage" ||
            sanitized == "NuclearMedicineImageStorage") {
            sanitized = format!("{}_Retired", sanitized);
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

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::tag::Tag;
use core::vm::VM;
use core::vr;

".to_owned()
        } else if *self == TableType::Uids {
"//! This is an auto-generated file. Do not make modifications here.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::uid::UID;

".to_owned()
        } else {
"//! This is an auto-generated file. Do not make modifications here.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::dict::uids;
use core::ts::TransferSyntax;

".to_owned()
        }
    }

    /// Processes a UID entry from CSV and returns the bit of code for it
    pub fn process_uid(&self, uid: &Uid) -> Option<String> {
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
\tident: \"{}\",
\tuid: \"{}\",
\tname: \"{}\",
}};

",
            // comment
            uid.name, uid.value, uid.uid_type,
            // definitions
            var_name, var_name, uid.value, uid.name);

        Some(code)
    }

    /// Processes a UID entry from CSV as a TransferSyntax and returns the bit of code for it
    pub fn process_transfer_syntax(&self, uid: &Uid) -> Option<String> {
        let var_name: String = TableType::sanitize_var_name(&uid.name);
        if var_name.is_empty() {
            return None;
        }

        let var_uid: String = format!("&uids::{}", var_name);
        let explicit_vr_val: String =
            if var_name.contains("Explicit") { "true" } else { "false" }.to_owned();
        
        let big_endian_val: String =
            if var_name.contains("BigEndian") { "true" } else { "false" }.to_owned();
        
        let deflated_val: String =
            if var_name.contains("Deflated") { "true"} else { "false" }.to_owned();

        // This assumes if it doesn't state "implicit" or "explicit" then it would be encapsulated
        // Due to this excerpt from the DICOM standard (Part 5 section 8.2): 
        // "Pixel data conveyed in the Pixel Data (7FE0,0010) may be sent either
        // in a Native (uncompressed) Format or in an Encapsulated Format (e.g., compressed)
        // defined outside the DICOM standard.""
        
        let encapsulated_val: String =
            if !var_name.contains("Explicit") && !var_name.contains("Implicit") { "true" } else { "false" }.to_owned();

        let code: String = format!(
"/// {}
/// 
/// - **UID:** {}
pub static {}: TransferSyntax = TransferSyntax {{
\tuid: {},
\texplicit_vr: {},
\tbig_endian: {},
\tdeflated: {},
\tencapsulated: {},
}};

",
            // comment
            uid.name, uid.value, var_name,
            // definitions
            var_uid, explicit_vr_val, big_endian_val, deflated_val, encapsulated_val);

        Some(code)
    }

    pub fn process_uid_lookup(&self, uid: &Uid) -> Option<String> {
        let var_name: String = TableType::sanitize_var_name(&uid.name);
        if var_name.is_empty() {
            return None;
        }

        let code: String = format!(
"
\t\tident_to_uid.insert(\"{}\", &uids::{});
\t\tid_to_uid.insert(\"{}\", &uids::{});
",
            var_name, var_name,
            uid.value, var_name,
        );

        Some(code)
    }

    pub fn process_transfer_syntax_lookup(&self, uid: &Uid) -> Option<String> {
        let var_name: String = TableType::sanitize_var_name(&uid.name);
        if var_name.is_empty() {
            return None;
        }

        let code: String = format!(
"
\t\tident_to_ts.insert(\"{}\", &ts::{});
\t\tid_to_ts.insert(\"{}\", &ts::{});
\t\tuid_to_ts.insert(&uids::{}, &ts::{});
",
            var_name, var_name,
            uid.value, var_name,
            var_name, var_name,
        );

        Some(code)
    }

    /// Processes an element entry from CSV and returns the bit of code for it
    pub fn process_element(&self, element: &DataElement) -> Option<String> {
        let var_name: String = TableType::sanitize_var_name(&element.keyword);
        if var_name.is_empty() {
            return None;
        }

        let tag_value: u32 =
            u32::from_str_radix(
                &element.tag
                    .replace("(", "")
                    .replace(")", "")
                    .replace(",", ""),
                16)
            .unwrap_or(0);
        
        if tag_value == 0 {
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
                let start: u32 = parts[0].parse::<u32>().expect(&format!("Missing start to VM: {}", element.vm));
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
\tident: \"{}\",
\ttag: 0x{:08X},
\timplicit_vr: {},
\tvm: {},
\tdesc: \"{}\",
}};

",
            // comment
            element.name, element.tag, vr, element.vm,
            // definitions
            var_name, var_name, tag_value, vr_value, vm, element.name);

        Some(code)
    }

    fn process_element_lookup(&self, element: &DataElement) -> Option<String> {
        let var_name: String = TableType::sanitize_var_name(&element.keyword);
        if var_name.is_empty() {
            return None;
        }

        let tag_value: u32 =
            u32::from_str_radix(
                &element.tag
                    .replace("(", "")
                    .replace(")", "")
                    .replace(",", ""),
                16)
            .unwrap_or(0);
        if tag_value == 0 {
            return None;
        }

        let dict: &str = match *self {
            TableType::DicomElements => "tags::",
            TableType::FileMetaElements => "fme::",
            TableType::DirStructureElements => "dse::",
            _ => return None
        };

        let code: String = format!("
\t\tident_to_elem.insert(\"{}\", &{}{});
\t\ttag_to_elem.insert(0x{:08X}, &{}{});
",
            var_name, dict, var_name,
            tag_value, dict, var_name,
        );

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