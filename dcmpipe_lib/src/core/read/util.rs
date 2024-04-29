use std::collections::BTreeMap;
use std::io::{ErrorKind, Read};

use super::parser::{Parser, Result};
use crate::core::dcmelement::DicomElement;
use crate::core::dcmobject::{DicomObject, DicomRoot};
use crate::core::read::error::ParseError;
use crate::defn::constants::tags;
use crate::defn::ts::TSRef;
use crate::defn::vl;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef, VR};

/// Whether the element is a non-standard parent-able element. These are non-SQ, non-ITEM elements
/// with a VR of `UN`, `OB`, `OF`, or `OW` and have a value length of `UndefinedLength`. These
/// types of elements are considered either private-tag sequences or otherwise whose contents are
/// encoded as IVRLE.
pub(crate) fn is_non_standard_seq(tag: u32, vr: VRRef, vl: ValueLength) -> bool {
    tag != tags::ITEM
        && (vr == &vr::UN || vr == &vr::OB || vr == &vr::OF || vr == &vr::OW)
        && vl == ValueLength::UndefinedLength
}

/// This is a variation of `Read::read_exact` however if zero bytes are read instead of returning
/// an error with `ErrorKind::UnexpectedEof` it will return an error with `ParseError::ExpectedEOF`.
fn read_exact_expect_eof(dataset: &mut impl Read, mut buf: &mut [u8]) -> Result<()> {
    let mut bytes_read: usize = 0;
    while !buf.is_empty() {
        match dataset.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                bytes_read += n;
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
    }
    if !buf.is_empty() {
        if bytes_read == 0 {
            Err(ParseError::ExpectedEOF)
        } else {
            Err(ParseError::IOError {
                source: std::io::Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("failed to fill whole buffer, read {} bytes", bytes_read),
                ),
            })
        }
    } else {
        Ok(())
    }
}

/// Reads a tag attribute from a given dataset
pub(crate) fn read_tag_from_dataset(dataset: &mut impl Read, big_endian: bool) -> Result<u32> {
    let mut buf: [u8; 2] = [0; 2];

    read_exact_expect_eof(dataset, &mut buf)?;
    let group_number: u32 = if big_endian {
        u32::from(u16::from_be_bytes(buf)) << 16
    } else {
        u32::from(u16::from_le_bytes(buf)) << 16
    };

    dataset.read_exact(&mut buf)?;
    let element_number: u32 = if big_endian {
        u32::from(u16::from_be_bytes(buf))
    } else {
        u32::from(u16::from_le_bytes(buf))
    };

    let tag: u32 = group_number + element_number;
    Ok(tag)
}

/// Reads a VR from a given dataset.
pub(crate) fn read_vr_from_dataset(dataset: &mut impl Read) -> Result<VRRef> {
    let mut buf: [u8; 2] = [0; 2];
    dataset.read_exact(&mut buf)?;
    let first_char: u8 = buf[0];
    let second_char: u8 = buf[1];

    let code: u16 = (u16::from(first_char) << 8) + u16::from(second_char);
    let vr: VRRef = match VR::from_code(code) {
        Some(found_vr) => {
            if found_vr.has_explicit_2byte_pad {
                dataset.read_exact(&mut buf)?;
            }
            found_vr
        }
        None => return Err(ParseError::UnknownExplicitVR(code)),
    };

    Ok(vr)
}

/// Reads a Value Length from a given dataset.
/// `dataset` The dataset to read bytes from.
/// `ts` The transfer syntax of the element being read from.
/// `vr` The VR of the current element the value length is being read for.
pub(crate) fn read_value_length_from_dataset(
    dataset: &mut impl Read,
    ts: TSRef,
    vr: VRRef,
) -> Result<ValueLength> {
    let value_length: u32 = if !ts.is_explicit_vr() || vr.has_explicit_2byte_pad {
        let mut buf: [u8; 4] = [0; 4];
        dataset.read_exact(&mut buf)?;
        if ts.is_big_endian() {
            u32::from_be_bytes(buf)
        } else {
            u32::from_le_bytes(buf)
        }
    } else {
        let mut buf: [u8; 2] = [0; 2];
        dataset.read_exact(&mut buf)?;
        if ts.is_big_endian() {
            u16::from_be_bytes(buf) as u32
        } else {
            u16::from_le_bytes(buf) as u32
        }
    };
    Ok(vl::from_value_length(value_length))
}

