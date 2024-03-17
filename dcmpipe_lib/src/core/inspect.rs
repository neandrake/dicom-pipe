//! Utilities for inspecting or debugging DICOM.

use std::fmt;

use crate::core::{
    dcmelement::DicomElement,
    dcmsqelem::SequenceElement,
    defn::{
        constants::{
            lookup::MINIMAL_DICOM_DICTIONARY,
            tags::{ITEM, ITEM_DELIMITATION_ITEM, SEQUENCE_DELIMITATION_ITEM},
        },
        vl::ValueLength,
        vr::{INVALID_VR, UN},
    },
    defn::{dcmdict::DicomDictionary, tag::Tag},
    RawValue,
};

#[cfg(feature = "stddicom")]
use crate::dict::stdlookup::STANDARD_DICOM_DICTIONARY;

/// Convenience for coordinating a tag's type display.
pub enum FormattedTagType {
    Known(u32, String),
    PrivateCreator(u32),
    PrivateSequence(u32),
    PrivateGroupLength(u32),
    Private(u32),
    GroupLength(u32),
    Unknown(u32),
}

impl fmt::Display for FormattedTagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedTagType::Known(_, display) => write!(f, "{}", display),
            FormattedTagType::PrivateCreator(_) => write!(f, "<PrivateCreator>"),
            FormattedTagType::PrivateSequence(_) => write!(f, "<PrivateSequence>"),
            FormattedTagType::PrivateGroupLength(_) => write!(f, "<PrivateGroupLength>"),
            FormattedTagType::Private(_) => write!(f, "<PrivateTag>"),
            FormattedTagType::GroupLength(_) => write!(f, "<GroupLength>"),
            FormattedTagType::Unknown(_) => write!(f, "<UnknownTag>"),
        }
    }
}

/// Convenience for coordinating a tag's display.
pub enum FormattedTagValue {
    Sequence,
    Error(String),
    Uid(String, String),
    Stringified(String),
}

impl fmt::Display for FormattedTagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedTagValue::Sequence => Ok(()),
            FormattedTagValue::Error(error_str) => write!(f, "<Error {}>", error_str),
            FormattedTagValue::Uid(uid, name) => {
                if name.is_empty() {
                    write!(f, "{}", uid)
                } else {
                    write!(f, "{} => {}", uid, name)
                }
            }
            FormattedTagValue::Stringified(value) => write!(f, "{}", value),
        }
    }
}

/// Convenience for coordinating an element's display with formatting options.
pub struct FormattedElement<'e> {
    /// The element to render.
    pub elem: &'e DicomElement,
    /// Whether the value can render across a single line or multiple lines.
    pub multiline: bool,
    /// The maximum number of value items to display.
    pub max_items: usize,
    /// Whether sequence and item delimiters should be rendered or not.
    pub hide_delims: bool,
    /// Whether group length elements should be rendered or not.
    pub hide_groups: bool,
}

impl<'e> FormattedElement<'e> {
    pub fn default(elem: &'e DicomElement) -> Self {
        Self {
            elem,
            multiline: false,
            max_items: 16,
            hide_delims: false,
            hide_groups: false,
        }
    }

    pub fn should_omit(&self) -> bool {
        (self.hide_delims
            && (self.elem.tag() == ITEM_DELIMITATION_ITEM
                || self.elem.tag() == SEQUENCE_DELIMITATION_ITEM))
            || (self.hide_groups && Tag::is_group_length(self.elem.tag()))
    }

    /// Formats `vec` converting each element to a String based on the given `func`.
    /// Returns true if the input `vec` had more items than rendered, based on `MAX_ITEMS_DISPLAYED`.
    fn format_vec_to_strings<T, F: Fn(T) -> String>(
        &self,
        vec: Vec<T>,
        func: F,
    ) -> (bool, Vec<String>) {
        let vec_len: usize = vec.len();
        let formatted: Vec<String> = vec
            .into_iter()
            .take(self.max_items)
            .map(func)
            .collect::<Vec<String>>();
        (formatted.len() < vec_len, formatted)
    }

