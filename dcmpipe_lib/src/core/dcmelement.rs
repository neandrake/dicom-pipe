//! DICOM Element Definition

use std::{fmt, iter::once};

use crate::core::{
    charset::{CSRef, DEFAULT_CHARACTER_SET},
    dcmsqelem::SequenceElement,
    defn::{
        constants::{
            tags::{ITEM, ITEM_DELIMITATION_ITEM, SEQUENCE_DELIMITATION_ITEM},
            ts::ExplicitVRLittleEndian,
        },
        is_non_standard_sq,
        tag::{Tag, TagNode, TagPath},
        ts::TSRef,
        vl::ValueLength,
        vr::VRRef,
        vr::{INVALID_VR, SQ},
    },
    read::parser::ParseResult,
    values::RawValue,
    write::valencode::ElemAndRawValue,
};

use super::values::ElementWithVr;

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
            "{}: VR[{:?}], VL[{:?}], TS[{}]",
            Tag::format_tag_to_display(self.tag()),
            self.vr(),
            self.vl(),
            self.ts().uid().ident()
        )
    }
}

impl DicomElement {
    /// Creates a new element with all fields specified.
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
        u32: From<T>,
    {
        let cs: CSRef = vr.get_proper_cs(cs);
        Self {
            tag: u32::from(tag),
            vr,
            vl,
            data,
            sq_path,
            ts,
            cs,
        }
    }

    /// Creates a new empty element with the given tag, VR, and transfer syntax. This element will
    /// be initialized with the default character set.
    pub fn new_empty<T>(tag: T, vr: VRRef, ts: TSRef) -> Self
    where
        u32: From<T>,
    {
        let cs: CSRef = vr.get_proper_cs(DEFAULT_CHARACTER_SET);
        Self {
            tag: u32::from(tag),
            vr,
            vl: ValueLength::Explicit(0),
            data: Vec::with_capacity(0),
            sq_path: Vec::with_capacity(0),
            ts,
            cs,
        }
    }

    /// Creates a new sentinel element which is not a valid `DicomElement`. This can be
    /// distinguished from a valid `DicomElement` by having a tag value of zero, VR of `INVALID`,
    /// and transfer syntax of `ExplicitVRLittleEndian`. Note that all three of these elements
    /// should be checked as a tag of zero is valid for DIMSE, "Command Group Length". Use the
    /// `is_sentinel()` function to check for this specific case.
    pub fn new_sentinel() -> Self {
        DicomElement {
            tag: 0,
            vr: &INVALID_VR,
            vl: ValueLength::Explicit(0),
            data: Vec::with_capacity(0),
            sq_path: Vec::with_capacity(0),
            ts: &ExplicitVRLittleEndian,
            cs: DEFAULT_CHARACTER_SET,
        }
    }

    pub fn tag(&self) -> u32 {
        self.tag
    }

    pub fn vl(&self) -> ValueLength {
        self.vl
    }

    pub fn vr(&self) -> VRRef {
        self.vr
    }

    pub fn ts(&self) -> TSRef {
        self.ts
    }

    pub fn cs(&self) -> CSRef {
        self.cs
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn sq_path(&self) -> &Vec<SequenceElement> {
        &self.sq_path
    }

    /// Returns the number of bytes this element will consist of when encoded into a dataset.
    /// Refer to Part 5, Chapter 7.1
    pub fn byte_size(&self) -> usize {
        let mut byte_len = 0usize;

        // tag
        byte_len += 4;

        // vr
        byte_len += if self.ts.explicit_vr() {
            if self.vr.has_explicit_2byte_pad {
                4
            } else {
                2
            }
        } else {
            0
        };

        // vl
        byte_len += if self.ts.explicit_vr() {
            if self.vr.has_explicit_2byte_pad {
                4
            } else {
                2
            }
        } else {
            4
        };

        // value
        byte_len += self.data.len();

        byte_len
    }

    /// Returns if this element is a `SQ` or if it should be parsed as though it were a sequence.
    pub fn is_sq_like(&self) -> bool {
        self.vr == &SQ || is_non_standard_sq(self.tag, self.vr, self.vl)
    }

    /// Returns whether the the size of the value field for this element is zero.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Checks if this `DicomElement` is a sentinel value, identified by having the following
    /// properties:
    ///
    /// - Tag value of zero
    /// - VR of `&vr::INVALID`
    /// - Transfer Syntax of `&ts::ImplicitVRLittleEndian`
    pub fn is_sentinel(&self) -> bool {
        self.tag == 0 && self.vr == &INVALID_VR && self.ts == &ExplicitVRLittleEndian
    }

    /// Creates a `TagPath` for the current element.
    pub fn create_tagpath(&self) -> TagPath {
        if self.is_sentinel() {
            TagPath::empty()
        } else {
            self.sq_path
                .iter()
                .filter(|sq| sq.seq_tag() != ITEM)
                .map(|sq| sq.node().clone())
                .chain(once(self.tag.into()))
                .collect::<Vec<TagNode>>()
                .into()
        }
    }

    /// Parses this element's data into native/raw value type.
    pub fn parse_value(&self) -> ParseResult<RawValue> {
        RawValue::try_from(self)
    }

    /// Parses this element's data into native/raw value type, using a specific value
    /// representation.
    pub fn parse_value_as(&self, vr: VRRef) -> ParseResult<RawValue> {
        RawValue::try_from(ElementWithVr(self, vr))
    }

    /// Encodes a `RawValue` into the binary data for this element.
    ///
    /// This will overwrite any existing value in this element's `self.data`.
    pub fn encode_val(&mut self, value: RawValue) -> ParseResult<()> {
        self.encode_val_with_vl(value, None)
    }

    /// Encodes a `RawValue` into the binary data for this element.
    ///
    /// This will overwrite any existing value in this element's `self.data`.
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
    pub fn encode_val_with_vl(
        &mut self,
        value: RawValue,
        vl: Option<ValueLength>,
    ) -> ParseResult<()> {
        self.data = ElemAndRawValue(self, value).try_into()?;

        self.vl = if vl.is_some() && self.is_sq_like() || self.tag == ITEM {
            vl.unwrap()
        } else if self.tag == ITEM_DELIMITATION_ITEM || self.tag == SEQUENCE_DELIMITATION_ITEM {
            ValueLength::Explicit(0)
        } else {
            u32::try_from(self.data.len())
                .map(ValueLength::Explicit)
                .unwrap_or(ValueLength::UndefinedLength)
        };

        Ok(())
    }
}
