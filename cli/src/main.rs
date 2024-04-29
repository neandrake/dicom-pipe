extern crate byteorder;
extern crate dcmpipe_lib;

use byteorder::{ByteOrder, BigEndian, LittleEndian};
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dict::dicom_elements as tags;
use dcmpipe_lib::core::dict::lookup::{TAG_BY_VALUE, UID_BY_ID};
use dcmpipe_lib::core::tag::Tag;
use dcmpipe_lib::core::ts::TSRef;
use dcmpipe_lib::core::vr;
use dcmpipe_lib::read::dcmparser::DicomStreamParser;
use dcmpipe_lib::read::tagstop::TagStop;
use dcmpipe_lib::read::CSRef;
use std::env;
use std::fs::File;
use std::io::Error;
use std::path::Path;

static MAX_BYTES_DISPLAY: usize = 16;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must specify dicom file to open");
    }
    let path: &Path = Path::new(&args[1]);

    if !path.is_file() {
        panic!(format!("Invalid path: {:?}", path));
    }

    let file: File = File::open(path)
        .expect(&format!("Unable to open file: {:?}", path));

    let mut dicom_iter: DicomStreamParser<File> = DicomStreamParser::new(file, TagStop::BeforeTag(tags::PixelData.tag));
    println!("\n# Dicom-File-Format File\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}", dicom_iter.get_ts().uid.ident);

    let mut prev_was_file_meta: bool = true;

    while let Some(elem) = dicom_iter.next() {
        if let Err(e) = elem {
            panic!(format!("{:?}", e));
        }
        let mut elem: DicomElement = elem.unwrap();
        if prev_was_file_meta && elem.tag > 0x0002FFFF {
            println!("\n# Dicom-Data-Set\n# Used TransferSyntax: {}", dicom_iter.get_ts().uid.ident);
            prev_was_file_meta = false;
        }
        let printed: Result<String, Error> = print_element(&mut elem, dicom_iter.get_ts(), dicom_iter.get_cs());
        if let Err(e) = printed {
            panic!(format!("{:?}", e));
        }
        let printed = printed.unwrap();
        println!("{}", printed);
    }
}

fn print_element(element: &mut DicomElement, ts: TSRef, cs: CSRef) -> Result<String, Error> {
    let tag_num: String = Tag::format_tag_to_display(element.tag);
    let tag_name: &str = if let Some(tag) = TAG_BY_VALUE.get(&element.tag) {
        tag.ident
    } else {
        "<Private Tag>"
    };
    let vr: &str = element.vr.ident;

    let cs: CSRef = element.vr.get_proper_cs(cs);
    let tag_value: String = if ts.big_endian {
        fmt_string_value::<BigEndian>(element, cs)?
    } else {
        fmt_string_value::<LittleEndian>(element, cs)?
    };

    Ok(format!("{} {} {} => {}", tag_num, vr, tag_name, tag_value))
}

/// Formats the value of this element as a string based on the VR
fn fmt_string_value<Endian: ByteOrder>(elem: &mut DicomElement, cs: CSRef) -> Result<String, Error> {
    if elem.is_empty() {
        return Ok("<EMPTY VALUE>".to_owned());
    }

    let mut sep: &str = ", ";
    let mut str_vals: Vec<String> = Vec::new();
    if elem.vr == &vr::AT {
        str_vals.push(Tag::format_tag_to_display(elem.parse_attribute::<Endian>()?));
    } else if elem.vr == &vr::FL || elem.vr == &vr::OF {
        sep = " / ";
        elem.parse_f32s::<Endian>()?.into_iter()
            .map(|val: f32| format!("{:.2}", val))
            .for_each(|val: String| str_vals.push(val));
    } else if elem.vr == &vr::FD || elem.vr == &vr::OD {
        sep = " / ";
        elem.parse_f64s::<Endian>()?.into_iter()
            .map(|val: f64| format!("{:.2}", val))
            .for_each(|val: String| str_vals.push(val));
    } else if elem.vr == &vr::SS || elem.vr == &vr::OW {
        sep = " / ";
        elem.parse_i16s::<Endian>()?.into_iter()
            .map(|val: i16| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
    } else if elem.vr == &vr::SL || elem.vr == &vr::OL {
        sep = " / ";
        elem.parse_i32s::<Endian>()?.into_iter()
            .map(|val: i32| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
    } else if elem.vr == &vr::UI {
        let str_val: String = elem.parse_string(cs)?;
        if let Some(uid) = UID_BY_ID.get(str_val.as_str()) {
            str_vals.push(format!("{} ({})", str_val, uid.name));
        } else {
            str_vals.push(str_val);
        }
    } else if elem.vr == &vr::UL {
        str_vals.push(format!("{}", elem.parse_u32::<Endian>()?));
    } else if elem.vr == &vr::US {
        str_vals.push(format!("{}", elem.parse_u16::<Endian>()?));
    } else if elem.vr.is_character_string {
        elem.parse_strings(cs)?.iter_mut()
            .map(|val: &mut String| format!("\"{}\"", val))
            .for_each(|val: String| str_vals.push(val));
    } else {
        let byte_vec: &Vec<u8> = elem.get_value().get_ref();
        byte_vec.iter()
            .take(MAX_BYTES_DISPLAY)
            .map(|val: &u8| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));

        if byte_vec.len() > MAX_BYTES_DISPLAY {
            str_vals.push("..".to_string());
        }
    }

    let num_vals: usize = str_vals.len();
    if num_vals == 1 {
        return Ok(str_vals.remove(0));
    }

    Ok(format!("[{}]", str_vals.into_iter().collect::<Vec<String>>().join(sep)))
}
