use quick_xml::Error as XmlError;
use quick_xml::events::{BytesText, Event};
use quick_xml::Reader;

use std::io::BufRead;

pub type XmlDicomDefinitionResult = Result<XmlDicomDefinition, XmlError>;

#[derive(Eq, PartialEq, Debug)]
pub enum XmlDicomDefinition {
    DicomElement(XmlDicomElement),
    FileMetaElement(XmlDicomElement),
    DirStructureElement(XmlDicomElement),
    Uid(XmlDicomUid),
    TransferSyntax(XmlDicomUid),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Serialize)]
pub struct XmlDicomElement {
    pub tag: String,
    pub name: String,
    pub keyword: String,
    pub vr: String,
    pub vm: String,
    pub obs: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Serialize)]
pub struct XmlDicomUid {
    pub value: String,
    pub name: String,
    pub uid_type: Option<String>,
    pub part: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomDefinitionTable {
    DicomElements,
    FileMetaElements,
    DirStructureElements,
    Uids,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomElementCell {
    CellTag,
    CellName,
    CellKeyword,
    CellVR,
    CellVM,
    CellObs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomUidCell {
    CellValue,
    CellName,
    CellType,
    CellPart,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomReadingState {
    Off,
    InTableHead,
    InTable,
    InDicomElementCell(XmlDicomElementCell),
    InDicomUidCell(XmlDicomUidCell),
}

pub struct XmlDicomDefinitionIterator<R: BufRead> {
    parser: Reader<R>,
    state: XmlDicomReadingState,

    table: XmlDicomDefinitionTable,

    element_tag: Option<String>,
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

    fn parse_text_bytes(&self, data: &BytesText) -> String {
        data.unescape_and_decode(&self.parser)
            .expect(&format!("Error parsing DICOM Entry Name: {:?}", data))
            .trim()
            .replace("\u{200b}", "")
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
        self.uid_value.is_some()
            && self.uid_name.is_some()
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
        loop {
            // TODO: Move buffer into a reusable field that gets cleared before each use here.
            // Currently unsure how to do this because it causes borrow problems.
            let mut buf: Vec<u8> = Vec::new();
            let res: Result<Event, XmlError> = self.parser.read_event(&mut buf);
            match res {
                Ok(Event::Start(ref e)) => {
                    let local_name: &[u8] = e.local_name();
                    match self.state {
                        XmlDicomReadingState::Off => if local_name == b"table" {
                            if let Some(xml_id_attr) = e.attributes()
                                .find(|attr| !attr.is_err() && attr.as_ref().unwrap().key == b"xml:id")
                                .map(|attr| attr.unwrap()) {
                                match xml_id_attr.value.as_ref() {
                                    b"table_6-1" => {
                                        self.table = XmlDicomDefinitionTable::DicomElements;
                                        self.state = XmlDicomReadingState::InTableHead;
                                    },
                                    b"table_7-1" => {
                                        self.table = XmlDicomDefinitionTable::FileMetaElements;
                                        self.state = XmlDicomReadingState::InTableHead;
                                    },
                                    b"table_8-1" => {
                                        self.table = XmlDicomDefinitionTable::DirStructureElements;
                                        self.state = XmlDicomReadingState::InTableHead;
                                    },
                                    b"table_A-1" => {
                                        self.table = XmlDicomDefinitionTable::Uids;
                                        self.state = XmlDicomReadingState::InTableHead;
                                    }
                                    _ => {}
                                }
                            }
                        },
                        XmlDicomReadingState::InTableHead => {
                            if local_name == b"tbody" {
                                self.state = XmlDicomReadingState::InTable;
                            }
                        },
                        XmlDicomReadingState::InTable => {
                            if local_name == b"para" {
                                match self.table {
                                    XmlDicomDefinitionTable::DicomElements | XmlDicomDefinitionTable::FileMetaElements | XmlDicomDefinitionTable::DirStructureElements => {
                                        self.state = XmlDicomReadingState::InDicomElementCell(XmlDicomElementCell::CellTag);
                                    },
                                    XmlDicomDefinitionTable::Uids => {
                                        self.state = XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::CellValue);
                                    },
                                    _ => {}
                                }
                            }
                        },
                        XmlDicomReadingState::InDicomElementCell(element_cell) => {
                            if local_name == b"para" {
                                self.state = match element_cell {
                                    XmlDicomElementCell::CellTag => XmlDicomReadingState::InDicomElementCell(XmlDicomElementCell::CellName),
                                    XmlDicomElementCell::CellName => XmlDicomReadingState::InDicomElementCell(XmlDicomElementCell::CellKeyword),
                                    XmlDicomElementCell::CellKeyword => XmlDicomReadingState::InDicomElementCell(XmlDicomElementCell::CellVR),
                                    XmlDicomElementCell::CellVR => XmlDicomReadingState::InDicomElementCell(XmlDicomElementCell::CellVM),
                                    XmlDicomElementCell::CellVM => XmlDicomReadingState::InDicomElementCell(XmlDicomElementCell::CellObs),
                                    XmlDicomElementCell::CellObs => XmlDicomReadingState::InTable,
                                };
                            }
                        },
                        XmlDicomReadingState::InDicomUidCell(uid_cell) => {
                            if local_name == b"para" {
                                self.state = match uid_cell {
                                    XmlDicomUidCell::CellValue => XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::CellName),
                                    XmlDicomUidCell::CellName => XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::CellType),
                                    XmlDicomUidCell::CellType => XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::CellPart),
                                    XmlDicomUidCell::CellPart => XmlDicomReadingState::InTable,
                                };
                            }
                        },
                    }
                }
                Ok(Event::End(ref e)) => {
                    let local_name = e.local_name();
                    match self.state {
                        XmlDicomReadingState::Off => {},
                        _ => if local_name == b"tr" {
                            if self.is_next_element_fully_read() {
                                let out = XmlDicomElement {
                                    tag: self.element_tag.take().unwrap(),
                                    name: self.element_name.take().unwrap(),
                                    keyword: self.element_keyword.take().unwrap(),
                                    vr: self.element_vr.take().unwrap(),
                                    vm: self.element_vm.take().unwrap(),
                                    obs: self.element_obs.take(),
                                };

                                self.clear_next();
                                self.state = XmlDicomReadingState::InTable;

                                match self.table {
                                    XmlDicomDefinitionTable::DicomElements => return Some(Ok(XmlDicomDefinition::DicomElement(out))),
                                    XmlDicomDefinitionTable::FileMetaElements => return Some(Ok(XmlDicomDefinition::FileMetaElement(out))),
                                    XmlDicomDefinitionTable::DirStructureElements => return Some(Ok(XmlDicomDefinition::DirStructureElement(out))),
                                    _ => {}
                                }
                            } else if self.is_next_uid_fully_read() {
                                let out = XmlDicomUid {
                                    value: self.uid_value.take().unwrap(),
                                    name: self.uid_name.take().unwrap(),
                                    uid_type: self.uid_type.take(),
                                    part: self.uid_part.take(),
                                };

                                self.clear_next();
                                self.state = XmlDicomReadingState::InTable;

                                match self.table {
                                    XmlDicomDefinitionTable::Uids => {
                                        // TODO: Is a clone necessary here?
                                        let type_clone: Option<String> = out.uid_type.clone();
                                        if type_clone.filter(|v| v == "Transfer Syntax").is_some() {
                                            return Some(Ok(XmlDicomDefinition::TransferSyntax(out)));
                                        }
                                        return Some(Ok(XmlDicomDefinition::Uid(out)));
                                    },
                                    _ => {}
                                }
                            }
                        } else if local_name == b"tbody" {
                            self.state = XmlDicomReadingState::Off;
                            self.table = XmlDicomDefinitionTable::Unknown;
                        },
                    }
                }
                Ok(Event::Text(data)) => match self.state {
                    XmlDicomReadingState::InDicomElementCell(element_cell) => {
                        match element_cell {
                            XmlDicomElementCell::CellTag => self.element_tag = Some(self.parse_text_bytes(&data)),
                            XmlDicomElementCell::CellName => self.element_name = Some(self.parse_text_bytes(&data)),
                            XmlDicomElementCell::CellKeyword => self.element_keyword = Some(self.parse_text_bytes(&data)),
                            XmlDicomElementCell::CellVR => self.element_vr = Some(self.parse_text_bytes(&data)),
                            XmlDicomElementCell::CellVM => self.element_vm = Some(self.parse_text_bytes(&data)),
                            XmlDicomElementCell::CellObs => self.element_obs = Some(self.parse_text_bytes(&data)),
                        }
                    }
                    XmlDicomReadingState::InDicomUidCell(uid_cell) => {
                        match uid_cell {
                            XmlDicomUidCell::CellValue => self.uid_value = Some(self.parse_text_bytes(&data)),
                            XmlDicomUidCell::CellName => self.uid_name = Some(self.parse_text_bytes(&data)),
                            XmlDicomUidCell::CellType => self.uid_type = Some(self.parse_text_bytes(&data)),
                            XmlDicomUidCell::CellPart => self.uid_part = Some(self.parse_text_bytes(&data)),
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof { .. }) => {
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }

        None
    }
}