    pub fn get_tag_type(&self) -> FormattedTagType {
        if let Some(tag) = MINIMAL_DICOM_DICTIONARY.get_tag_by_number(self.elem.tag()) {
            FormattedTagType::Known(tag.tag, tag.ident.to_string())
        } else if Tag::is_private_creator(self.elem.tag()) {
            FormattedTagType::PrivateCreator(self.elem.tag())
        } else if Tag::is_private(self.elem.tag()) && self.elem.is_sq_like() {
            FormattedTagType::PrivateSequence(self.elem.tag())
        } else if Tag::is_private_group_length(self.elem.tag()) {
            FormattedTagType::PrivateGroupLength(self.elem.tag())
        } else if Tag::is_private(self.elem.tag()) {
            FormattedTagType::Private(self.elem.tag())
        } else if Tag::is_group_length(self.elem.tag()) {
            FormattedTagType::GroupLength(self.elem.tag())
        } else {
            #[cfg(feature = "stddicom")]
            if let Some(tag) = STANDARD_DICOM_DICTIONARY.get_tag_by_number(self.elem.tag()) {
                return FormattedTagType::Known(tag.tag, tag.ident.to_string());
            }

            FormattedTagType::Unknown(self.elem.tag())
        }
    }

    pub fn get_tag_value(&self) -> FormattedTagValue {
        if self.elem.is_sq_like() {
            return FormattedTagValue::Sequence;
        }

        let mut sep = if self.multiline { " " } else { "\\" };
        let elem_value = match self.elem.parse_value() {
            Err(e) => return FormattedTagValue::Error(e.to_string()),
            Ok(val) => val,
        };

        let (add_ellipses, mut str_vals) = match elem_value {
            RawValue::Attribute(attrs) => {
                self.format_vec_to_strings(attrs, |attr| Tag::format_tag_to_display(attr.0))
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
                        return FormattedTagValue::Uid(uid_str.to_string(), uid_name.to_string());
                    }
                    None => return FormattedTagValue::Uid(uid_str.to_string(), String::new()),
                }
            }
            RawValue::Strings(strings) => {
                if self.multiline {
                    sep = "\n";
                }
                self.format_vec_to_strings(strings, |val: String| {
                    if !self.multiline {
                        val.replace("\r\n", " / ").replace('\n', " / ")
                    } else {
                        val
                    }
                })
            }
            RawValue::Shorts(shorts) => {
                self.format_vec_to_strings(shorts, |val: i16| format!("{}", val))
            }
            RawValue::UShorts(ushorts) => {
                self.format_vec_to_strings(ushorts, |val: u16| format!("{}", val))
            }
            RawValue::Ints(ints) => self.format_vec_to_strings(ints, |val: i32| format!("{}", val)),
            RawValue::UInts(uints) => {
                self.format_vec_to_strings(uints, |val: u32| format!("{}", val))
            }
            RawValue::Longs(longs) => {
                self.format_vec_to_strings(longs, |val: i64| format!("{}", val))
            }
            RawValue::ULongs(ulongs) => {
                self.format_vec_to_strings(ulongs, |val: u64| format!("{}", val))
            }
            RawValue::Floats(floats) => {
                self.format_vec_to_strings(floats, |val: f32| format!("{:.2}", val))
            }
            RawValue::Doubles(doubles) => {
                self.format_vec_to_strings(doubles, |val: f64| format!("{:.2}", val))
            }
            RawValue::Bytes(bytes) => {
                self.format_vec_to_strings(bytes, |val: u8| format!("{:02x}", val))
            }
            RawValue::Words(words) => {
                self.format_vec_to_strings(words, |val: u16| format!("{:04x}", val))
            }
            RawValue::DWords(dwords) => {
                self.format_vec_to_strings(dwords, |val: u32| format!("{:06x}", val))
            }
            RawValue::QWords(qwords) => {
                self.format_vec_to_strings(qwords, |val: u64| format!("{:08x}", val))
            }
        };

        if add_ellipses {
            str_vals.push("..".to_string());
        }

        let num_vals: usize = str_vals.len();
        if num_vals == 1 {
            return FormattedTagValue::Stringified(str_vals.remove(0));
        }

        FormattedTagValue::Stringified(str_vals.join(sep).to_string())
    }
}

