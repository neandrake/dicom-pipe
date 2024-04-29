use std::fmt;
use std::fs::File;
use std::iter::Peekable;
use std::path::Path;

use anyhow::{anyhow, Result};
use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::read::{ParseError, Parser, ParserBuilder};
use dcmpipe_lib::core::RawValue;
use dcmpipe_lib::defn::dcmdict::DicomDictionary;
use dcmpipe_lib::defn::tag::Tag;

pub(crate) mod archiveapp;
pub(crate) mod browseapp;
#[cfg(feature = "index")]
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

pub(crate) enum TagCategory {
    Known(u32, String),
    PrivateCreator(u32),
    PrivateSequence(u32),
    PrivateGroupLength(u32),
    Private(u32),
    GroupLength(u32),
    Unknown(u32),
}

impl From<&DicomElement> for TagCategory {
    fn from(element: &DicomElement) -> Self {
        if let Some(tag) = STANDARD_DICOM_DICTIONARY.get_tag_by_number(element.get_tag()) {
            TagCategory::Known(tag.tag, tag.ident.to_string())
        } else if Tag::is_private_creator(element.get_tag()) {
            TagCategory::PrivateCreator(element.get_tag())
        } else if Tag::is_private(element.get_tag()) && element.is_seq_like() {
            TagCategory::PrivateSequence(element.get_tag())
        } else if Tag::is_private_group_length(element.get_tag()) {
            TagCategory::PrivateGroupLength(element.get_tag())
        } else if Tag::is_private(element.get_tag()) {
            TagCategory::Private(element.get_tag())
        } else if Tag::is_group_length(element.get_tag()) {
            TagCategory::GroupLength(element.get_tag())
        } else {
            TagCategory::Unknown(element.get_tag())
        }
    }
}

impl fmt::Display for TagCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagCategory::Known(_, display) => write!(f, "{}", display),
            TagCategory::PrivateCreator(_) => write!(f, "<PrivateCreator>"),
            TagCategory::PrivateSequence(_) => write!(f, "<PrivateSequence>"),
            TagCategory::PrivateGroupLength(_) => write!(f, "<PrivateGroupLength>"),
            TagCategory::Private(_) => write!(f, "<PrivateTag>"),
            TagCategory::GroupLength(_) => write!(f, "<GroupLength>"),
            TagCategory::Unknown(_) => write!(f, "<UnknownTag>"),
        }
    }
}

pub(crate) struct ElementWithLineFmt<'elem>(pub &'elem DicomElement, pub bool);

pub(crate) enum TagValue {
    Sequence,
    Error(String),
    Uid(String, String),
    Stringified(String),
}

impl fmt::Display for TagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagValue::Sequence => Ok(()),
            TagValue::Error(error_str) => write!(f, "<Error {}>", error_str),
            TagValue::Uid(uid, name) => {
                if name.is_empty() {
                    write!(f, "{}", uid)
                } else {
                    write!(f, "{} => {}", uid, name)
                }
            }
            TagValue::Stringified(value) => write!(f, "{}", value),
        }
    }
}

impl<'elem> From<ElementWithLineFmt<'elem>> for TagValue {
    fn from(value: ElementWithLineFmt<'elem>) -> Self {
        let elem = value.0;
        let multiline = value.1;

        if elem.is_seq_like() {
            return TagValue::Sequence;
        }

        let mut sep = if multiline { " " } else { "\\" };
        let elem_value = match elem.parse_value() {
            Err(e) => return TagValue::Error(e.to_string()),
            Ok(val) => val,
        };

        let (add_ellipses, mut str_vals) = match elem_value {
            RawValue::Attribute(attrs) => {
                format_vec_to_strings(attrs, |attr| Tag::format_tag_to_display(attr.0))
            }
            RawValue::Uid(uid_str) => {
                let uid_lookup = STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&uid_str);
                match uid_lookup {
                    Some(found_uid) => {
                        let uid_name = if let Some((name, _detail)) = found_uid.name.split_once(':')
                        {
                            name
                        } else {
                            found_uid.name
                        };
                        return TagValue::Uid(uid_str.to_string(), uid_name.to_string());
                    }
                    None => return TagValue::Uid(uid_str.to_string(), String::new()),
                }
            }
            RawValue::Strings(strings) => {
                if multiline {
                    sep = "\n";
                }
                format_vec_to_strings(strings, |val: String| {
                    if !multiline {
                        val.replace("\r\n", " / ").replace('\n', " / ")
                    } else {
                        val
                    }
                })
            }
            RawValue::Shorts(shorts) => {
                format_vec_to_strings(shorts, |val: i16| format!("{}", val))
            }
            RawValue::UnsignedShorts(ushorts) => {
                format_vec_to_strings(ushorts, |val: u16| format!("{}", val))
            }
            RawValue::Integers(ints) => format_vec_to_strings(ints, |val: i32| format!("{}", val)),
            RawValue::UnsignedIntegers(uints) => {
                format_vec_to_strings(uints, |val: u32| format!("{}", val))
            }
            RawValue::Longs(longs) => format_vec_to_strings(longs, |val: i64| format!("{}", val)),
            RawValue::UnsignedLongs(ulongs) => {
                format_vec_to_strings(ulongs, |val: u64| format!("{}", val))
            }
            RawValue::Floats(floats) => {
                format_vec_to_strings(floats, |val: f32| format!("{:.2}", val))
            }
            RawValue::Doubles(doubles) => {
                format_vec_to_strings(doubles, |val: f64| format!("{:.2}", val))
            }
            RawValue::Bytes(bytes) => {
                format_vec_to_strings(bytes, |val: u8| format!("{:02x}", val))
            }
            RawValue::Words(words) => {
                format_vec_to_strings(words, |val: u16| format!("{:04x}", val))
            }
            RawValue::DoubleWords(dwords) => {
                format_vec_to_strings(dwords, |val: u32| format!("{:06x}", val))
            }
            RawValue::QuadWords(qwords) => {
                format_vec_to_strings(qwords, |val: u64| format!("{:08x}", val))
            }
        };

        if add_ellipses {
            str_vals.push("..".to_string());
        }

        let num_vals: usize = str_vals.len();
        if num_vals == 1 {
            return TagValue::Stringified(str_vals.remove(0));
        }

        TagValue::Stringified(str_vals.join(sep).to_string())
    }
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
