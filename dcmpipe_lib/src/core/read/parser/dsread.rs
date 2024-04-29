//! This module contains the primary logic for reading the components of a DICOM Element from the
//! dataset.
//!
//! The functionality contained within should only be called when reading FileMetaElements or
//! regular DICOM DataSet Elements.
//!
//! The parsing logic is intended to be minimal and lenient in order to handle old or
//! improperly-encoded DICOM. References to the DICOM Standard are made in inline comments to
//! clarify or justify the logic. This parsing does not attempt to interpret or decode the bytes of
//! the value field, and it does not validate the structure of sequences (i.e. the proper uses of
//! the Item, ItemDelimitationItem, and SequenceDelimitationItem elements.

use std::io::{ErrorKind, Read};

use crate::core::{
    charset::CSRef,
    dcmelement::DicomElement,
    dcmsqelem::SequenceElement,
    defn::{
        constants::{tags, ts},
        tag::Tag,
        ts::TSRef,
        vl::ValueLength,
        vr::{self, VRRef},
    },
    read::{
        self,
        parser::{ParseResult, Parser},
        util::is_non_standard_seq,
        ParseError,
    },
};

impl<'d, R: Read> Parser<'d, R> {
    /// Reads a tag attribute from the dataset, unless `self.partial_tag` is `Some`.
    pub(super) fn read_tag(&mut self, ts: TSRef) -> ParseResult<u32> {
        let tag: u32 = if let Some(partial_tag) = self.partial_tag {
            partial_tag
        } else {
            let tag: u32 = read::util::read_tag_from_dataset(&mut self.dataset, ts.big_endian())?;
            self.bytes_read += 4;
            self.partial_tag.replace(tag);
            tag
        };
        self.tag_last_read = tag;
        Ok(tag)
    }

    /// Reads the remainder of the dicom element from the dataset. This assumes `self.read_tag()`
    /// was called just prior and its result passed as the tag parameter here.
    pub(super) fn read_dicom_element(
        &mut self,
        tag: u32,
        elem_ts: TSRef,
    ) -> ParseResult<DicomElement> {
        // Part 5, Section 7.5
        // There are three special SQ related Data Elements that are not ruled by the VR encoding
        // rules conveyed by the Transfer Syntax. They shall be encoded as Implicit VR. These
        // special Data Elements are Item (FFFE,E000), Item Delimitation Item (FFFE,E00D), and
        // Sequence Delimitation Item (FFFE,E0DD). However, the Data Set within the Value Field of
        // the Data Element Item (FFFE,E000) shall be encoded according to the rules conveyed by the
        // Transfer Syntax.
        let is_seq_delim = tag == tags::SEQUENCE_DELIMITATION_ITEM
            || tag == tags::ITEM_DELIMITATION_ITEM
            || tag == tags::ITEM;
        // See: Part 5, Section 6.2.2
        // Elements within a Private Sequence with VR of UN should be in ImplicitVR.
        // Elements within a Private Sequence with VR of SQ and VL of Undefined should use the
        //   Dataset Transfer Syntax.
        // XXX: ?? Elements within a Private Sequence with VR of SQ and VL of Explicit should be in
        //   ImplicitVR.
        let is_parent_priv_seq = self.current_path.iter().rev().any(|sq_el| {
            Tag::is_private(sq_el.seq_tag())
                && is_non_standard_seq(sq_el.seq_tag(), sq_el.vr(), sq_el.vl())
        });

        let ts: TSRef = if is_seq_delim || is_parent_priv_seq {
            if elem_ts.big_endian() {
                &ts::ImplicitVRBigEndian
            } else {
                &ts::ImplicitVRLittleEndian
            }
        } else {
            elem_ts
        };

        let vr: VRRef = if ts.explicit_vr {
            // The `partial_vr` may be populated as part of initial dataset parsing when attempting
            // to detect the transfer syntax. The UnknownExplicitVR error used here is only
            // transient error to transition to the `self.read_vr()`.
            let vr_res: ParseResult<VRRef> = self
                .partial_vr
                .take()
                .ok_or(ParseError::UnknownExplicitVR(0))
                .or_else(|_e| self.read_vr());
            match vr_res {
                Ok(vr) => vr,
                Err(ParseError::UnknownExplicitVR(_code)) => &vr::INVALID,
                Err(e) => return Err(e),
            }
        } else {
            // Implicit VR, look up in the current dictionary or assume UN if not resolved.
            if let Some(vr) = self.lookup_vr(tag) {
                vr
            } else {
                &vr::UN
            }
        };
        self.vr_last_used.replace(vr);

        let vl: ValueLength = if let Some(partial_vl) = self.partial_vl {
            self.partial_vl.take();
            partial_vl
        } else {
            self.read_value_length(ts, vr)?
        };
        self.vl_last_used.replace(vl);

        let parse_as_seq: bool = read::util::is_non_standard_seq(tag, vr, vl);
        let ts: TSRef = if parse_as_seq {
            if !ts.big_endian() {
                &ts::ImplicitVRLittleEndian
            } else {
                &ts::ImplicitVRBigEndian
            }
        } else {
            ts
        };
        self.ts_last_used.replace(ts);

        // Sequence and item elements should let the iterator handle parsing its contents and not
        // associate bytes to the element's value. The exception are item elements within pixel data
        // which are used to encapsulate frames; their value is pixel data and not other elements.
        let in_pixeldata: bool = self.is_in_pixeldata();

        // Determine whether the value should be read in as byte values or instead should continue
        // being parsed as more elements.
        let skip_bytes: bool =
            vr == &vr::SQ || (tag == tags::ITEM && !in_pixeldata) || parse_as_seq;

        //eprintln!("{}", &self.get_debug_str(ts, tag, vr, vl));

        let bytes: Vec<u8> = if skip_bytes {
            Vec::with_capacity(0)
        } else {
            self.read_value_field(tag, vl)?
        };

        let ancestors: Vec<SequenceElement> = self.current_path.clone();

        let cs: CSRef = if let Some(sq) = ancestors.last() {
            sq.cs()
        } else {
            self.cs
        };

        Ok(DicomElement::new(tag, vr, vl, ts, cs, bytes, ancestors))
    }

