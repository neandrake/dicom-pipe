//! Parses the XML format of Part 6 of the DICOM Standard.
//!
//! The XML document is fairly large so this uses quick-xml as a SAX-style XML parser, resulting in
//! efficient processing of the XML document but with added complexity. To use properly the
//! structure of the expected document needs to be tracked as quick-xml emits events from the
//! document.

use std::io::BufRead;

use quick_xml::events::{BytesText, Event};
use quick_xml::name::{LocalName, QName};
use quick_xml::Error as XmlError;
use quick_xml::Reader;

pub type XmlDicomDefinitionResult = Result<XmlDicomDefinition, XmlError>;

/// The different types of objects parsed out of the xml document.
#[derive(Eq, PartialEq, Debug)]
pub enum XmlDicomDefinition {
    DicomElement(XmlDicomElement),
    FileMetaElement(XmlDicomElement),
    DirStructureElement(XmlDicomElement),
    Uid(XmlDicomUid),
    TransferSyntax(XmlDicomUid),
}

/// The contents of a DICOM Element defined in the xml document.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct XmlDicomElement {
    pub tag: u32,
    pub name: String,
    pub keyword: String,
    pub vr: String,
    pub vm: String,
    pub obs: Option<String>,
}

/// The contents of a UID defined in the xml document.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct XmlDicomUid {
    pub value: String,
    pub name: String,
    pub uid_type: Option<String>,
    pub part: Option<String>,
}


/// The different table structures that appear in the xml document. This is used to track the state
/// of where the parser is currently located.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomDefinitionTable {
    DicomElements,
    FileMetaElements,
    DirStructureElements,
    Uids,
    Unknown,
}

/// The cells within the DICOM Element table. This is used to track the state of where the parser
/// is currently located after traversing into `XmlDicomDefinitionTable::DicomElements`,
/// `FileMetaElements`, or `DirStructureElements`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomElementCell {
    Tag,
    Name,
    Keyword,
    VR,
    VM,
    Obs,
}

/// The structure of the DICOM UID table. This is used to track the state of where the parser is
/// currently located after traversing into `XmlDicomDefinitionTable::Uids`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomUidCell {
    Value,
    Name,
    Type,
    Part,
}

/// These are additional transient traversal states used when traversing through the different
/// tables and cells.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomReadingState {
    Off,
    InTable,
    InTableBody,
    InDicomElementCell(XmlDicomElementCell),
    InDicomUidCell(XmlDicomUidCell),
}

pub struct XmlDicomDefinitionIterator<R: BufRead> {
    parser: Reader<R>,
    state: XmlDicomReadingState,

    table: XmlDicomDefinitionTable,

    element_tag: Option<u32>,
    element_name: Option<String>,
    element_keyword: Option<String>,
    element_vr: Option<String>,
    element_vm: Option<String>,
    element_obs: Option<String>,

    uid_value: Option<String>,
    uid_name: Option<String>,
    uid_type: Option<String>,
    uid_part: Option<String>,
}

impl<R: BufRead> XmlDicomDefinitionIterator<R> {
    pub fn new(xml: R) -> XmlDicomDefinitionIterator<R> {
        let mut reader = Reader::from_reader(xml);
        reader.expand_empty_elements(true).trim_text(true);
        XmlDicomDefinitionIterator {
            parser: reader,
            state: XmlDicomReadingState::Off,

            table: XmlDicomDefinitionTable::Unknown,

            element_tag: None,
            element_name: None,
            element_keyword: None,
            element_vr: None,
            element_vm: None,
            element_obs: None,

            uid_value: None,
            uid_name: None,
            uid_type: None,
            uid_part: None,
        }
    }

    fn parse_text_bytes(&self, data: &BytesText<'_>) -> String {
        data.unescape()
            .unwrap_or_else(|err| panic!("Error parsing DICOM Entry Name: {:?}\n\t{:?}", data, err))
            .trim()
            .replace('\u{200b}', "")
    }

