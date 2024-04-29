//! A buffered DICOM reader

#[cfg(feature = "deflate")]
pub(crate) mod dataset {
    use libflate::deflate::Decoder;
    use std::io::{BufReader, Read, Result};

    pub(crate) struct Dataset<DatasetType: Read> {
        deflated: Decoder<BufReader<DatasetType>>,
        read_deflated: bool,
    }

    impl<DatasetType: Read> Dataset<DatasetType> {
        pub fn new(dataset: DatasetType, buffsize: usize) -> Dataset<DatasetType> {
            Dataset {
                deflated: Decoder::new(BufReader::with_capacity(buffsize, dataset)),
                read_deflated: false,
            }
        }

        pub fn set_read_deflated(&mut self, read_deflated: bool) {
            self.read_deflated = read_deflated;
        }
    }

    impl<DatasetType: Read> Read for Dataset<DatasetType> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            if self.read_deflated {
                self.deflated.read(buf)
            } else {
                self.deflated.as_inner_mut().read(buf)
            }
        }
    }
}

#[cfg(not(feature = "deflate"))]
pub(crate) mod dataset {
    use std::io::{BufReader, Read, Result};

    pub(crate) struct Dataset<DatasetType: Read> {
        dataset: BufReader<DatasetType>,
    }

    impl<DatasetType: Read> Dataset<DatasetType> {
        pub fn new(dataset: DatasetType, buffsize: usize) -> Dataset<DatasetType> {
            Dataset {
                dataset: BufReader::with_capacity(buffsize, dataset),
            }
        }
    }

    impl<DatasetType: Read> Read for Dataset<DatasetType> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.dataset.read(buf)
        }
    }
}