impl<'e> fmt::Display for FormattedElement<'e> {
    /// Renders an element on a single line, includes indentation based on depth in sequences. If the
    /// standard dicom dictionary feature is enabled then the tag name will attempt to be resolved
    /// using it.
    ///
    /// ```text
    /// (gggg,eeee) VR TagName [VL] | TagValue
    /// ```
    /// or
    /// ```text
    /// (gggg,eeee) VR TagName [0] <empty>
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Group Length tags are deprecated, see note on Part 5 Section 7.2
        if self.hide_groups && self.elem.tag().trailing_zeros() >= 16 {
            return Ok(());
        }

        // These are delimiter items that are not very useful to see
        if self.hide_delims
            && (self.elem.tag() == ITEM_DELIMITATION_ITEM
                || self.elem.tag() == SEQUENCE_DELIMITATION_ITEM)
        {
            return Ok(());
        }

        // Some (malformed?) datasets have a bunch of zeroes between self.elem..
        if self.elem.tag() == 0
            && ((self.elem.ts().explicit_vr() && self.elem.vr() == &INVALID_VR)
                || (!self.elem.ts().explicit_vr() && self.elem.vr() == &UN))
            && self.elem.vl() == ValueLength::Explicit(0)
        {
            return Ok(());
        }

        let tag_num: String = Tag::format_tag_to_display(self.elem.tag());
        let tag_name: FormattedTagType = self.get_tag_type();
        let vr: &str = self.elem.vr().ident;

        let vl: String = match self.elem.vl() {
            ValueLength::Explicit(len) => {
                if len % 2 != 0 {
                    format!("[*{:?}]", self.elem.vl())
                } else {
                    format!("[{:?}]", self.elem.vl())
                }
            }
            ValueLength::UndefinedLength => "[u/l]".to_string(),
        };

        // Sequence path will nest tags under ITEM self.elem.. Double the indentation level for the
        // number of nested sequences (non-ITEM), and each ITEM self.elem.should be nested one level.
        // If the current self.elem.is a delimiter then reduce the associated indentation level.
        let seq_path: &Vec<SequenceElement> = self.elem.sq_path();
        let non_item_parents = seq_path
            .iter()
            .filter(|sq_el| sq_el.seq_tag() != ITEM)
            .count();
        let item_parents = seq_path
            .iter()
            .filter(|sq_el| sq_el.seq_tag() == ITEM)
            .count();
        let mut indent_width = non_item_parents * 2 + item_parents;
        if self.elem.tag() == ITEM_DELIMITATION_ITEM {
            indent_width -= 1;
        } else if self.elem.tag() == SEQUENCE_DELIMITATION_ITEM {
            indent_width -= 2;
        }

        if self.elem.tag() == ITEM {
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
            return write!(
                f,
                "{indentation:indent_width$}{tag_name} {item_desc}",
                indentation = "",
                indent_width = indent_width,
                tag_name = tag_name,
                item_desc = item_desc,
            );
        }

        let tag_value: FormattedTagValue = self.get_tag_value();
        let mut tag_value: String = tag_value.to_string();
        if !tag_value.is_empty() {
            if self.elem.is_empty() {
                tag_value = format!(" {}", tag_value);
            } else {
                tag_value = format!(" | {}", tag_value);
            }
        }

        write!(
            f,
            "{indentation:indent_width$}{tag_num} {vr} {tag_name} {vl}{tag_value}",
            indentation = "",
            indent_width = indent_width,
            tag_num = tag_num,
            vr = vr,
            tag_name = tag_name,
            vl = vl,
            tag_value = tag_value,
        )
    }
}
