//! A buffered DICOM writer

#[cfg(feature = "deflate")]
pub(crate) mod dataset {
    use libflate::deflate::Encoder;
    use std::io::{BufWriter, Result, Write, IntoInnerError};

    pub(crate) struct Dataset<DatasetType: Write> {
        encoder: Encoder<BufWriter<DatasetType>>,
        write_deflated: bool,
    }

    impl<DatasetType: Write> Dataset<DatasetType> {
        pub fn new(dataset: DatasetType, buffsize: usize) -> Dataset<DatasetType> {
            Dataset {
                encoder: Encoder::new(BufWriter::with_capacity(buffsize, dataset)),
                write_deflated: false,
            }
        }

        pub fn set_write_deflated(&mut self, write_deflated: bool) {
            self.write_deflated = write_deflated;
        }

        pub fn into_inner(self) -> std::result::Result<DatasetType, IntoInnerError<BufWriter<DatasetType>>> {
            self.encoder.into_inner().into_inner()
        }
    }

    impl<DatasetType: Write> Write for Dataset<DatasetType> {
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

#[cfg(not(feature = "deflate"))]
pub(crate) mod dataset {
    use std::io::{BufWriter, Result, Write};

    pub(crate) struct Dataset<DatasetType: Write> {
        dataset: BufWriter<DatasetType>,
    }

    impl<DatasetType: Write> Dataset<DatasetType> {
        pub fn new(dataset: DatasetType, buffsize: usize) -> Dataset<DatasetType> {
            Dataset {
                dataset: BufWriter::with_capacity(buffsize, dataset),
            }
        }

        pub fn into_inner(self) -> std::result::Result<DatasetType, IntoInnerError<DatasetType>> {
            self.dataset.into_inner()
        }
    }

    impl<DatasetType: Write> Write for Dataset<DatasetType> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.dataset.write(buf)
        }

        fn flush(&mut self) -> Result<()> {
            self.dataset.flush()
        }
    }
}
