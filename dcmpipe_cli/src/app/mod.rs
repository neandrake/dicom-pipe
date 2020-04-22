use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_dict::dict::tags;
use dcmpipe_lib::core::dcmelement::{DicomElement, RawValue};
use dcmpipe_lib::core::dcmsqelem::SequenceElement;
use dcmpipe_lib::defn::dcmdict::DicomDictionary;
use dcmpipe_lib::defn::tag::Tag;

use dcmpipe_lib::core::dcmparser::{Parser, ParserBuilder};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::iter::Peekable;
use std::path::Path;

pub(crate) mod args;
pub(crate) mod cursiveapp;
pub(crate) mod fullobjapp;
pub(crate) mod lowmemapp;
pub(crate) mod scanapp;

static MAX_ITEMS_DISPLAYED: usize = 16;

static HIDE_GROUP_TAGS: bool = false;
static HIDE_DELIMITATION_TAGS: bool = false;

pub(crate) trait CommandApplication {
    fn run(&mut self) -> Result<(), Error>;
}

fn parse_file(path: &Path) -> Result<Parser<'_, File>, Error> {
    if !path.is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("invalid file: {}", path.display()),
        ));
    }

    let file: File = File::open(path)?;
    let mut parser: Parser<'_, File> = ParserBuilder::default()
        .dictionary(&STANDARD_DICOM_DICTIONARY)
        .build(file);

    let mut peeker: Peekable<&mut Parser<'_, File>> = parser.by_ref().peekable();

    let first: Option<&Result<DicomElement, Error>> = peeker.peek();
    if let Some(Err(_)) = first {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("file is not dicom: {}", path.display()),
        ));
    } else if first.is_none() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("file is empty: {}", path.display()),
        ));
    }

    Ok(parser)
}

/// Renders an element on a single line, includes indentation based on depth in sequences
/// ```
/// (gggg,eeee) VR TagName | TagValue
/// ```
/// or
/// ```
/// (gggg,eeee) VR TagName <empty>
/// ```
/// Names for unknown tags will render as `<UnknownTag>`
fn render_element(element: &DicomElement) -> Result<Option<String>, Error> {
    // Group Length tags are deprecated, see note on Part 5 Section 7.2
    if HIDE_GROUP_TAGS && element.tag.trailing_zeros() >= 16 {
        return Ok(None);
    }

    // These are delimiter items that are not very useful to see
    if HIDE_DELIMITATION_TAGS
        && (element.tag == tags::ItemDelimitationItem.tag
            || element.tag == tags::SequenceDelimitationItem.tag)
    {
        return Ok(None);
    }

    let tag_num: String = Tag::format_tag_to_display(element.tag);
    let tag_name: &str = if let Some(tag) = STANDARD_DICOM_DICTIONARY.get_tag_by_number(element.tag)
    {
        tag.ident
    } else {
        "<Unknown Tag>"
    };
    let vr: &str = element.vr.ident;

    let seq_path: &Vec<SequenceElement> = element.get_sequence_path();

    let mut indent_width: usize = seq_path.len();
    if indent_width > 0 {
        if element.tag == tags::SequenceDelimitationItem.tag
            || element.tag == tags::ItemDelimitationItem.tag
        {
            indent_width -= 1;
        } else if element.tag != tags::Item.tag {
            indent_width += 1;
        }
    }
    indent_width *= 2;

    if element.tag == tags::Item.tag {
        let path: String = seq_path
            .iter()
            .map(|seq_elem: &SequenceElement| {
                format!(
                    "{}{}",
                    Tag::format_tag_to_display(seq_elem.get_seq_tag()),
                    seq_elem
                        .get_item_number()
                        .map(|item_no: u32| format!("[{}]", item_no))
                        .unwrap_or_else(|| "".to_string())
                )
            })
            .collect::<Vec<String>>()
            .join(".");
        let item_desc: String = if let Some(last_seq_elem) = seq_path.last() {
            format!(
                "{} - {} {} [{:?}]",
                last_seq_elem
                    .get_item_number()
                    .map(|item_no: u32| format!("#{}", item_no))
                    .unwrap_or_else(|| "#[NO ITEM NUMBER]".to_string()),
                path,
                vr,
                element.vl,
            )
        } else {
            String::new()
        };
        return Ok(Some(format!(
            "{indentation:indent_width$}{tag_name} {item_desc}",
            indentation = "",
            indent_width = indent_width,
            tag_name = tag_name,
            item_desc = item_desc,
        )));
    }

    let mut tag_value: String = if element.is_seq_like() {
        String::new()
    } else if element.is_empty() {
        "<empty>".to_owned()
    } else {
        render_value(element)?
    };

    if !tag_value.is_empty() {
        if element.is_empty() {
            tag_value = format!(" {}", tag_value);
        } else {
            tag_value = format!(" | {}", tag_value);
        }
    }

    Ok(Some(format!(
        "{indentation:indent_width$}{tag_num} {vr} {tag_name} [{vl:?}]{tag_value}",
        indentation = "",
        indent_width = indent_width,
        tag_num = tag_num,
        vr = vr,
        tag_name = tag_name,
        vl = element.vl,
        tag_value = tag_value,
    )))
}

