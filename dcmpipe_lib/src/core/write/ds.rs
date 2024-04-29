//! A buffered DICOM writer

#[cfg(feature = "compress")]
pub(crate) mod dataset {
    use libflate::deflate::Encoder;
    use std::io::{Result, Write};

    #[derive(Debug)]
    pub(crate) struct Dataset<W: Write> {
        encoder: Encoder<W>,
        write_deflated: bool,
    }

    impl<W: Write> Dataset<W> {
        pub fn new(dataset: W) -> Dataset<W> {
            Dataset {
                encoder: Encoder::new(dataset),
                write_deflated: false,
            }
        }

        pub fn set_write_deflated(&mut self, write_deflated: bool) {
            self.write_deflated = write_deflated;
        }

        pub fn into_inner(self) -> W {
            self.encoder.into_inner()
        }
    }

    impl<W: Write> Write for Dataset<W> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.write_deflated {
                self.encoder.write(buf)
            } else {
                self.encoder.as_inner_mut().write(buf)
            }
        }

        fn flush(&mut self) -> Result<()> {
            if self.write_deflated {
                self.encoder.flush()
            } else {
                self.encoder.as_inner_mut().flush()
            }
        }
    }
}

#[cfg(not(feature = "compress"))]
pub(crate) mod dataset {
    use std::io::{Result, Write};

    #[derive(Debug)]
    pub(crate) struct Dataset<W: Write> {
        dataset: W,
    }

    impl<W: Write> Dataset<W> {
        pub fn new(dataset: W) -> Dataset<W> {
            Dataset { dataset }
        }

        pub fn into_inner(self) -> W {
            self.dataset
        }
    }

    impl<W: Write> Write for Dataset<W> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.dataset.write(buf)
        }

        fn flush(&mut self) -> Result<()> {
            self.dataset.flush()
        }
    }
}
