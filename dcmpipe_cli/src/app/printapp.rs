use std::fs::File;
use std::io::{self, StdoutLock, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_dict::dict::tags;
use dcmpipe_lib::core::dcmelement::{DicomElement, RawValue};
use dcmpipe_lib::core::dcmsqelem::SequenceElement;
use dcmpipe_lib::core::read::Parser;
use dcmpipe_lib::defn::constants::tags::FILE_META_GROUP_END;
use dcmpipe_lib::defn::dcmdict::DicomDictionary;
use dcmpipe_lib::defn::tag::Tag;
use dcmpipe_lib::defn::vl::ValueLength;
use dcmpipe_lib::defn::vr;

use crate::app::{parse_file, CommandApplication};

static HIDE_GROUP_TAGS: bool = false;
static HIDE_DELIMITATION_TAGS: bool = false;
static MAX_ITEMS_DISPLAYED: usize = 16;

pub struct PrintApp {
    file: PathBuf,
}

impl PrintApp {
    pub fn new(file: PathBuf) -> PrintApp {
        PrintApp { file }
    }
}

impl PrintApp {
    fn render_stream(
        &mut self,
        mut parser: Parser<'_, File>,
        stdout: &mut StdoutLock<'_>,
    ) -> Result<()> {
        let mut prev_was_file_meta: bool = true;

        while let Some(elem) = parser.next() {
            let elem: DicomElement = elem?;

            if prev_was_file_meta && elem.get_tag() > FILE_META_GROUP_END {
                stdout.write_all(
                    format!(
                        "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                        parser.get_ts().uid.ident
                    )
                    .as_ref(),
                )?;
                prev_was_file_meta = false;
            }

            let printed: Option<String> = render_element(&elem)?;

            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }
        }
        Ok(())
    }
}

impl CommandApplication for PrintApp {
    fn run(&mut self) -> Result<()> {
        let path_buf: PathBuf = self.file.clone();
        let path: &Path = path_buf.as_path();
        let parser: Parser<'_, File> = parse_file(path)?;

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(format!(
            "\n# Dicom-File-Format File: {:#?}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
            path,
            parser.get_ts().uid.ident).as_ref()
        )?;

        self.render_stream(parser, &mut stdout)?;

        Ok(())
    }
}

/// Renders an element on a single line, includes indentation based on depth in sequences
/// ```
/// (gggg,eeee) VR TagName [VL] | TagValue
/// ```
/// or
/// ```
/// (gggg,eeee) VR TagName [0] <empty>
/// ```
/// Names for unknown tags will render as `<UnknownTag>`
fn render_element(element: &DicomElement) -> Result<Option<String>> {
    // Group Length tags are deprecated, see note on Part 5 Section 7.2
    if HIDE_GROUP_TAGS && element.get_tag().trailing_zeros() >= 16 {
        return Ok(None);
    }

    // These are delimiter items that are not very useful to see
    if HIDE_DELIMITATION_TAGS
        && (element.get_tag() == tags::ItemDelimitationItem.tag
            || element.get_tag() == tags::SequenceDelimitationItem.tag)
    {
        return Ok(None);
    }

    // Some (malformed?) datasets have a bunch of zeroes between elements.
    if element.get_tag() == 0
        && element.get_vr() == &vr::INVALID
        && element.get_vl() == ValueLength::Explicit(0)
    {
        return Ok(None);
    }

    let tag_num: String = Tag::format_tag_to_display(element.get_tag());
    let tag_name: &str =
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
        };

    let vr: &str = element.get_vr().ident;

    let vl: String = match element.get_vl() {
        ValueLength::Explicit(_) => format!("[{:?}]", element.get_vl()),
        ValueLength::UndefinedLength => "[u/l]".to_string(),
    };

    // Sequence path will nest tags under ITEM elements. Double the indentation level for the
    // number of nested sequences (non-ITEM), and each ITEM element should be nested one level.
    // If the current element is a delimiter then reduce the associated indentation level.
    let seq_path: &Vec<SequenceElement> = element.get_sequence_path();
    let non_item_parents = seq_path
        .iter()
        .filter(|sq_el| sq_el.get_seq_tag() != tags::Item.tag)
        .count();
    let item_parents = seq_path
        .iter()
        .filter(|sq_el| sq_el.get_seq_tag() == tags::Item.tag)
        .count();
    let mut indent_width = non_item_parents * 2 + item_parents;
    if element.get_tag() == tags::ItemDelimitationItem.tag {
        indent_width -= 1;
    } else if element.get_tag() == tags::SequenceDelimitationItem.tag {
        indent_width -= 2;
    }

    if element.get_tag() == tags::Item.tag {
        let item_desc: String = if let Some(last_seq_elem) = seq_path.last() {
            format!(
                "{} {} {}",
                last_seq_elem
                    .get_item_number()
                    .map(|item_no: usize| format!("#{}", item_no))
                    .unwrap_or_else(|| "#[NO ITEM NUMBER]".to_string()),
                vr,
                vl,
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
        "{indentation:indent_width$}{tag_num} {vr} {tag_name} {vl}{tag_value}",
        indentation = "",
        indent_width = indent_width,
        tag_num = tag_num,
        vr = vr,
        tag_name = tag_name,
        vl = vl,
        tag_value = tag_value,
    )))
}

/// Formats the value of this element as a string based on the VR
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
    Ok(format!(
        "{}",
        str_vals.into_iter().collect::<Vec<String>>().join(sep)
    ))
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
