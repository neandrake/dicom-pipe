/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

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
