//! This module contains the implementation for `Parser`'s transfer syntax detection.
//!
//! This is designed to be the initial state of the parser when parsing files, as opposed to
//! parsing DICOM datasets from DIMSE network traffic which requires the use of
//! ExplicitVRLittleEndian. The is also designed so that if file preamble/prefix is detected
//! to keep the parser in the `DetectTransferSyntax` state, expecting this to be called one more
//! time to start detection again.
//!
//! The logic is complex, to avoid backtracking and managing additional buffers -- it only pulls
//! the minimal bytes from the stream for detection and puts the parser state into partial-read
//! status. This is what populates the `partial_tag`, `partial_vr`, and `partial_vl` fields so they
//! are re-used by the other state iterations. It may also populate the file preamble.

use std::io::{Cursor, Read};

use crate::{
    core::read::{
        self,
        parser::{ParseError, ParseResult, Parser, ParserState, FILE_PREAMBLE_LENGTH},
    },
    defn::{
        constants::{tags, ts},
        ts::TSRef,
        vl::ValueLength,
        vr::{self, VRRef},
    },
};

const MAX_VALUE_LENGTH_IN_DETECT: u32 = 100;

impl<'dict, DatasetType: Read> Parser<'dict, DatasetType> {
    /// Performs the `ParserState::DetectTransferSyntax` iteration.
    /// Detects little-vs-big endian and implicit-vs-explicit VR. This strategy is not fully
    /// complete however it does cover a wide variety of cases. It does not cover scenarios where
    /// endian-detection could possibly succeed incorrectly however these scenarios would likely
    /// be very odd dicom cases that most other libraries are also unable to parse.
    ///
    /// After this iteration runs once the `self.partial_` fields may be filled in,
    /// `self.file_preamble` may be filled in, and `self.detected_ts` may be changed.
    ///
    /// The strategy this uses is to parse just a few bytes from the dataset to see if it looks like
    /// the start of a dicom element, back-tracking the read bytes if they are not interpretable as
    /// valid parts of a dicom element. The steps for parsing are based on
    /// 1. Parse a tag and check whether it looks like a valid tag value
    ///    (`0 < tag < SOP_INSTANCE_UID`). The tag is parsed as both little and big endian.
    /// 2. Parse a VR assuming it's explicit. If the VR does not match a known VR then it's assumed
    ///    the transfer syntax is implicit.
    /// 3. Parse a value length. The value length parsed is checked to be reasonable for a dicom
    ///    element that would appear early in a dicom dataset (file-meta or early group `0008`,
    ///    which would be `< MAX_VALUE_LENGTH_IN_DETECT`). The elements at the beginning of a
    ///    dataset should have fairly small values (for things like UIDs, etc.). The Pixel-med
    ///    library also uses this same value comparison.
    /// 4. Otherwise it's assumed the start of the file is proprietary file preamble. These bytes
    ///    are then skipped and detection begins again.
    pub(super) fn iterate_detect_state(&mut self) -> ParseResult<()> {
        // start off assuming EVRLE, the default for File-Meta
        let mut ts: TSRef = &ts::ExplicitVRLittleEndian;

        // if this function has been called previously it will have read data into
        // `self.file_preamble` for invalid element cases - if preamble is already read then those
        // invalid element cases should now turn into errors
        let already_read_preamble: bool = self.file_preamble.is_some();

        // as bytes are read from `self.dataset` they will be copied into this `file_preamble`, then
        // if it's determined that we're likely in a file preamble the rest of the standard
        // preamble will be read from the dataset and stored into `self.file_preamble`
        let mut file_preamble: [u8; FILE_PREAMBLE_LENGTH] = [0; FILE_PREAMBLE_LENGTH];
        let mut bytes_read: usize = 0;

        let mut buf: [u8; 4] = [0; 4];
        self.dataset.read_exact(&mut buf)?;

        // copy the read bytes into preamble in case we determine this is a preamble -- use a
        // cursor to allow re-parsing the same bytes again for checking both endian
        file_preamble[bytes_read..(bytes_read + buf.len())].copy_from_slice(&buf);
        bytes_read += buf.len();
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);

        let mut tag: u32 = read::util::read_tag_from_dataset(&mut cursor, ts.big_endian())?;

        // a zero tag is valid for dimse, however dimse parser should not need to start by
        // identifying the transfer syntax, as it should always be ImplicitVRLittleEndian
        if tag == 0 {
            // if tag is zero then assume preamble, jump forward and attempt to detect tag after it

            // if file preamble was already read then flip into Element mode and let it fail
            if already_read_preamble {
                self.detected_ts = &ts::ImplicitVRLittleEndian;
                self.partial_tag = Some(tag);
                self.bytes_read += bytes_read as u64;
                self.state = ParserState::Element;
                return Ok(());
            }

            // read the remainder of the preamble, the prefix
            self.dataset
                .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
            self.bytes_read += file_preamble.len() as u64;
            self.file_preamble = Some(file_preamble);
            self.iterate_prefix()?;
            self.state = ParserState::DetectTransferSyntax;
            return Ok(());
        } else if !(tags::FILE_META_INFORMATION_GROUP_LENGTH..=tags::SOP_INSTANCE_UID)
            .contains(&tag)
        {
            cursor.set_position(0);
            ts = &ts::ExplicitVRBigEndian;
            tag = read::util::read_tag_from_dataset(&mut cursor, ts.big_endian())?;

            // if switching endian didn't result in a valid tag then try skipping preamble/prefix
            if !(tags::FILE_META_INFORMATION_GROUP_LENGTH..=tags::SOP_INSTANCE_UID).contains(&tag) {
                // if file preamble was already read then flip into Element mode and let it fail
                if already_read_preamble {
                    self.detected_ts = &ts::ImplicitVRLittleEndian;
                    self.partial_tag = Some(tag);
                    self.bytes_read += bytes_read as u64;
                    self.state = ParserState::Element;
                    return Ok(());
                }

                // read the remainder of the preamble, the prefix
                self.dataset
                    .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
                self.bytes_read += file_preamble.len() as u64;
                self.file_preamble = Some(file_preamble);
                self.iterate_prefix()?;
                self.state = ParserState::DetectTransferSyntax;
                return Ok(());
            }
        }

