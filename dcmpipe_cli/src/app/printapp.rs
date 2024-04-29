//! The print command renders the contents of a DICOM dataset to stdout, in a format similar to the
//! dcmdump tool.

use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;

use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmsqelem::SequenceElement;
use dcmpipe_lib::core::read::Parser;
use dcmpipe_lib::defn::constants::tags::FILE_META_GROUP_END;
use dcmpipe_lib::defn::tag::Tag;
use dcmpipe_lib::defn::ts::TSRef;
use dcmpipe_lib::defn::vl::ValueLength;
use dcmpipe_lib::defn::vr;
use dcmpipe_lib::dict::tags;

use crate::app::{parse_file, CommandApplication};
use crate::args::PrintArgs;

use super::{ElementWithLineFmt, TagCategory, TagValue};

static HIDE_GROUP_TAGS: bool = false;
static HIDE_DELIMITATION_TAGS: bool = false;

pub struct PrintApp {
    args: PrintArgs,
}

impl PrintApp {
    pub fn new(args: PrintArgs) -> PrintApp {
        PrintApp { args }
    }
}

impl CommandApplication for PrintApp {
    fn run(&mut self) -> Result<()> {
        let path_buf: PathBuf = self.args.file.clone();
        let path: &Path = path_buf.as_path();
        let mut parser: Parser<'_, File> = parse_file(path, true)?;

        let mut stdout = io::stdout().lock();
        stdout.write_all(format!(
            "\n# Dicom-File-Format File: {:#?}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
            path,
            parser.get_ts().uid.ident).as_ref()
        )?;

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

            let printed: Option<String> = render_element(parser.get_ts(), &elem)?;

            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }
        }

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
fn render_element(ts: TSRef, element: &DicomElement) -> Result<Option<String>> {
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
        && ((ts.is_explicit_vr() && element.get_vr() == &vr::INVALID)
            || (!ts.is_explicit_vr() && element.get_vr() == &vr::UN))
        && element.get_vl() == ValueLength::Explicit(0)
    {
        return Ok(None);
    }

    let tag_num: String = Tag::format_tag_to_display(element.get_tag());
    let tag_name: TagCategory = element.into();
    let vr: &str = element.get_vr().ident;

    let vl: String = match element.get_vl() {
        ValueLength::Explicit(len) => {
            if len % 2 != 0 {
                format!("[*{:?}]", element.get_vl())
            } else {
                format!("[{:?}]", element.get_vl())
            }
        }
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

    let tag_value: TagValue = ElementWithLineFmt(element, false).into();
    let mut tag_value: String = tag_value.to_string();
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
