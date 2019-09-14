use dcmpipe_lib::core::dcmparser::{Parser, ParserBuilder};
use dcmpipe_lib::core::tagstop::TagStop;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

pub struct MockDicomDataset {
    pub data: Vec<u8>,
    pub pos: usize,
}

impl MockDicomDataset {
    fn create_parser(mockup: MockDicomDataset, tagstop: TagStop) -> Parser<MockDicomDataset> {
        ParserBuilder::new(mockup).tagstop(tagstop).build()
    }

    pub fn standard_dicom_preamble() -> Parser<MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
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
        MockDicomDataset::create_parser(mockup, TagStop::EndOfDataset)
    }

    pub fn invalid_dicom_prefix() -> Parser<MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
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
        MockDicomDataset::create_parser(mockup, TagStop::EndOfDataset)
    }

    pub fn nonzero_preamble() -> Parser<MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
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
        MockDicomDataset::create_parser(mockup, TagStop::EndOfDataset)
    }

    pub fn standard_dicom_preamble_diff_startpos_and_short_dataset() -> Parser<MockDicomDataset> {
        let mockup: MockDicomDataset = MockDicomDataset {
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
        MockDicomDataset::create_parser(mockup, TagStop::EndOfDataset)
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
