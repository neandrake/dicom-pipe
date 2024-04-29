# dcmpipe #

A set of crates for reading and writing DICOM.

![Screenshot](mdassets/screenshot.png "Screenshot")

## Goals ##
- The core library should be small/minimal with few dependencies. It should be possible to target WASM and create a small lean library for use in web pages.
- The core library should support DICOM files for valid transfer media and transfer syntax.
- Optional support for DICOM message exchange, not required by the core library.
- Optional support for the standard DICOM dictionary, not required by the core library.
- Importing of customized dictionaries with private tag entries.

## Crates ##
- `dcmpipe_cli`: Command-line tools utilizing the core library.
- `dcmpipe_dict_builder`: Parses the DICOM Standard XML files for producting the standard DICOM dictionary. This is intended to be used by `build.rs` scripts.
- `dcmpipe_lib`: The core library, supporting read and write of DICOM data sets.

## Milestone v1.0 ##
- [ ] Conform to [Rust's API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html)
- [x] Add support for writing DICOM streams
- [ ] Add support for DICOM network connectivity
