use dcmpipe_dict::dict::dicom_elements as tags;
use dcmpipe_dict::dict::lookup::{TAG_BY_VALUE, UID_BY_UID};
use dcmpipe_lib::core::dcmelement::{DicomElement, SequenceElement};
use dcmpipe_lib::defn::tag::Tag;
use dcmpipe_lib::defn::vr;
use std::io::Error;

mod cursiveapp;
mod fullobjapp;
mod lowmemapp;

pub use cursiveapp::CursiveApp;
pub use fullobjapp::FullObjApp;
pub use lowmemapp::LowMemApp;

static MAX_BYTES_DISPLAY: usize = 16;
static MAX_ITEMS_DISPLAYED: usize = 16;

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
    if element.tag.trailing_zeros() >= 16 {
        // Group Length tags are deprecated, see note on Part 5 Section 7.2
        return Ok(None);
    }
    if element.tag == tags::ItemDelimitationItem.tag
        || element.tag == tags::SequenceDelimitationItem.tag
    {
        // These are delimiter items that are not very useful to see
        return Ok(None);
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
    if indent_width > 0 && element.tag != tags::Item.tag {
        indent_width += 1;
    }
    indent_width *= 2;

    if element.tag == tags::Item.tag {
        let path: String = seq_path
            .iter()
            .map(|seq_elem: &SequenceElement| {
                format!(
                    "{}[{}]",
                    Tag::format_tag_to_path_display(seq_elem.get_seq_tag()),
                    seq_elem.get_item_number().unwrap_or(0xFFFF_FFFF)
                )
            })
            .collect::<Vec<String>>()
            .join(".");
        let item_desc: String = if let Some(last_seq_elem) = seq_path.last() {
            format!(
                "#{} - {}",
                last_seq_elem.get_item_number().unwrap_or(0xFFFF_FFFF),
                path
            )
        } else {
            String::new()
        };
        return Ok(Some(format!(
            "{indentation:indent_width$}{tag_name} {item_desc}",
            indentation = "",
            indent_width = indent_width,
            tag_name = tag_name,
            item_desc = item_desc
        )));
    }

    let mut tag_value: String = if element.vr == &vr::SQ {
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
        "{indentation:indent_width$}{tag_num} {vr} {tag_name}{tag_value}",
        indentation = "",
        indent_width = indent_width,
        tag_num = tag_num,
        vr = vr,
        tag_name = tag_name,
        tag_value = tag_value
    )))
}

/// Formats the value of this element as a string based on the VR
fn render_value(elem: &DicomElement) -> Result<String, Error> {
    let mut ellipses: bool = false;
    let mut sep: &str = ", ";
    let mut str_vals: Vec<String> = Vec::new();
    if elem.vr == &vr::AT {
        str_vals.push(Tag::format_tag_to_display(elem.parse_attribute()?));
    } else if elem.vr == &vr::FL || elem.vr == &vr::OF {
        sep = " / ";
        let vec: Vec<f32> = elem.parse_f32s()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: f32| format!("{:.2}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::FD || elem.vr == &vr::OD {
        sep = " / ";
        let vec: Vec<f64> = elem.parse_f64s()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: f64| format!("{:.2}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::SS || elem.vr == &vr::OW {
        sep = " / ";
        let vec: Vec<i16> = elem.parse_i16s()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: i16| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::SL || elem.vr == &vr::OL {
        sep = " / ";
        let vec: Vec<i32> = elem.parse_i32s()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: i32| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::UI {
        let str_val: String = elem.parse_string()?;
        if let Some(uid) = UID_BY_UID.get(str_val.as_str()) {
            str_vals.push(format!("{} ({})", str_val, uid.name));
        } else {
            str_vals.push(str_val);
        }
    } else if elem.vr == &vr::UL {
        str_vals.push(format!("{}", elem.parse_u32()?));
    } else if elem.vr == &vr::US {
        str_vals.push(format!("{}", elem.parse_u16()?));
    } else if elem.vr.is_character_string {
        let vec: Vec<String> = elem.parse_strings()?;
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

/// Renders an element's tag in the format
/// ```
/// (gggg,eeee) VR TagName
/// ```
/// Names for private tags will render as `<PrivateTag>`
pub fn render_element_tag(element: &DicomElement) -> String {
    let tag_num: String = Tag::format_tag_to_display(element.tag);

    let tag_name: &str = if let Some(tag) = TAG_BY_VALUE.get(&element.tag) {
        &tag.ident
    } else {
        "<Private Tag>"
    };

    format!("{} {} {}", tag_num, element.vr.ident, tag_name)
}
