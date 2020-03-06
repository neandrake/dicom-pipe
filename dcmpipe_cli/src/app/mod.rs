use dcmpipe_dict::dict::dicom_elements as tags;
use dcmpipe_dict::dict::lookup::{TAG_BY_VALUE, UID_BY_UID};
use dcmpipe_lib::core::dcmelement::{Attribute, DicomElement};
use dcmpipe_lib::defn::tag::Tag;
use dcmpipe_lib::defn::vr;
use std::io::Error;

mod cursiveapp;
mod fullobjapp;
mod lowmemapp;

pub use cursiveapp::CursiveApp;
use dcmpipe_lib::core::dcmsqelem::SequenceElement;
use dcmpipe_lib::defn::vl::ValueLength;
pub use fullobjapp::FullObjApp;
pub use lowmemapp::LowMemApp;
use std::convert::TryFrom;

static MAX_BYTES_DISPLAY: usize = 16;
static MAX_ITEMS_DISPLAYED: usize = 16;

static HIDE_GROUP_TAGS: bool = false;
static HIDE_DELIMITATION_TAGS: bool = false;

/// Renders an element on a single line, includes indentation based on depth in sequences
/// ```
/// (gggg,eeee) VR TagName | TagValue
/// ```
/// or
/// ```
/// (gggg,eeee) VR TagName <empty>
/// ```
/// Names for private tags will render as `<PrivateTag>`
fn render_element(element: &DicomElement) -> Result<Option<String>, Error> {
    // Group Length tags are deprecated, see note on Part 5 Section 7.2
    if HIDE_GROUP_TAGS {
        if element.tag.trailing_zeros() >= 16 {
            return Ok(None);
        }
    }

    // These are delimiter items that are not very useful to see
    if HIDE_DELIMITATION_TAGS {
        if element.tag == tags::ItemDelimitationItem.tag
            || element.tag == tags::SequenceDelimitationItem.tag
        {
            return Ok(None);
        }
    }

    let tag_num: String = Tag::format_tag_to_display(element.tag);
    let tag_name: &str = if let Some(tag) = TAG_BY_VALUE.get(&element.tag) {
        tag.ident
    } else {
        "<Private Tag>"
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
    if elem.vr == &vr::AT {
        str_vals.push(Tag::format_tag_to_display(Attribute::try_from(elem)?.0));
    } else if elem.vr == &vr::FD || elem.vr == &vr::OF || elem.vr == &vr::OD || elem.vr == &vr::FL {
        sep = " / ";
        let vec: Vec<f64> = match elem.vl {
            ValueLength::Explicit(len)
                if (elem.vr == &vr::OD || elem.vr == &vr::FL) && len > 0 && len % 8 == 0 =>
            {
                Vec::<f64>::try_from(elem)?
            }
            ValueLength::Explicit(len) if len > 0 && len % 4 == 0 => Vec::<f32>::try_from(elem)?
                .into_iter()
                .map(f64::from)
                .collect::<Vec<f64>>(),
            ValueLength::Explicit(1) => vec![f64::from(elem.get_data()[0])],
            _ => vec![],
        };
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: f64| format!("{:.2}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::SS {
        sep = " / ";
        let vec: Vec<i16> = match elem.vl {
            ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => Vec::<i16>::try_from(elem)?,
            ValueLength::Explicit(1) => vec![i16::from(elem.get_data()[0])],
            _ => vec![],
        };
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: i16| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::SL {
        sep = " / ";
        let vec: Vec<i32> = match elem.vl {
            ValueLength::Explicit(len) if len > 0 && len % 4 == 0 => Vec::<i32>::try_from(elem)?,
            ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => Vec::<i16>::try_from(elem)?
                .into_iter()
                .map(i32::from)
                .collect::<Vec<i32>>(),
            ValueLength::Explicit(1) => vec![i32::from(elem.get_data()[0])],
            _ => vec![],
        };
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: i32| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::UI {
        let str_val: String = String::try_from(elem)?;
        if let Some(uid) = UID_BY_UID.get(str_val.as_str()) {
            str_vals.push(format!("{} ({})", str_val, uid.name));
        } else {
            str_vals.push(str_val);
        }
    } else if elem.vr == &vr::UL || elem.vr == &vr::OL || elem.vr == &vr::OW || elem.vr == &vr::US {
        sep = " / ";
        let vec: Vec<u32> = match elem.vl {
            ValueLength::Explicit(len)
                if (elem.vr == &vr::UL || elem.vr == &vr::OL) && len > 0 && len % 4 == 0 =>
            {
                Vec::<u32>::try_from(elem)?
            }
            ValueLength::Explicit(len) if len > 0 && len % 2 == 0 => Vec::<u16>::try_from(elem)?
                .into_iter()
                .map(u32::from)
                .collect::<Vec<u32>>(),
            ValueLength::Explicit(1) => vec![u32::from(elem.get_data()[0])],
            _ => vec![],
        };
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: u32| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr.is_character_string {
        let vec: Vec<String> = Vec::<String>::try_from(elem)?;
        let vec_len: usize = vec.len();
        vec.iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: &String| {
                if val.is_empty() {
                    String::new()
                } else {
                    format!("\"{}\"", val)
                }
            })
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else {
        let vec: &Vec<u8> = elem.get_data();
        vec.iter()
            .take(MAX_BYTES_DISPLAY)
            .map(|val: &u8| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec.len() > str_vals.len();
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
