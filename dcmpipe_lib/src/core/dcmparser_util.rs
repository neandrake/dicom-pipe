use crate::core::dcmelement::DicomElement;
use crate::core::dcmobject::{DicomObject, DicomRoot};
use crate::core::dcmparser::Parser;
use crate::defn::constants::tags;
use crate::defn::vl;
use crate::defn::vl::ValueLength;
use crate::defn::vr::{self, VRRef, VR};
use std::collections::BTreeMap;
use std::io::{Error, Read};

/// Whether the element is a non-standard parent-able element. These are non-SQ, non-ITEM elements
/// with a VR of `UN`, `OB`, or `OW` and have a value length of `UndefinedLength`. These types of
/// elements are considered either private-tag sequences or otherwise whose contents are encoded
/// as IVRLE.
pub fn is_non_standard_seq(tag: u32, vr: VRRef, vl: ValueLength) -> bool {
    tag != tags::ITEM
        && (vr == &vr::UN || vr == &vr::OB || vr == &vr::OW)
        && vl == ValueLength::UndefinedLength
}

/// Reads a tag attribute from a given dataset
pub fn read_tag_from_dataset(dataset: &mut impl Read, big_endian: bool) -> Result<u32, Error> {
    let mut buf: [u8; 2] = [0; 2];

    dataset.read_exact(&mut buf)?;
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
pub fn read_vr_from_dataset(dataset: &mut impl Read) -> Result<Option<VRRef>, Error> {
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
        None => return Ok(None),
    };

    Ok(Some(vr))
}

/// Reads a Value Length from a given dataset.
/// `dataset` The dataset to read bytes from
/// `read_4bytes` Whether 4 bytes or 2 bytes should be read from the dataset. Refer to the dicom
///                 standard -- implicit vr transfer syntax uses 4 bytes for value length, explicit
///                 vr uses 2 bytes, but if explicit and the VR has 2-byte padding then 4 bytes
///                 should be parsed.
/// `big_endian` Whether to use big or little endian
pub fn read_value_length_from_dataset(
    dataset: &mut impl Read,
    read_4bytes: bool,
    big_endian: bool,
) -> Result<ValueLength, Error> {
    let value_length: u32 = if read_4bytes {
        let mut buf: [u8; 4] = [0; 4];
        dataset.read_exact(&mut buf)?;

        if big_endian {
            u32::from_be_bytes(buf)
        } else {
            u32::from_le_bytes(buf)
        }
    } else {
        let mut buf: [u8; 2] = [0; 2];
        dataset.read_exact(&mut buf)?;

        if big_endian {
            u32::from(u16::from_be_bytes(buf))
        } else {
            u32::from(u16::from_le_bytes(buf))
        }
    };
    Ok(vl::from_value_length(value_length))
}

/// Parses elements to build a `DicomObject` tree to represent the dataset in-memory
pub fn parse_into_object<'dict, DatasetType: Read>(
    parser: &mut Parser<'dict, DatasetType>,
) -> Result<DicomRoot<'dict>, Error> {
    let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
    let mut items: Vec<DicomObject> = Vec::new();
    if let Some(Err(e)) = parse_into_object_recurse(parser, &mut child_nodes, &mut items) {
        return Err(e);
    }
    // Copy the parser state only after having parsed elements, to get appropriate transfer syntax
    // and specfic character set.
    let root: DicomRoot<'_> = DicomRoot::new(
        parser.get_ts(),
        parser.get_cs(),
        parser.get_dictionary(),
        child_nodes,
    );
    Ok(root)
}

/// Iterates through the parser populating values into the given `nodes` map. Elements which are
/// sequence-like (contain sub-elements) will be recursed into so child elements are added to their
/// node. The sequence path length is used to determine when parsing an element whether it escapes
/// the current level a of recursion, and how far back up it should go (the end of a sequence can
/// be the end of multiple sequences).
/// `parser` The parser elements are being read from
/// `nodes` The map of nodes which elements should be parsed into
fn parse_into_object_recurse<DatasetType: Read>(
    parser: &mut Parser<'_, DatasetType>,
    child_nodes: &mut BTreeMap<u32, DicomObject>,
    items: &mut Vec<DicomObject>,
) -> Option<Result<DicomElement, Error>> {
    let mut prev_seq_path_len: usize = 0;
    let mut next_element: Option<Result<DicomElement, Error>> = parser.next();
    while let Some(Ok(element)) = next_element {
        let tag: u32 = element.tag;
        let cur_seq_path_len: usize = element.get_sequence_path().len() + 1;

        if prev_seq_path_len == 0 {
            prev_seq_path_len = cur_seq_path_len;
        }

        if cur_seq_path_len < prev_seq_path_len {
            // if the next element has a shorter path than the previous one it should not be added
            // to the given node but returned so it can be added to a parent node.
            return Some(Ok(element));
        }

        let mut possible_next_elem: Option<Result<DicomElement, Error>> = None;
        // checking sequence or item tag should match dcmparser.read_dicom_element() which
        // does not read a value for those elements but lets the parser read its value as
        // separate elements which we're considering child elements.
        let dcmobj: DicomObject = if element.is_seq_like() || tag == tags::ITEM {
            let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
            let mut items: Vec<DicomObject> = Vec::new();
            possible_next_elem = parse_into_object_recurse(parser, &mut child_nodes, &mut items);
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

        // if an element was returned from the recursive call that means the recursive call iterated
        // into an element which is not a child of the node we passed into the recursive call and
        // it should instead be added elsewhere up the tree.
        if let Some(Ok(next_elem)) = possible_next_elem {
            let next_seq_path_len: usize = next_elem.get_sequence_path().len() + 1;
            if next_seq_path_len < cur_seq_path_len {
                // if that element is still shorter than the current then it needs passed up further
                return Some(Ok(next_elem));
            } else if next_seq_path_len == cur_seq_path_len {
                // if it matches the same level as the current node then it should be "inserted"
                // into the element iteration so it will be checked for being sequence-like and
                // then added into the current node.
                next_element = Some(Ok(next_elem));
                continue;
            }
        } else if let Some(Err(_)) = possible_next_elem {
            // errors need propagated all the way back up
            return possible_next_elem;
        }

        // parse the next element from the dataset
        next_element = parser.next();
    }

    // return the last value from the parser which will either be None or Error
    next_element
}
