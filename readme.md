# dcmpipe
A set of crates for reading and writing DICOM.

#### Goals

- The core library should be small/minimal with few dependencies. It should be possible to target WASM and create a small lean library for use in web pages.
- The core library should support DICOM files as well as DICOM network streams.
- Allow building private tag dictionaries with `dcmpipe_dict_builder` and using them with the core library.
- Provide basic command-line utilities for inspecting DICOM files and communicating with DICOM networks.

#### Crates

- `dcmpipe_lib`: The core library of definitions for the DICOM format and ability to read/write dicom.
- `dcmpipe_dict_builder`: Functionality for parsing the dicom standard and output tags and uids along with map lookups using `phf`. This is intended to be used by `build.rs` scripts.
- `dcmpipe_dict`: The standard dicom tags/uids generated using `dcmpipe_dict_builder`.
- `dcmpipe_cli`: Several examples of parsing dicom streams to display the contents.
- `dcmpipe_archive`: Command-line tool for managing DICOM archives

{F19846}

#### Milestone v1.0 

- [ ] {T10} - Implement appropriate error handling for a library
- [ ] {T11} Add support for logging
- [ ] Conform to [[ https://rust-lang.github.io/api-guidelines/checklist.html | Rust's API Guidelines ]]
- [ ] Add support for writing DICOM streams
- [ ] Add support for DICOM network connectivity