    fn parse_text_bytes_as_u32(&self, data: &BytesText<'_>) -> Option<u32> {
        u32::from_str_radix(
            &self.parse_text_bytes(data).replace(['(', ')', ','], ""),
            16,
        ).ok()
    }

    fn is_next_element_fully_read(&self) -> bool {
        // observation may not have content
        self.element_tag.is_some()
            && self.element_name.is_some()
            && self.element_keyword.is_some()
            && self.element_vr.is_some()
            && self.element_vm.is_some()
    }

    fn is_next_uid_fully_read(&self) -> bool {
        // type and part may not have content
        self.uid_value.is_some() && self.uid_name.is_some()
    }

    fn clear_next(&mut self) {
        self.element_tag = None;
        self.element_name = None;
        self.element_keyword = None;
        self.element_vr = None;
        self.element_vm = None;
        self.element_obs = None;

        self.uid_value = None;
        self.uid_name = None;
        self.uid_type = None;
        self.uid_part = None;
    }
}

impl<R: BufRead> Iterator for XmlDicomDefinitionIterator<R> {
    type Item = XmlDicomDefinitionResult;

    fn next(&mut self) -> Option<XmlDicomDefinitionResult> {
        let mut buf: Vec<u8> = Vec::new();
        loop {
            buf.clear();
            let res: Result<Event<'_>, XmlError> = self.parser.read_event_into(&mut buf);
            match res {
                // The start of new elements (e.g. "<table>" instead of "</table>") are used to
                // keep the state machine (`state` and `table` fields) updated to track what the
                // data we can expect to encounter.
                Ok(Event::Start(ref e)) => {
                    let local_name: LocalName = e.local_name();
                    match self.state {
                        // If we're currently in the Off state then that means we've just started
                        // parsing or outside of tables so we should first expect to hit a "table"
                        // element.
                        XmlDicomReadingState::Off => {
                            if local_name == QName(b"table").into() {
                                // The tables that we're interested in parsing have a "xml:id"
                                // attribute identifying which table this is for.
                                if let Some(xml_id_attr) = e
                                    .attributes()
                                    .find(|attr| {
                                        !attr.is_err()
                                            && attr.as_ref().unwrap().key == QName(b"xml:id")
                                    })
                                    .map(|attr| attr.unwrap())
                                {
                                    // Flip state of the table we're entering so we expected to
                                    // encounter the cells for the expected structure.
                                    match xml_id_attr.value.as_ref() {
                                        b"table_6-1" => {
                                            self.table = XmlDicomDefinitionTable::DicomElements;
                                            self.state = XmlDicomReadingState::InTable;
                                        },
                                        b"table_7-1" => {
                                            self.table = XmlDicomDefinitionTable::FileMetaElements;
                                            self.state = XmlDicomReadingState::InTable;
                                        },
                                        b"table_8-1" => {
                                            self.table =
                                                XmlDicomDefinitionTable::DirStructureElements;
                                            self.state = XmlDicomReadingState::InTable;
                                        },
                                        b"table_A-1" => {
                                            self.table = XmlDicomDefinitionTable::Uids;
                                            self.state = XmlDicomReadingState::InTable;
                                        },
                                        // Unknown table, set the initial state.
                                        _ => {
                                            self.table = XmlDicomDefinitionTable::Unknown;
                                            self.state = XmlDicomReadingState::Off;
                                        },
                                    }
                                }
                            }
                        },
                        // We've previously entered a "table" we wish to parse further contents of.
                        XmlDicomReadingState::InTable => {
                            // Flip state to being in a table body once we've passed the "thead"
                            // element.
                            if local_name == QName(b"tbody").into() {
                                self.state = XmlDicomReadingState::InTableBody;
                            }
                        },
                        // We've previously entered a "tbody" for a relevant table, based on the
                        // table we've entered update the state for the expected cell to first
                        // encounter.
                        XmlDicomReadingState::InTableBody => {
                            if local_name == QName(b"para").into() {
                                match self.table {
                                    XmlDicomDefinitionTable::DicomElements
                                    | XmlDicomDefinitionTable::FileMetaElements
                                    | XmlDicomDefinitionTable::DirStructureElements => {
                                        // The first cell in these tables is the Tag number.
                                        self.state = XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Tag,
                                        );
                                    },
                                    XmlDicomDefinitionTable::Uids => {
                                        // The first cell in this table is the Value (actual UID).
                                        self.state = XmlDicomReadingState::InDicomUidCell(
                                            XmlDicomUidCell::Value,
                                        );
                                    },
                                    // We're in an unknown state, bounce the state machine back out
                                    // to top-level. The rest of this table will be ignored but
                                    // that's probably the only reasonable thing to do.
                                    _ => {
                                        self.table = XmlDicomDefinitionTable::Unknown;
                                        self.state = XmlDicomReadingState::Off;
                                    },
                                }
                            }
                        },
                        // We've previously entered into a table structured for DICOM elements.
                        XmlDicomReadingState::InDicomElementCell(element_cell) => {
                            // The contents of the "td" element are a "para" element, whose text
                            // content is the value of the cell we're interested in parsing.
                            if local_name == QName(b"para").into() {
                                // Set the next expected cell to encounter based on the current.
                                self.state = match element_cell {
                                    XmlDicomElementCell::Tag => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Name,
                                        )
                                    },
                                    XmlDicomElementCell::Name => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Keyword,
                                        )
                                    },
                                    XmlDicomElementCell::Keyword => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::VR,
                                        )
                                    },
                                    XmlDicomElementCell::VR => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::VM,
                                        )
                                    },
                                    XmlDicomElementCell::VM => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Obs,
                                        )
                                    },
                                    // The last cell in the table, flip back to being in the table
                                    // body so the next traversal would jump into the first cell
                                    // for this structure again.
                                    XmlDicomElementCell::Obs => XmlDicomReadingState::InTableBody,
                                };
                            }
                        },
                        // We've previously entered into a table structured for DICOM UIDs.
                        XmlDicomReadingState::InDicomUidCell(uid_cell) => {
                            if local_name == QName(b"para").into() {
                                self.state = match uid_cell {
                                    XmlDicomUidCell::Value => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Name)
                                    },
                                    XmlDicomUidCell::Name => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Type)
                                    },
                                    XmlDicomUidCell::Type => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Part)
                                    },
                                    // The last cell in the table, flip back to being in the table
                                    // body so the next traversal would jump into the first cell
                                    // for this structure again.
                                    XmlDicomUidCell::Part => XmlDicomReadingState::InTableBody,
                                };
                            }
                        }
                    }
                },

                // Events for element-ends (e.g. "</tr>" and not "<tr>") are used to check if
                // enough data was extracted and structured based on the expected format.
                Ok(Event::End(ref e)) => {
                    let local_name: LocalName = e.local_name();
                    match self.state {
                        // Unexpected state.
                        XmlDicomReadingState::Off => {},

                        _ => {
                            // If a "tr" element ended check to see if all necessary fields for a
                            // DICOM Element or UID were extracted.
                            if local_name == QName(b"tr").into() {
                                if self.is_next_element_fully_read() {
                                    let out = XmlDicomElement {
                                        tag: self.element_tag.take().unwrap(),
                                        name: self.element_name.take().unwrap(),
                                        keyword: self.element_keyword.take().unwrap(),
                                        vr: self.element_vr.take().unwrap(),
                                        vm: self.element_vm.take().unwrap(),
                                        obs: self.element_obs.take(),
                                    };

                                    // Clear out all cell data extracted and put back into a state
                                    // of being in the table body, ready for the next row.
                                    self.clear_next();
                                    self.state = XmlDicomReadingState::InTableBody;

                                    // For the iterator, return the item extracted, typed based on
                                    // the current table being parsed.
                                    match self.table {
                                        XmlDicomDefinitionTable::DicomElements => {
                                            return Some(Ok(XmlDicomDefinition::DicomElement(out)))
                                        },
                                        XmlDicomDefinitionTable::FileMetaElements => {
                                            return Some(Ok(XmlDicomDefinition::FileMetaElement(
                                                out,
                                            )))
                                        },
                                        XmlDicomDefinitionTable::DirStructureElements => {
                                            return Some(Ok(
                                                XmlDicomDefinition::DirStructureElement(out),
                                            ))
                                        },
                                        // Unexpected state.
                                        _ => {}
                                    }
                                } else if self.is_next_uid_fully_read() {
                                    let out = XmlDicomUid {
                                        value: self.uid_value.take().unwrap(),
                                        name: self.uid_name.take().unwrap(),
                                        uid_type: self.uid_type.take(),
                                        part: self.uid_part.take(),
                                    };

                                    // Clear out all cell data extracted and put back into a state
                                    // of being in the table body, ready for the next row.
                                    self.clear_next();
                                    self.state = XmlDicomReadingState::InTableBody;

                                    // For the iterator, return the item extracted, typed based on
                                    // the current table being parsed.
                                    #[allow(clippy::single_match)]
                                    match self.table {
                                        // The UID table lumps all UIDs together, but here the
                                        // Transfer Syntaxes are distinguished separately.
                                        XmlDicomDefinitionTable::Uids => {
                                            if out.uid_type.as_ref()
                                                .filter(|v| "Transfer Syntax".eq(*v))
                                                .is_some()
                                            {
                                                return Some(Ok(
                                                    XmlDicomDefinition::TransferSyntax(out),
                                                ));
                                            }
                                            return Some(Ok(XmlDicomDefinition::Uid(out)));
                                        },
                                        // Unexpected state.
                                        _ => {}
                                    }
                                }
                                // If a row ended and not all fields were filled in then clear all
                                // the fields to avoid polluting the next row.
                                eprintln!("Bad row? {:?} - {:?}", self.element_tag, self.element_name);
                                self.clear_next();
                            } else if local_name == QName(b"tbody").into() {
                                // If the "table" element ended bump the state back out into
                                // detecting the next table being parsed.
                                self.state = XmlDicomReadingState::Off;
                                self.table = XmlDicomDefinitionTable::Unknown;
                            }
                        }
                    }
                },

                // When Text content occurs the data is extracted and set based on the current
                // state machine.
                Ok(Event::Text(data)) => match self.state {
                    XmlDicomReadingState::InDicomElementCell(element_cell) => match element_cell {
                        XmlDicomElementCell::Tag => {
                            self.element_tag = self.parse_text_bytes_as_u32(&data)
                        },
                        XmlDicomElementCell::Name => {
                            self.element_name = Some(self.parse_text_bytes(&data))
                        },
                        XmlDicomElementCell::Keyword => {
                            self.element_keyword = Some(self.parse_text_bytes(&data))
                        },
                        XmlDicomElementCell::VR => {
                            self.element_vr = Some(self.parse_text_bytes(&data))
                        },
                        XmlDicomElementCell::VM => {
                            self.element_vm = Some(self.parse_text_bytes(&data))
                        },
                        XmlDicomElementCell::Obs => {
                            self.element_obs = Some(self.parse_text_bytes(&data))
                        },
                    },
                    XmlDicomReadingState::InDicomUidCell(uid_cell) => match uid_cell {
                        XmlDicomUidCell::Value => {
                            self.uid_value = Some(self.parse_text_bytes(&data))
                        },
                        XmlDicomUidCell::Name => self.uid_name = Some(self.parse_text_bytes(&data)),
                        XmlDicomUidCell::Type => self.uid_type = Some(self.parse_text_bytes(&data)),
                        XmlDicomUidCell::Part => self.uid_part = Some(self.parse_text_bytes(&data)),
                    },
                    _ => {},
                },
                Ok(Event::Eof { .. }) => {
                    break;
                },
                Ok(_) => {},
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }

        None
    }
}
