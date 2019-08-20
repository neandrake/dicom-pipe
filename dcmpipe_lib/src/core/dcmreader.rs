use crate::core::dcmelement::DicomElement;
use crate::core::dcmobject::DicomObject;
use crate::core::dcmparser::Parser;
use crate::defn::constants::tags;
use byteorder::ReadBytesExt;
use std::io::Error;

pub fn parse_stream<StreamType: ReadBytesExt>(
    parser: &mut Parser<StreamType>,
) -> Result<DicomObject, Error> {
    let mut root: DicomObject = DicomObject::new_root();
    if let Some(Err(e)) = parse_stream_recurse(parser, &mut root) {
        return Err(e);
    }
    Ok(root)
}

fn parse_stream_recurse<StreamType: ReadBytesExt>(
    parser: &mut Parser<StreamType>,
    parent: &mut DicomObject,
) -> Option<Result<DicomElement, Error>> {
    let mut prev_seq_path_len: usize = 0;
    let mut next_element: Option<Result<DicomElement, Error>> = parser.next();
    while let Some(Ok(element)) = next_element {
        let cur_seq_path_len: usize = element.get_sequence_path().len() + 1;
        if prev_seq_path_len == 0 {
            prev_seq_path_len = cur_seq_path_len;
        }
        if cur_seq_path_len < prev_seq_path_len {
            return Some(Ok(element));
        }

        let mut possible_next_elem: Option<Result<DicomElement, Error>> = None;
        // checking sequence or item tag should match dcmparser.read_dicom_element() which
        // does not read a value for those elements but lets the parser read its value as
        // separate elements which we're considering child elements.
        let dcmobj: DicomObject = if element.is_seq() || element.tag == tags::ITEM {
            let mut object: DicomObject = DicomObject::new_with_element(element);
            possible_next_elem = parse_stream_recurse(parser, &mut object);
            object
        } else {
            DicomObject::new_with_element(element)
        };
        if let Err(e) = parent.put_object(dcmobj) {
            return Some(Err(e));
        }

        prev_seq_path_len = cur_seq_path_len;

        if let Some(Ok(next_elem)) = possible_next_elem {
            let next_seq_path_len: usize = next_elem.get_sequence_path().len() + 1;
            if next_seq_path_len < cur_seq_path_len {
                return Some(Ok(next_elem));
            } else if next_seq_path_len == cur_seq_path_len {
                next_element = Some(Ok(next_elem));
                continue;
            }
        } else if let Some(Err(_)) = possible_next_elem {
            return possible_next_elem;
        }

        next_element = parser.next();
    }

    None
}
