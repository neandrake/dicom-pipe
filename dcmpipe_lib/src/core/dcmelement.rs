//! DICOM Element Definition

use std::fmt;
use std::iter::once;

use crate::{
    core::{
        charset::{CSRef, DEFAULT_CHARACTER_SET},
        dcmsqelem::SequenceElement,
        read::{self, parser::ParseResult},
        values::RawValue,
    },
    defn::{
        constants::tags,
        tag::{Tag, TagNode, TagPath},
        ts::TSRef,
        vl::ValueLength,
        vr::{self, VRRef},
    },
};

use super::write::valencode::ElemAndRawValue;

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
pub struct DicomElement {
    tag: u32,
    vr: VRRef,
    vl: ValueLength,

    data: Vec<u8>,
    sq_path: Vec<SequenceElement>,

    ts: TSRef,
    cs: CSRef,
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: VR[{:?}], VL[{:?}], TS[{:?}]",
            Tag::format_tag_to_display(self.get_tag()),
            self.get_vr(),
            self.get_vl(),
            self.get_ts().get_uid().get_ident()
        )
    }
}

impl DicomElement {
    pub fn new<T>(
        tag: T,
        vr: VRRef,
        vl: ValueLength,
        ts: TSRef,
        cs: CSRef,
        data: Vec<u8>,
        sq_path: Vec<SequenceElement>,
    ) -> Self
    where
        T: Into<u32>,
    {
        let cs: CSRef = vr.get_proper_cs(cs);
        DicomElement {
            tag: Into::<u32>::into(tag),
            vr,
            vl,
            data,
            sq_path,
            ts,
            cs,
        }
    }

    pub fn new_empty<T>(tag: T, vr: VRRef, ts: TSRef) -> Self
    where
        T: Into<u32>,
    {
        let cs: CSRef = vr.get_proper_cs(DEFAULT_CHARACTER_SET);
        DicomElement {
            tag: Into::<u32>::into(tag),
            vr,
            vl: ValueLength::UndefinedLength,
            data: Vec::with_capacity(0),
            sq_path: Vec::with_capacity(0),
            ts,
            cs,
        }
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    pub fn get_vl(&self) -> ValueLength {
        self.vl
    }

    pub fn get_vr(&self) -> VRRef {
        self.vr
    }

    pub fn get_ts(&self) -> TSRef {
        self.ts
    }

    pub fn get_cs(&self) -> CSRef {
        self.cs
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_sequence_path(&self) -> &Vec<SequenceElement> {
        &self.sq_path
    }

    /// Returns if this element is a `SQ` or if it should be parsed as though it were a sequence
    pub fn is_seq_like(&self) -> bool {
        self.vr == &vr::SQ || read::util::is_non_standard_seq(self.tag, self.vr, self.vl)
    }

    /// Returns whether the the size of the value for this element is zero
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Creates a `TagPath` for the current element
    pub fn get_tagpath(&self) -> TagPath {
        self.sq_path
            .iter()
            .filter(|sq| sq.get_seq_tag() != tags::ITEM)
            .map(|sq| sq.get_node().clone())
            .chain(once(self.tag.into()))
            .collect::<Vec<TagNode>>()
            .into()
    }

    /// Parses this element's data into native/raw value type.
    pub fn parse_value(&self) -> ParseResult<RawValue> {
        RawValue::try_from(self)
    }

    /// Encodes a RawValue into the binary data for this element.
    ///
    /// This will overwrite any existing value in this element in `self.data`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be encoded according to `self.vr`.
    /// * `vl` - The value length to use. If `None` then the value length will be computed and
    /// `ValueLength::Explicit` will be assigned to `self.vl`. If `Some` then it will only be used
    /// if `self.is_seq_like()` would return true, otherwise the value length will be computed and
    /// `ValueLength::Explicit` will be assigned to `self.vl`. Unconditionally, `self.vl` will be
    /// assigned `ValueLength::Explicit(0)` if this element is `Item`, `ItemDelimitationItem`, or
    /// `SequenceDelimitationItem`.
    pub fn encode_value(&mut self, value: RawValue, vl: Option<ValueLength>) -> ParseResult<()> {
        self.data = ElemAndRawValue(self, value).try_into()?;

        self.vl = if vl.is_some() && self.is_seq_like() || self.tag == tags::ITEM {
            vl.unwrap()
        } else if self.tag == tags::ITEM_DELIMITATION_ITEM
            || self.tag == tags::SEQUENCE_DELIMITATION_ITEM
        {
            ValueLength::Explicit(0)
        } else {
            ValueLength::Explicit(self.data.len() as u32)
        };

        Ok(())
    }
}