    /// Reads VR from the dataset. This should only be done for ExplicitVR transfer syntaxes.
    /// If the VR read from the dataset indicates it contains additional 2-byte-padding for
    /// explicit VRs then those bytes are also read (and thrown away). If the bytes do not
    /// correspond to a valid/known VR then `ParseError::UnknownExplicitVR` is returned.
    fn read_vr(&mut self) -> ParseResult<VRRef> {
        match read::util::read_vr_from_dataset(&mut self.dataset) {
            Ok(vr) => {
                self.bytes_read += 2;
                if vr.has_explicit_2byte_pad {
                    self.bytes_read += 2;
                }
                Ok(vr)
            }
            Err(e) => {
                self.bytes_read += 2;
                Err(e)
            }
        }
    }

    /// Looks up the implicit VR of the given tag in the current dictionary.
    fn lookup_vr(&self, tag: u32) -> Option<VRRef> {
        self.dictionary
            .get_tag_by_number(tag)
            .and_then(|read_tag: &Tag| read_tag.implicit_vr())
    }

    /// Reads a Value Length attribute from the dataset using the given transfer syntax. The number
    /// of bytes representing the value length depends on transfer syntax. If the VR has a 2-byte
    /// padding then those bytes are also read from the dataset.
    fn read_value_length(&mut self, ts: TSRef, vr: VRRef) -> ParseResult<ValueLength> {
        let result: ParseResult<ValueLength> =
            read::util::read_value_length_from_dataset(&mut self.dataset, ts, vr);
        if result.is_ok() {
            // For Implicit VR or Explicit w/ 2-byte pad then Value Length is read as a u32,
            // otherwise it's read as a u16.
            if !ts.explicit_vr() || vr.has_explicit_2byte_pad {
                self.bytes_read += 4;
            } else {
                self.bytes_read += 2;
            }
        }
        result
    }

    /// Reads the value field of the dicom element into a byte array. If the `ValueLength` is
    /// undefined then this returns an empty array as elements with undefined length should have
    /// their contents parsed as dicom elements.
    fn read_value_field(&mut self, tag: u32, vl: ValueLength) -> ParseResult<Vec<u8>> {
        match vl {
            // Undefined length means that the contents of the element are other dicom elements to
            // be parsed. Don't read data from the dataset in this case.
            ValueLength::Explicit(0) | ValueLength::UndefinedLength => Ok(Vec::with_capacity(0)),
            ValueLength::Explicit(value_length) => {
                // If length is odd we only read that exact bytes from the dataset but the bytes
                // we should return from this should be padded with a zero in order to always
                // return an even-length value.
                let value_length: usize = usize::try_from(value_length).unwrap_or_default();
                let buffer_size: usize = if value_length % 2 != 0 {
                    value_length + 1
                } else {
                    value_length
                };
                let mut buffer: Vec<u8> = vec![0; buffer_size];
                let buffer_slice: &mut [u8] = &mut buffer.as_mut_slice()[0..value_length];
                let result: ParseResult<()> = self.dataset.read_exact(buffer_slice).map_err(|e| {
                    // Some datasets may end with this DataSetTrailingPadding tag (or just all
                    // zeroes) and also have value length which does not match the actual value
                    // field's size. The standard indicates that the content of the value field
                    // should hold no significance - consider this not an error.
                    // See Part 10, Section 7.2
                    if (tag == 0 || tag == tags::DATASET_TRAILING_PADDING)
                        && e.kind() == ErrorKind::UnexpectedEof
                    {
                        // TODO: Take what values were read and return that as a byte array, so the
                        //       original contents of the dataset are retained if needed.
                        ParseError::ExpectedEOF
                    } else {
                        ParseError::IOError { source: e }
                    }
                });

                match result {
                    Ok(_) => {
                        self.bytes_read += u64::try_from(value_length).unwrap_or_default();
                        Ok(buffer)
                    }
                    Err(ParseError::ExpectedEOF) => {
                        self.bytes_read += u64::try_from(value_length).unwrap_or_default();
                        Err(ParseError::ExpectedEOF)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
