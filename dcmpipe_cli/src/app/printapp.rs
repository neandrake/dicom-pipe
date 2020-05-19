use std::fs::File;
use std::io::{self, StdoutLock, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;

use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;
use dcmpipe_dict::dict::tags;
use dcmpipe_lib::core::dcmelement::{DicomElement, RawValue};
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::dcmparser::Parser;
use dcmpipe_lib::core::dcmparser_util::parse_into_object;
use dcmpipe_lib::core::dcmsqelem::SequenceElement;
use dcmpipe_lib::defn::dcmdict::DicomDictionary;
use dcmpipe_lib::defn::tag::Tag;
use dcmpipe_lib::defn::ts::TSRef;

use crate::app::{parse_file, CommandApplication};

static HIDE_GROUP_TAGS: bool = false;
static HIDE_DELIMITATION_TAGS: bool = false;
static MAX_ITEMS_DISPLAYED: usize = 16;

pub struct PrintApp {
    stream: bool,
    file: PathBuf,
}

impl PrintApp {
    pub fn new(stream: bool, file: PathBuf) -> PrintApp {
        PrintApp { stream, file }
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

            if prev_was_file_meta && elem.tag > 0x0002_FFFF {
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

    fn render_root(
        &mut self,
        mut parser: Parser<'_, File>,
        mut stdout: &mut StdoutLock<'_>,
    ) -> Result<()> {
        let dcmroot: DicomRoot<'_> =
            parse_into_object(&mut parser)?.expect("Failed to parse any dicom elements");
        self.render_objects(&dcmroot, true, parser.get_ts(), &mut stdout)
    }

    fn render_objects(
        &mut self,
        dcmnode: &impl DicomNode,
        mut prev_was_file_meta: bool,
        ts: TSRef,
        stdout: &mut StdoutLock<'_>,
    ) -> Result<()> {
        for (tag, obj) in dcmnode.iter_child_nodes() {
            let elem: &DicomElement = obj.get_element();

            if prev_was_file_meta && *tag > 0x0002_FFFF {
                stdout.write_all(
                    format!(
                        "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                        ts.uid.ident
                    )
                    .as_ref(),
                )?;
                prev_was_file_meta = false;
            }

            let printed: Option<String> = render_element(&elem)?;
            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }

            // display items first followed by children. the only situation where an element
            // may have both items and children is a sequence with items whose only child is
            // its ending sequence delimiter.
            for index in 0..obj.get_item_count() {
                let child_obj: &DicomObject = obj.get_item_by_index(index + 1).unwrap();
                let child_elem: &DicomElement = child_obj.get_element();
                if let Some(printed) = render_element(child_elem)? {
                    stdout.write_all(format!("{}\n", printed).as_ref())?;
                }
                self.render_objects(child_obj, prev_was_file_meta, ts, stdout)?;
            }
            if obj.get_child_count() > 0 {
                self.render_objects(obj, prev_was_file_meta, ts, stdout)?;
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

        if self.stream {
            self.render_stream(parser, &mut stdout)?;
        } else {
            self.render_root(parser, &mut stdout)?;
        }

        Ok(())
    }
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
fn render_element(element: &DicomElement) -> Result<Option<String>> {
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
    if indent_width > 0
        && element.tag != tags::SequenceDelimitationItem.tag
        && element.tag != tags::ItemDelimitationItem.tag
        && element.tag != tags::Item.tag
    {
        indent_width += 1;
    }
    indent_width *= 2;

    if element.tag == tags::Item.tag {
        let item_desc: String = if let Some(last_seq_elem) = seq_path.last() {
            format!(
                "{} {} [{:?}]",
                last_seq_elem
                    .get_item_number()
                    .map(|item_no: usize| format!("#{}", item_no))
                    .unwrap_or_else(|| "#[NO ITEM NUMBER]".to_string()),
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
pub fn render_value(elem: &DicomElement) -> Result<String> {
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
        RawValue::Uid(mut uid_str) => {
            if uid_str.len() > 64 {
                uid_str = String::from_utf8(uid_str.as_bytes()[0..64].to_vec())
                    .unwrap_or_else(|_| "<Unviewable>".to_string());
                uid_str = format!("[>64bytes] {}", uid_str);
            }
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
                    let formatted: String = val.replace("\r\n", " / ").replace("\n", " / ");
                    format!("\"{}\"", formatted)
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
            ellipses =
                format_vec_to_strings(bytes, &mut str_vals, |val: u8| format!("{:02x}", val));
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