/// Parses elements to build a `DicomObject` to represent the parsed dataset as an in-memory tree.
/// Returns `None` if the parser's first element fails to parse properly, assumed to be a non-DICOM
/// dataset. Any errors after a successful first element being parsed are returned as `Result::Err`.
pub fn parse_into_object<'dict, DatasetType: Read>(
    parser: &mut Parser<'dict, DatasetType>,
) -> Result<Option<DicomRoot<'dict>>> {
    let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
    let mut items: Vec<DicomObject> = Vec::new();

    let parse_result: Option<Result<DicomElement>> =
        parse_into_object_recurse(parser, &mut child_nodes, &mut items, true);

    if !parser.behavior.allow_partial_object {
        if let Some(Err(e)) = parse_result {
            return Err(e);
        }
    }

    // If no child nodes and no items were parsed then this isn't valid dicom.
    if child_nodes.is_empty() && items.is_empty() {
        return Ok(None);
    }

    // Copy the parser state only after having parsed elements, to get appropriate transfer syntax
    // and specific character set.
    let root: DicomRoot<'_> = DicomRoot::new(
        parser.get_ts(),
        parser.get_cs(),
        parser.get_dictionary(),
        child_nodes,
        items,
    );
    Ok(Some(root))
}

/// Iterates through the parser populating values into the given `child_nodes` map. Elements which
/// are sequence-like (contain sub-elements) will be recursed into so child elements are added to
/// their node. The sequence path length is used to determine when parsing an element whether it
/// escapes the current level a of recursion, and how far back up it should go (the end of a
/// sequence can be the end of multiple sequences).
///
/// `parser` The parser elements are being read from
/// `child_nodes` The map of child nodes which elements should be parsed into
/// `items` The list of nodes which item elements should be parsed into
/// `is_root_level` Whether the root level is being parsed, or within child nodes
fn parse_into_object_recurse<DatasetType: Read>(
    parser: &mut Parser<'_, DatasetType>,
    child_nodes: &mut BTreeMap<u32, DicomObject>,
    items: &mut Vec<DicomObject>,
    is_root_level: bool,
) -> Option<Result<DicomElement>> {
    let mut prev_seq_path_len: usize = 0;
    let mut next_element: Option<Result<DicomElement>> = parser.next();

    // If the first element at the root level is an error then this is probably not valid dicom.
    if is_root_level {
        if let Some(Err(_)) = next_element {
            return None;
        }
    }

    while let Some(Ok(element)) = next_element {
        let tag: u32 = element.get_tag();
        let cur_seq_path_len: usize = element.get_sequence_path().len() + 1;

        if prev_seq_path_len == 0 {
            prev_seq_path_len = cur_seq_path_len;
        }

        if cur_seq_path_len < prev_seq_path_len {
            // If the next element has a shorter path than the previous one it should not be added
            // to the given node but returned so it can be added to a parent node.
            return Some(Ok(element));
        }

        let mut possible_next_elem: Option<Result<DicomElement>> = None;
        // Checking sequence or item tag should match dcmparser.read_dicom_element() which
        // does not read a value for those elements but lets the parser read its value as
        // separate elements which we're considering child elements.
        let dcmobj: DicomObject = if element.is_seq_like()
            || (tag == tags::ITEM && element.get_vl() != ValueLength::Explicit(0))
        {
            let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
            let mut items: Vec<DicomObject> = Vec::new();
            possible_next_elem =
                parse_into_object_recurse(parser, &mut child_nodes, &mut items, false);
            DicomObject::new_with_children(element, child_nodes, items)
        } else {
            DicomObject::new(element)
        };
        if tag == tags::ITEM {
            items.push(dcmobj);
        } else {
            child_nodes.insert(tag, dcmobj);
        }

        prev_seq_path_len = cur_seq_path_len;

        // If an element was returned from the recursive call that means the recursive call iterated
        // into an element which is not a child of the node we passed into the recursive call and
        // it should instead be added elsewhere up the tree.
        if let Some(Ok(next_elem)) = possible_next_elem {
            let next_seq_path_len: usize = next_elem.get_sequence_path().len() + 1;
            match next_seq_path_len {
                val if val < cur_seq_path_len => {
                    // If that element is still shorter than the current then it needs passed up further.
                    return Some(Ok(next_elem));
                }
                val if val == cur_seq_path_len => {
                    // If it matches the same level as the current node then it should be "inserted"
                    // into the element iteration so it will be checked for being sequence-like and
                    // then added into the current node.
                    next_element = Some(Ok(next_elem));
                    continue;
                }
                _ => {}
            }
        } else if let Some(Err(_)) = possible_next_elem {
            // Errors need propagated all the way back up.
            return possible_next_elem;
        }

        // Parse the next element from the dataset.
        next_element = parser.next();
    }

    // Return the last value from the parser which will either be None or Error.
    next_element
}
