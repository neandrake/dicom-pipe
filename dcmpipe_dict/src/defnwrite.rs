use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Write};
use std::path::{Path, PathBuf};

use crate::xmlparser::{
    XmlDicomDefinition, XmlDicomDefinitionIterator, XmlDicomDefinitionResult, XmlDicomElement,
    XmlDicomUid,
};

static LOOKUP_PREAMBLE: &'static str =
    "//! This is an auto-generated file. Do not make modifications here.

use crate::defn::tag::TagRef;
use crate::defn::ts::TSRef;
use crate::defn::uid::UIDRef;
use crate::dict::dicom_elements as tags;
use crate::dict::dir_structure_elements as dse;
use crate::dict::file_meta_elements as fme;
use crate::dict::transfer_syntaxes as ts;
use crate::dict::uids;

";

static DICOM_ELEMENT_PREAMBLE: &'static str =
    "//! This is an auto-generated file. Do not make modifications here.

#![allow(non_upper_case_globals)]

use crate::defn::tag::Tag;
use crate::defn::vm::VM;
use crate::defn::vr;

";

static TRANSFER_SYNTAX_PREAMBLE: &'static str =
    "//! This is an auto-generated file. Do not make modifications here.

#![allow(non_upper_case_globals)]

use crate::defn::ts::TransferSyntax;
use crate::dict::uids;

";

static UID_PREAMBLE: &'static str =
    "//! This is an auto-generated file. Do not make modifications here.

#![allow(non_upper_case_globals)]

use crate::defn::uid::UID;

";

