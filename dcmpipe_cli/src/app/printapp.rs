//! The print command renders the contents of a DICOM dataset to stdout, in a format similar to the
//! dcmdump tool.

use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;

use dcmpipe_lib::{
    core::{
        dcmelement::DicomElement,
        dcmsqelem::SequenceElement,
        defn::{constants::tags::FILE_META_GROUP_END, tag::Tag, ts::TSRef, vl::ValueLength, vr},
        read::Parser,
    },
    dict::tags,
};

use crate::{
    app::{parse_file, CommandApplication},
    args::PrintArgs,
};

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
            parser.ts().uid.ident).as_ref()
        )?;

        let mut prev_was_file_meta: bool = true;

        while let Some(elem) = parser.next() {
            let elem: DicomElement = elem?;

            if prev_was_file_meta && elem.tag() > FILE_META_GROUP_END {
                stdout.write_all(
                    format!(
                        "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                        parser.ts().uid.ident
                    )
                    .as_ref(),
                )?;
                prev_was_file_meta = false;
            }

            let printed: Option<String> = render_element(parser.ts(), &elem)?;

            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }
        }

        Ok(())
    }
}

/// Renders an element on a single line, includes indentation based on depth in sequences.
/// ```
/// (gggg,eeee) VR TagName [VL] | TagValue
/// ```
/// or
/// ```
/// (gggg,eeee) VR TagName [0] <empty>
/// ```
fn render_element(ts: TSRef, element: &DicomElement) -> Result<Option<String>> {
    // Group Length tags are deprecated, see note on Part 5 Section 7.2
    if HIDE_GROUP_TAGS && element.tag().trailing_zeros() >= 16 {
        return Ok(None);
    }

    // These are delimiter items that are not very useful to see
    if HIDE_DELIMITATION_TAGS
        && (element.tag() == tags::ItemDelimitationItem.tag
            || element.tag() == tags::SequenceDelimitationItem.tag)
    {
        return Ok(None);
    }

    // Some (malformed?) datasets have a bunch of zeroes between elements.
    if element.tag() == 0
        && ((ts.explicit_vr() && element.vr() == &vr::INVALID)
            || (!ts.explicit_vr() && element.vr() == &vr::UN))
        && element.vl() == ValueLength::Explicit(0)
    {
        return Ok(None);
    }

    let tag_num: String = Tag::format_tag_to_display(element.tag());
    let tag_name: TagCategory = element.into();
    let vr: &str = element.vr().ident;

    let vl: String = match element.vl() {
        ValueLength::Explicit(len) => {
            if len % 2 != 0 {
                format!("[*{:?}]", element.vl())
            } else {
                format!("[{:?}]", element.vl())
            }
        }
        ValueLength::UndefinedLength => "[u/l]".to_string(),
    };

    // Sequence path will nest tags under ITEM elements. Double the indentation level for the
    // number of nested sequences (non-ITEM), and each ITEM element should be nested one level.
    // If the current element is a delimiter then reduce the associated indentation level.
    let seq_path: &Vec<SequenceElement> = element.sequence_path();
    let non_item_parents = seq_path
        .iter()
        .filter(|sq_el| sq_el.seq_tag() != tags::Item.tag)
        .count();
    let item_parents = seq_path
        .iter()
        .filter(|sq_el| sq_el.seq_tag() == tags::Item.tag)
        .count();
    let mut indent_width = non_item_parents * 2 + item_parents;
    if element.tag() == tags::ItemDelimitationItem.tag {
        indent_width -= 1;
    } else if element.tag() == tags::SequenceDelimitationItem.tag {
        indent_width -= 2;
    }

    if element.tag() == tags::Item.tag {
        let item_desc: String = if let Some(last_seq_elem) = seq_path.last() {
            format!(
                "{} {} {}",
                last_seq_elem
                    .item()
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
