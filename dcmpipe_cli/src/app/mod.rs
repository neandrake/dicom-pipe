use std::fs::File;
use std::iter::Peekable;
use std::path::Path;

use anyhow::{anyhow, Result};
use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_lib::core::dcmelement::{DicomElement, RawValue};
use dcmpipe_lib::core::read::{ParseError, Parser, ParserBuilder};
use dcmpipe_lib::defn::dcmdict::DicomDictionary;
use dcmpipe_lib::defn::tag::Tag;

pub(crate) mod archiveapp;
pub(crate) mod editapp;
pub(crate) mod indexapp;
pub(crate) mod printapp;
pub(crate) mod scanapp;

static MAX_ITEMS_DISPLAYED: usize = 16;

pub(crate) trait CommandApplication {
    fn run(&mut self) -> Result<()>;
}

fn parse_file(path: &Path, allow_partial_object: bool) -> Result<Parser<'_, File>> {
    if !path.is_file() {
        return Err(anyhow!("invalid file: {}", path.display()));
    }

    let file: File = File::open(path)?;
    let mut parser: Parser<'_, File> = ParserBuilder::default()
        .allow_partial_object(allow_partial_object)
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

    let mut peeker: Peekable<&mut Parser<'_, File>> = parser.by_ref().peekable();

    let first: Option<&Result<DicomElement, ParseError>> = peeker.peek();
    if let Some(Err(_)) = first {
        return Err(anyhow!("file is not dicom: {}", path.display()));
    } else if first.is_none() {
        return Err(anyhow!("file is empty: {}", path.display()));
    }

    Ok(parser)
}

/// Returns the name of the element, or an appropriate descriptive name if not known.
pub fn render_tag_name(element: &DicomElement) -> &str {
    if let Some(tag) = STANDARD_DICOM_DICTIONARY.get_tag_by_number(element.get_tag()) {
        tag.ident
    } else if Tag::is_private_creator(element.get_tag()) {
        "<PrivateCreator>"
    } else if Tag::is_private(element.get_tag()) && element.is_seq_like() {
        "<PrivateSequence>"
    } else if Tag::is_private_group_length(element.get_tag()) {
        "<PrivateGroupLength>"
    } else if Tag::is_private(element.get_tag()) {
        "<PrivateTag>"
    } else if Tag::is_group_length(element.get_tag()) {
        "<GroupLength>"
    } else {
        "<UnknownTag>"
    }
}

/// Formats the value of this element as a string based on the VR.
pub fn render_value(elem: &DicomElement) -> Result<String> {
    if elem.is_seq_like() {
        return Ok(String::new());
    }

    let (add_ellipses, mut str_vals) = match elem.parse_value()? {
        RawValue::Attribute(attr) => (false, vec![Tag::format_tag_to_display(attr.0)]),
        RawValue::Uid(uid_str) => {
            let uid_lookup = STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&uid_str);
            let uid_display = if uid_str.len() > 64 {
                String::from_utf8(uid_str.as_bytes()[0..64].to_vec())
                    .unwrap_or_else(|_| "<Unviewable>".to_string());
                format!("[>64bytes] {}", uid_str)
            } else {
                uid_str
            };
            if let Some(uid) = uid_lookup {
                let name = if let Some((name, _detail)) = uid.name.split_once(':') {
                    name
                } else {
                    uid.name
                };
                (false, vec![format!("{} => {}", uid_display, name)])
            } else {
                (false, vec![uid_display])
            }
        }
        RawValue::Strings(strings) => format_vec_to_strings(strings, |val: String| {
            if val.is_empty() {
                String::new()
            } else {
                val.replace("\r\n", " / ").replace('\n', " / ")
            }
        }),
        RawValue::Floats(floats) => format_vec_to_strings(floats, |val: f32| format!("{:.2}", val)),
        RawValue::Doubles(doubles) => {
            format_vec_to_strings(doubles, |val: f64| format!("{:.2}", val))
        }
        RawValue::Shorts(shorts) => format_vec_to_strings(shorts, |val: i16| format!("{}", val)),
        RawValue::UnsignedShorts(ushorts) => {
            format_vec_to_strings(ushorts, |val: u16| format!("{}", val))
        }
        RawValue::Integers(ints) => format_vec_to_strings(ints, |val: i32| format!("{}", val)),
        RawValue::UnsignedIntegers(uints) => {
            format_vec_to_strings(uints, |val: u32| format!("{}", val))
        }
        RawValue::Bytes(bytes) => format_vec_to_strings(bytes, |val: u8| format!("{:02x}", val)),
    };

    if add_ellipses {
        str_vals.push("..".to_string());
    }

    let num_vals: usize = str_vals.len();
    if num_vals == 1 {
        return Ok(str_vals.remove(0));
    }

    let sep: &str = "\\";
    Ok(str_vals
        .into_iter()
        .collect::<Vec<String>>()
        .join(sep)
        .to_string())
}

/// Formats `vec` converting each element to a String based on the given `func`.
/// Returns true if the input `vec` had more items than rendered, based on `MAX_ITEMS_DISPLAYED`.
fn format_vec_to_strings<T, F: Fn(T) -> String>(vec: Vec<T>, func: F) -> (bool, Vec<String>) {
    let vec_len: usize = vec.len();
    let formatted: Vec<String> = vec
        .into_iter()
        .take(MAX_ITEMS_DISPLAYED)
        .map(func)
        .collect::<Vec<String>>();
    (formatted.len() < vec_len, formatted)
}
