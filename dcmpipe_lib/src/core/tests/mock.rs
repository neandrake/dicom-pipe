use crate::core::dcmparser::DicomStreamParser;
use crate::core::tagstop::TagStop;
use crate::defn::tag::TagRef;
use crate::defn::ts::TSRef;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

pub static TAG_BY_VALUE: phf::Map<u32, TagRef> = phf::Map {
    key: 0,
    disps: ::phf::Slice::Static(&[]),
    entries: ::phf::Slice::Static(&[]),
};

pub static TS_BY_UID: phf::Map<&'static str, TSRef> = phf::Map {
    key: 0,
    disps: ::phf::Slice::Static(&[]),
    entries: ::phf::Slice::Static(&[]),
};

pub struct MockDicomStream {
    pub data: Vec<u8>,
    pub pos: usize,
}

impl MockDicomStream {
    fn create_parser(
        mockup: MockDicomStream,
        tagstop: TagStop,
    ) -> DicomStreamParser<MockDicomStream> {
        DicomStreamParser::new(mockup, tagstop, &TAG_BY_VALUE, &TS_BY_UID)
    }

    pub fn standard_dicom_preamble() -> DicomStreamParser<MockDicomStream> {
        let mockup: MockDicomStream = MockDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        };
        MockDicomStream::create_parser(mockup, TagStop::EndOfStream)
    }

    pub fn invalid_dicom_prefix() -> DicomStreamParser<MockDicomStream> {
        let mockup: MockDicomStream = MockDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = 'D' as u8;
                data[129] = 'O' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        };
        MockDicomStream::create_parser(mockup, TagStop::EndOfStream)
    }

    pub fn nonzero_preamble() -> DicomStreamParser<MockDicomStream> {
        let mockup: MockDicomStream = MockDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0xFFu8; 132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 0,
        };
        MockDicomStream::create_parser(mockup, TagStop::EndOfStream)
    }

    pub fn standard_dicom_preamble_diff_startpos_and_short_stream(
    ) -> DicomStreamParser<MockDicomStream> {
        let mockup: MockDicomStream = MockDicomStream {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = 'D' as u8;
                data[129] = 'I' as u8;
                data[130] = 'C' as u8;
                data[131] = 'M' as u8;
                data
            },
            pos: 131,
        };
        MockDicomStream::create_parser(mockup, TagStop::EndOfStream)
    }
}

impl Read for MockDicomStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.pos >= self.data.len() {
            return Result::Ok(0);
        }

        let mut count: usize = 0;
        for i in self.pos..self.data.len() {
            if count >= buf.len() {
                break;
            }
            buf[count] = self.data[i];
            count += 1;
        }
        self.pos = self.pos + count;
        Result::Ok(count)
    }
}

impl Seek for MockDicomStream {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        let newpos: usize = match pos {
            SeekFrom::Start(n) => 0usize.saturating_add(n as usize),
            SeekFrom::Current(n) => self.pos.saturating_add(n as usize),
            SeekFrom::End(n) => self.data.len().saturating_sub(n as usize),
        };

        if newpos < self.data.len() {
            self.pos = newpos;
            return Result::Ok(newpos as u64);
        }

        return Result::Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!("seek to invalid position: {:?}", newpos),
        ));
    }
}