macro_rules! uid_definition {
    ($($args:tt)*) => {
        format!("/// {}
///
/// - **UID:** {}
/// - **UID Type:** {}
pub static {}: UID = UID {{
    ident: \"{}\",
    uid: \"{}\",
    name: \"{}\",
}};

", $($args)*)
    };
}

macro_rules! transfer_syntax_definition {
    ($($args:tt)*) => {
        format!(
"/// {}
///
/// - **UID:** {}
pub static {}: TransferSyntax = TransferSyntax {{
    uid: {},
    explicit_vr: {},
    big_endian: {},
    deflated: {},
    encapsulated: {},
}};

", $($args)*)
    };
}

macro_rules! dicom_element_definition {
    ($($args:tt)*) => {
        format!(
"/// {}
///
/// - **Tag:** {}
/// - **VR:** {}
/// - **VM:** {}
pub static {}: Tag = Tag {{
    ident: \"{}\",
    tag: 0x{:04X}_{:04X},
    implicit_vr: {},
    vm: {},
    desc: \"{}\",
}};

", $($args)*)
    };
}

pub fn process_xml_file(file: File, folder: &Path) -> Result<(), Error> {
    let bufread: BufReader<File> = BufReader::new(file);
    let xml_definitions: Vec<XmlDicomDefinition> = XmlDicomDefinitionIterator::new(bufread)
        .map(|item: XmlDicomDefinitionResult| {
            item.unwrap_or_else(|_| panic!("Error parsing XML dicom entry"))
        })
        .collect::<Vec<XmlDicomDefinition>>();

    process_entries(xml_definitions, folder)?;

    Ok(())
}

fn process_entries(xml_definitions: Vec<XmlDicomDefinition>, folder: &Path) -> Result<(), Error> {
    let mut tag_ident_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();
    let mut tag_tag_lookup_phf: phf_codegen::Map<u32> = phf_codegen::Map::new();

    let mut uid_ident_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();
    let mut uid_id_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();

    let mut ts_ident_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();
    let mut ts_id_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();

    let mut dicom_elements: String = String::new();
    let mut file_meta_elements: String = String::new();
    let mut dir_structure_elements: String = String::new();
    let mut uids: String = String::new();
    let mut transfer_syntax_uids: String = String::new();

    for defn in xml_definitions {
        match defn {
            XmlDicomDefinition::DicomElement(e) => {
                if let Some(code) = process_element(
                    &e,
                    "tags::",
                    &mut tag_ident_lookup_phf,
                    &mut tag_tag_lookup_phf,
                ) {
                    dicom_elements.push_str(&code);
                }
            }
            XmlDicomDefinition::FileMetaElement(e) => {
                if let Some(code) = process_element(
                    &e,
                    "fme::",
                    &mut tag_ident_lookup_phf,
                    &mut tag_tag_lookup_phf,
                ) {
                    file_meta_elements.push_str(&code);
                }
            }
            XmlDicomDefinition::DirStructureElement(e) => {
                if let Some(code) = process_element(
                    &e,
                    "dse::",
                    &mut tag_ident_lookup_phf,
                    &mut tag_tag_lookup_phf,
                ) {
                    dir_structure_elements.push_str(&code);
                }
            }
            XmlDicomDefinition::Uid(uid) => {
                if let Some(code) =
                    process_uid(&uid, &mut uid_ident_lookup_phf, &mut uid_id_lookup_phf)
                {
                    uids.push_str(&code);
                }
            }
            XmlDicomDefinition::TransferSyntax(uid) => {
                if let Some(code) =
                    process_uid(&uid, &mut uid_ident_lookup_phf, &mut uid_id_lookup_phf)
                {
                    uids.push_str(&code);
                }
                if let Some(code) =
                    process_transfer_syntax(&uid, &mut ts_ident_lookup_phf, &mut ts_id_lookup_phf)
                {
                    transfer_syntax_uids.push_str(&code);
                }
            }
        }
    }

    // Remove trailing newlines
    dicom_elements.remove(dicom_elements.len() - 2);
    file_meta_elements.remove(file_meta_elements.len() - 2);
    dir_structure_elements.remove(dir_structure_elements.len() - 2);
    uids.remove(uids.len() - 2);
    transfer_syntax_uids.remove(transfer_syntax_uids.len() - 2);

    std::fs::create_dir_all(folder)?;

    let path_buf: PathBuf = folder.to_path_buf();
    let mut dicom_elements_file = path_buf.clone();
    dicom_elements_file.push("dicom_elements.rs");
    let mut file_meta_elements_file = path_buf.clone();
    file_meta_elements_file.push("file_meta_elements.rs");
    let mut dir_structure_elements_file = path_buf.clone();
    dir_structure_elements_file.push("dir_structure_elements.rs");
    let mut uids_file = path_buf.clone();
    uids_file.push("uids.rs");
    let mut transfer_syntaxes_file = path_buf.clone();
    transfer_syntaxes_file.push("transfer_syntaxes.rs");

    save_codefile(
        dicom_elements_file.as_path(),
        DICOM_ELEMENT_PREAMBLE.to_owned(),
        &dicom_elements,
    )?;
    save_codefile(
        file_meta_elements_file.as_path(),
        DICOM_ELEMENT_PREAMBLE.to_owned(),
        &file_meta_elements,
    )?;
    save_codefile(
        dir_structure_elements_file.as_path(),
        DICOM_ELEMENT_PREAMBLE.to_owned(),
        &dir_structure_elements,
    )?;
    save_codefile(uids_file.as_path(), UID_PREAMBLE.to_owned(), &uids)?;
    save_codefile(
        transfer_syntaxes_file.as_path(),
        TRANSFER_SYNTAX_PREAMBLE.to_owned(),
        &transfer_syntax_uids,
    )?;

    let mut lookup_file_path: PathBuf = folder.to_path_buf().clone();
    lookup_file_path.push("lookup.rs");

    let mut lookup_file: BufWriter<File> =
        BufWriter::new(File::create(lookup_file_path.as_path()).unwrap());
    write!(&mut lookup_file, "{}", LOOKUP_PREAMBLE)?;

    write!(
        &mut lookup_file,
        "pub static TAG_BY_IDENT: phf::Map<&'static str, TagRef> = "
    )?;
    tag_ident_lookup_phf.build(&mut lookup_file)?;

    write!(&mut lookup_file, ";\n\n")?;
    write!(
        &mut lookup_file,
        "pub static TAG_BY_VALUE: phf::Map<u32, TagRef> = "
    )?;
    tag_tag_lookup_phf.build(&mut lookup_file)?;

    write!(&mut lookup_file, ";\n\n")?;
    write!(
        &mut lookup_file,
        "pub static TS_BY_IDENT: phf::Map<&'static str, TSRef> = "
    )?;
    ts_ident_lookup_phf.build(&mut lookup_file)?;

    write!(&mut lookup_file, ";\n\n")?;
    write!(
        &mut lookup_file,
        "pub static TS_BY_ID: phf::Map<&'static str, TSRef> = "
    )?;
    ts_id_lookup_phf.build(&mut lookup_file)?;

    write!(&mut lookup_file, ";\n\n")?;
    write!(
        &mut lookup_file,
        "pub static UID_BY_IDENT: phf::Map<&'static str, UIDRef> = "
    )?;
    uid_ident_lookup_phf.build(&mut lookup_file)?;

    write!(&mut lookup_file, ";\n\n")?;
    write!(
        &mut lookup_file,
        "pub static UID_BY_ID: phf::Map<&'static str, UIDRef> = "
    )?;
    uid_id_lookup_phf.build(&mut lookup_file)?;
    write!(&mut lookup_file, ";\n")?;

    Ok(())
}

/// Processes a UID entry into code definition
fn process_uid(
    uid: &XmlDicomUid,
    ident_lookup: &mut phf_codegen::Map<String>,
    id_lookup: &mut phf_codegen::Map<String>,
) -> Option<String> {
    let var_name: String = sanitize_var_name(&uid.name);
    if var_name.is_empty() {
        return None;
    }

    let type_clone = uid.uid_type.clone();
    let comment_uid_type: String = if let Some(uid_type) = type_clone {
        uid_type
    } else {
        String::new()
    };

    let code: String = uid_definition!(
        uid.name,
        uid.value,
        comment_uid_type, // comment placeholders
        var_name,
        var_name,
        uid.value,
        uid.name
    ); // field placeholders

    let var_name_key: String = var_name.clone();
    ident_lookup.entry(var_name_key, &format!("&uids::{}", var_name));

    let id_val_key: String = uid.value.clone();
    id_lookup.entry(id_val_key, &format!("&uids::{}", var_name));

    Some(code)
}

/// Processes a TransferSyntax UID into code definition
fn process_transfer_syntax(
    uid: &XmlDicomUid,
    ident_lookup: &mut phf_codegen::Map<String>,
    id_lookup: &mut phf_codegen::Map<String>,
) -> Option<String> {
    let var_name: String = sanitize_var_name(&uid.name);
    if var_name.is_empty() {
        return None;
    }

    let var_uid: String = format!("&uids::{}", var_name);
    let explicit_vr_val: String = if var_name.contains("Explicit") {
        "true"
    } else {
        "false"
    }
    .to_owned();

    // Part 5 section 7.3
    // All unretired Transfer Syntaxes in DICOM require the use of Little Endian Byte Ordering.
    let big_endian_val: String = if var_name.contains("BigEndian") {
        "true"
    } else {
        "false"
    }
    .to_owned();

    let deflated_val: String = if var_name.contains("Deflate") {
        "true"
    } else {
        "false"
    }
    .to_owned();

    // This assumes if it doesn't state "implicit" or "explicit" then it would be encapsulated
    // Due to this excerpt from the DICOM standard (Part 5 section 8.2):
    // "Pixel data conveyed in the Pixel Data (7FE0,0010) may be sent either
    // in a Native (uncompressed) Format or in an Encapsulated Format (e.g., compressed)
    // defined outside the DICOM standard.""

    let encapsulated_val: String = if var_name.contains("Encapsulate") {
        "true"
    } else {
        "false"
    }
    .to_owned();

    let code: String = transfer_syntax_definition!(
        uid.name,
        uid.value,
        var_name, // comment placeholders
        var_uid,
        explicit_vr_val,
        big_endian_val,
        deflated_val,
        encapsulated_val
    ); // field placeholders
    let var_name_key: String = var_name.clone();
    ident_lookup.entry(var_name_key, &format!("&ts::{}", var_name));
    let id_val_lookup: String = uid.value.clone();
    id_lookup.entry(id_val_lookup, &format!("&ts::{}", var_name));

    Some(code)
}

/// Processes a dicom element into code definition
fn process_element(
    element: &XmlDicomElement,
    dict: &str,
    ident_lookup: &mut phf_codegen::Map<String>,
    tag_lookup: &mut phf_codegen::Map<u32>,
) -> Option<String> {
    let var_name: String = sanitize_var_name(&element.name);
    if var_name.is_empty() {
        return None;
    }

    let tag_value: u32 = u32::from_str_radix(
        &element
            .tag
            .replace("(", "")
            .replace(")", "")
            .replace(",", ""),
        16,
    )
    .unwrap_or(0);

    if tag_value == 0 {
        return None;
    }

    let tag_group: u32 = (tag_value >> 16) & 0x0000_FFFF;
    let tag_element: u32 = tag_value & 0x0000_FFFF;

    let vr: &str = element.vr.split_whitespace().next().unwrap();
    let vr_value: String = if vr == "See" {
        "None".to_owned()
    } else {
        format!("Some(&vr::{})", vr)
    };

    let vm: String = if element.vm == "1-n or 1".to_string() {
        "&VM::OneOrMore".to_owned()
    } else if let Ok(vm_val) = element.vm.parse::<u32>() {
        format!("&VM::Distinct({})", vm_val)
    } else {
        let parts: Vec<&str> = element.vm.split('-').collect::<Vec<&str>>();
        let start: u32 = parts[0]
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Missing start to VM: {}", element.vm));
        let end: &str = parts[1];

        if end == "n" {
            format!("&VM::AtLeast({})", start)
        } else if let Ok(end_val) = end.parse::<u32>() {
            format!("&VM::AtMost({})", end_val)
        } else {
            format!("&VM::MultipleOf({})", start)
        }
    };

    let code: String = dicom_element_definition!(
        element.name,
        element.tag,
        vr,
        element.vm, // comment placeholders
        var_name,
        var_name,
        tag_group,
        tag_element,
        vr_value,
        vm,
        element.name
    ); // field placeholders

    let var_name_key: String = var_name.clone();
    ident_lookup.entry(var_name_key, &format!("&{}{}", dict, var_name));
    tag_lookup.entry(tag_value, &format!("&{}{}", dict, var_name));

    Some(code)
}

fn save_codefile(filename: &Path, preamble: String, code: &str) -> Result<(), Error> {
    let mut out_rs_file: File = File::create(filename)?;
    out_rs_file.write_all(&preamble.into_bytes())?;
    out_rs_file.write_all(code.as_bytes())?;
    out_rs_file.flush()?;
    out_rs_file.sync_all()?;
    Ok(())
}

/// Cleans up the definition name to be a valid rust identifier.
/// Some things straight up don't have a name aside from "(Retired)" o.O
/// Such as: 1.2.840.10008.5.1.4.1.​1.​40 and (0018,0061)
/// So check whether the returned string is empty
fn sanitize_var_name(var_name: &str) -> String {
    let is_retired: bool = var_name.contains("Retired");
    // let is_trial: bool = var_name.contains("Trial");

    let mut sanitized: String = var_name
        .replace("(Retired)", "")
        .replace("(Trial)", "_Trial")
        .replace(" ", "")
        .replace("'", "")
        .replace("-", "")
        .replace(",", "")
        .replace("(", "")
        .replace(")", "")
        .replace(".", "")
        .replace("/", "")
        .replace("[", "")
        .replace("]", "")
        .replace("\"", "_")
        .replace("&", "_and_")
        .replace("µ", "micro")
        .split(':')
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
    if is_retired
        && (sanitized == "UltrasoundMultiframeImageStorage".to_string()
            || sanitized == "UltrasoundImageStorage".to_string()
            || sanitized == "NuclearMedicineImageStorage".to_string()
            || sanitized == "DopplerSampleVolumeXPosition".to_string()
            || sanitized == "DopplerSampleVolumeYPosition".to_string()
            || sanitized == "TMLinePositionX0".to_string()
            || sanitized == "TMLinePositionY0".to_string()
            || sanitized == "TMLinePositionX1".to_string()
            || sanitized == "TMLinePositionY1".to_string()
            || sanitized == "ParallelReductionFactorInplane".to_string()
            || sanitized == "LossyImageCompression".to_string()
            || sanitized == "PlacerOrderNumberImagingServiceRequest".to_string()
            || sanitized == "FillerOrderNumberImagingServiceRequest".to_string()
            || sanitized == "ImageRotation".to_string()
            || sanitized == "ReferencedImageBoxSequence".to_string())
    {
        sanitized = format!("{}_Retired", sanitized);
    }

    // if is_trial
    //     && (sanitized == "SpecimenDescriptionSequence"
    //     || sanitized == "RequestedProcedureDescription"
    //     || sanitized == "AlgorithmDescription"
    //     || sanitized == "LanguageCodeSequence"
    //     || sanitized == "DisplayedAreaTopLeftHandCorner"
    //     || sanitized == "DisplayedAreaBottomRightHandCorner"
    //     || sanitized == "BeamOrderIndex"
    //     || sanitized == "DoubleExposureMeterset"
    //     || sanitized == "DoubleExposureFieldDelta") {
    //     sanitized = format!("{}_Trial", sanitized);
    // }

    if let Some(first_char) = sanitized.chars().next() {
        if !first_char.is_ascii() || !first_char.is_alphabetic() {
            return format!("Tag_{}", sanitized);
        }
    }
    sanitized
}
