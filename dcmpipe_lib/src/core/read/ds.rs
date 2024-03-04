//! A buffered DICOM reader

#[cfg(feature = "compress")]
pub(crate) mod dataset {
    use libflate::deflate::Decoder;
    use std::io::{BufReader, Read, Result};

    #[derive(Debug)]
    pub(crate) struct Dataset<R: Read> {
        decoder: Decoder<BufReader<R>>,
        read_deflated: bool,
    }

    impl<R: Read> Dataset<R> {
        pub fn new(dataset: R, buffsize: usize) -> Dataset<R> {
            Dataset {
                decoder: Decoder::new(BufReader::with_capacity(buffsize, dataset)),
                read_deflated: false,
            }
        }

        pub fn set_read_deflated(&mut self, read_deflated: bool) {
            self.read_deflated = read_deflated;
        }
    }

    impl<R: Read> Read for Dataset<R> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            if self.read_deflated {
                self.decoder.read(buf)
            } else {
                self.decoder.as_inner_mut().read(buf)
            }
        }
    }
}

#[cfg(not(feature = "compress"))]
pub(crate) mod dataset {
    use std::io::{BufReader, Read, Result};

    #[derive(Debug)]
    pub(crate) struct Dataset<R: Read> {
        dataset: BufReader<R>,
    }

    impl<R: Read> Dataset<R> {
        pub fn new(dataset: R, buffsize: usize) -> Dataset<R> {
            Dataset {
                dataset: BufReader::with_capacity(buffsize, dataset),
            }
        }
    }

    impl<R: Read> Read for Dataset<R> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.dataset.read(buf)
        }
    }
}
