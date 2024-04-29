use dcmpipe_lib;

use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

use dcmpipe_lib::core::read::stop::ParseStop;
use dcmpipe_lib::core::read::{Parser, ParserBuilder};

pub struct MockDicomDataset {
    pub data: Vec<u8>,
    pub pos: usize,
}

impl MockDicomDataset {
    fn create_parser(
        mockup: MockDicomDataset,
        tagstop: ParseStop,
    ) -> Parser<'static, MockDicomDataset> {
        ParserBuilder::default().stop(tagstop).build(mockup)
    }

    pub fn standard_dicom_preamble() -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = u8::try_from('D').unwrap_or_default();
                data[129] = u8::try_from('I').unwrap_or_default();
                data[130] = u8::try_from('C').unwrap_or_default();
                data[131] = u8::try_from('M').unwrap_or_default();
                data
            },
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn invalid_dicom_prefix() -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = u8::try_from('D').unwrap_or_default();
                data[129] = u8::try_from('O').unwrap_or_default();
                data[130] = u8::try_from('C').unwrap_or_default();
                data[131] = u8::try_from('M').unwrap_or_default();
                data
            },
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn nonzero_preamble() -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0xFFu8; 132];
                data[128] = u8::try_from('D').unwrap_or_default();
                data[129] = u8::try_from('I').unwrap_or_default();
                data[130] = u8::try_from('C').unwrap_or_default();
                data[131] = u8::try_from('M').unwrap_or_default();
                data
            },
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn standard_dicom_preamble_diff_startpos_and_short_dataset(
    ) -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: {
                let mut data: Vec<u8> = vec![0u8; 132];
                data[128] = u8::try_from('D').unwrap_or_default();
                data[129] = u8::try_from('I').unwrap_or_default();
                data[130] = u8::try_from('C').unwrap_or_default();
                data[131] = u8::try_from('M').unwrap_or_default();
                data
            },
            pos: 131,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }

    pub fn build_mock_parser(element_bytes: &[&[u8]]) -> Parser<'static, MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
            data: element_bytes.concat(),
            pos: 0,
        };
        MockDicomDataset::create_parser(mockup, ParseStop::EndOfDataset)
    }
}

impl Read for MockDicomDataset {
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

impl Seek for MockDicomDataset {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        let newpos: usize = match pos {
            SeekFrom::Start(n) => usize::try_from(n).unwrap_or(usize::MAX),
            SeekFrom::Current(n) => self
                .pos
                .saturating_add(usize::try_from(n).unwrap_or(usize::MAX)),
            SeekFrom::End(n) => self
                .data
                .len()
                .saturating_sub(usize::try_from(n).unwrap_or(usize::MAX)),
        };

        if newpos < self.data.len() {
            self.pos = newpos;
            return Result::Ok(u64::try_from(newpos).unwrap_or_default());
        }

        Result::Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!("seek to invalid position: {:?}", newpos),
        ))
    }
}
