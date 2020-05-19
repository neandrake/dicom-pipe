# dcmpipe_lib #

This crate is the core DICOM reading/writing library. This crate is focused only on basic reading/writing of DICOM
datasets and should have few dependencies. It does not depend on the DICOM standard dictionary, instead able to parse
through most dicom based only on knowing a minimal set of DICOM constants.

This crate should eventually compile with `#[no_std]` and be compatible for WASM targets.
