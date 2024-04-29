# dcmpipe_lib

This crate is the core DICOM reading/writing library. This crate is meant to be minimal with few dependencies. It does
not depend on the DICOM standard dictionary, instead able to parse through most dicom based only on knowing a minimal
set of DICOM constants.

This crate should eventually compile with `#[no_std]` and be compatible for WASM targets.