        // if not an expected non-file-meta tag then try big-endian
        if !ts.big_endian() && tag < tags::FILE_META_INFORMATION_GROUP_LENGTH
            || tag > tags::SOP_INSTANCE_UID
        {
            cursor.set_position(0);
            ts = &ts::ExplicitVRBigEndian;
            tag = read::util::read_tag_from_dataset(&mut cursor, ts.big_endian())?;
        }

        // doesn't appear to be a valid tag in either big or little endian
        if tag < tags::FILE_META_INFORMATION_GROUP_LENGTH
            || tag > tags::SOP_INSTANCE_UID && already_read_preamble
        {
            // testing tag in either endian didn't seem to work, set as DICOM default
            self.detected_ts = &ts::ImplicitVRLittleEndian;
            self.partial_tag = Some(tag);
            self.bytes_read += bytes_read as u64;
            self.state = ParserState::Element;
            return Ok(());
        }

        // read in 4 bytes. for implicit vr 4 bytes are used for value length. if it's not implicit
        // then the first two bytes are re-parsed as vr and the later two bytes are the value length
        // note: we could attempt to match the first two bytes as a valid VR to determine implicit
        // vs. explicit vr however this current approach also works because all VR options have
        // really high binary values.
        let mut buf: [u8; 4] = [0; 4];
        self.dataset.read_exact(&mut buf)?;
        // if we haven't already skipped preamble/prefix then continue to copy bytes read from the
        // dataset into the buffer to be stored as preamble we discover that we are in a preamble
        if !already_read_preamble {
            file_preamble[bytes_read..(bytes_read + buf.len())].copy_from_slice(&buf);
        }
        bytes_read += buf.len();
        cursor = Cursor::new(&buf);

        let vr: VRRef = match read::util::read_vr_from_dataset(&mut cursor) {
            Ok(vr) => {
                self.partial_vr = Some(vr);
                if vr.has_explicit_2byte_pad {
                    // if explict & padded then the padding was read-in already and we have to read
                    // in the next 4 bytes for the value length.
                    self.dataset.read_exact(&mut buf)?;
                    if !already_read_preamble {
                        file_preamble[bytes_read..(bytes_read + buf.len())].copy_from_slice(&buf);
                    }
                    bytes_read += buf.len();
                    cursor = Cursor::new(&buf);
                } else {
                    // with no padding value length is only 2 bytes and was read-in as part of the
                    // buffer for vr above.
                    cursor.set_position(2);
                }

                if ts.big_endian() {
                    ts = &ts::ExplicitVRBigEndian;
                } else {
                    ts = &ts::ExplicitVRLittleEndian;
                }
                vr
            }
            Err(ParseError::UnknownExplicitVR(_)) => {
                // unknown VR so this was likely a value length read in, reset the 4-byte buffer
                // read in as vr and next read it as value length.
                cursor.set_position(0);
                if ts.big_endian() {
                    ts = &ts::ImplicitVRBigEndian;
                } else {
                    ts = &ts::ImplicitVRLittleEndian;
                }

                // vr is used to determine how many bytes the value length should be, which after
                // switching transfer syntax to implicit it will always be 4byte u32.
                &vr::INVALID
            }
            Err(e) => {
                return Err(e);
            }
        };

        // assume implicit VR so read a value length and and if it's reasonably low then this is
        // likely implicit
        let vl: ValueLength = read::util::read_value_length_from_dataset(&mut cursor, ts, vr)?;
        if let ValueLength::Explicit(len) = vl {
            // if a value length is read which makes sense for file-meta elements then assume this
            // is implicit endian and let regular parsing continue
            if len < MAX_VALUE_LENGTH_IN_DETECT {
                self.detected_ts = ts;
                self.partial_tag = Some(tag);
                self.partial_vl = Some(vl);
                self.bytes_read += bytes_read as u64;
                // FileMeta is coded to read as ExplicitVRLittleEndian and since we've determined
                // this is implicit we skip to Element which will follow self.ts
                if tag < tags::FILE_META_GROUP_END {
                    if tag == tags::FILE_META_INFORMATION_GROUP_LENGTH {
                        self.state = ParserState::GroupLength;
                    } else {
                        self.state = ParserState::FileMeta;
                    }
                } else {
                    self.state = ParserState::Element;
                }
                return Ok(());
            }
        }

        if already_read_preamble {
            self.detected_ts = &ts::ImplicitVRLittleEndian;
            self.partial_tag = Some(tag);
            self.partial_vl = Some(vl);
            self.bytes_read += bytes_read as u64;
            self.state = ParserState::Element;
            return Ok(());
        }

        // garbage data so likely in preamble, finish reading preamble and prefix, restart detect
        self.dataset
            .read_exact(&mut file_preamble[bytes_read..FILE_PREAMBLE_LENGTH])?;
        self.bytes_read += file_preamble.len() as u64;
        self.file_preamble = Some(file_preamble);
        self.partial_tag = None;
        self.partial_vr = None;
        self.partial_vl = None;
        self.iterate_prefix()?;
        self.state = ParserState::DetectTransferSyntax;
        Ok(())
    }
}