/// Formats the value of this element as a string based on the VR
fn render_value(elem: &DicomElement) -> Result<String, Error> {
    if elem.is_seq_like() {
        return Ok(String::new());
    }

    let mut ellipses: bool = false;
    let mut sep: &str = ", ";
    let mut str_vals: Vec<String> = Vec::new();

    match elem.parse_value()? {
        RawValue::Attribute(attr) => {
            str_vals.push(Tag::format_tag_to_display(attr.0));
        }
        RawValue::Uid(uid_str) => {
            if let Some(uid) = STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&uid_str) {
                str_vals.push(format!("{} ({})", uid_str, uid.name));
            } else {
                str_vals.push(uid_str);
            }
        }
        RawValue::Strings(strings) => {
            ellipses = format_vec_to_strings(strings, &mut str_vals, |val: String| {
                if val.is_empty() {
                    String::new()
                } else {
                    format!("\"{}\"", val)
                }
            });
        }
        RawValue::Doubles(doubles) => {
            sep = " / ";
            ellipses =
                format_vec_to_strings(doubles, &mut str_vals, |val: f64| format!("{:.2}", val));
        }
        RawValue::Shorts(shorts) => {
            sep = " / ";
            ellipses = format_vec_to_strings(shorts, &mut str_vals, |val: i16| format!("{}", val));
        }
        RawValue::Integers(ints) => {
            sep = " / ";
            ellipses = format_vec_to_strings(ints, &mut str_vals, |val: i32| format!("{}", val));
        }
        RawValue::UnsignedIntegers(uints) => {
            sep = " / ";
            ellipses = format_vec_to_strings(uints, &mut str_vals, |val: u32| format!("{}", val));
        }
        RawValue::Bytes(bytes) => {
            ellipses = format_vec_to_strings(bytes, &mut str_vals, |val: u8| format!("{}", val));
        }
    }

    if ellipses {
        str_vals.push("..".to_string());
    }

    let num_vals: usize = str_vals.len();
    if num_vals == 1 {
        return Ok(str_vals.remove(0));
    }

    Ok(format!(
        "[{}]",
        str_vals.into_iter().collect::<Vec<String>>().join(sep)
    ))
}

fn format_vec_to_strings<T, F: Fn(T) -> String>(
    vec: Vec<T>,
    str_vals: &mut Vec<String>,
    func: F,
) -> bool {
    let vec_len: usize = vec.len();
    vec.into_iter()
        .take(MAX_ITEMS_DISPLAYED)
        .map(func)
        .for_each(|val: String| str_vals.push(val));
    vec_len > str_vals.len()
}
