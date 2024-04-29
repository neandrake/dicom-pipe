/*
   Copyright 2024 Christopher Speck

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

//! A buffered DICOM reader

#[cfg(feature = "compress")]
pub(crate) mod dataset {
    use libflate::deflate::Decoder;
    use std::io::{Read, Result};

    #[derive(Debug)]
    pub(crate) struct Dataset<R: Read> {
        decoder: Decoder<R>,
        read_deflated: bool,
    }

    impl<R: Read> Dataset<R> {
        pub fn new(dataset: R) -> Dataset<R> {
            Dataset {
                decoder: Decoder::new(dataset),
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
    use std::io::{Read, Result};

    #[derive(Debug)]
    pub(crate) struct Dataset<R: Read> {
        dataset: R,
    }

    impl<R: Read> Dataset<R> {
        pub fn new(dataset: R) -> Dataset<R> {
            Dataset { dataset }
        }
    }

    impl<R: Read> Read for Dataset<R> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.dataset.read(buf)
        }
    }
}
