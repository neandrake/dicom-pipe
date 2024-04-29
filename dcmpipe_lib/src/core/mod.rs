pub mod charset;
pub mod dcmelement;
pub mod dcmobject;
pub mod dcmsqelem;
pub mod read;
pub mod write;
pub mod values;

pub use values::Attribute;
pub use values::RawValue;

/// The DICOM Prefix that should follow File Preambles.
pub static DICOM_PREFIX: &[u8; DICOM_PREFIX_LENGTH] = b"DICM";

/// The length of a DICOM Prefix.
pub const DICOM_PREFIX_LENGTH: usize = 4;

/// The length of a File Preamble.
pub const FILE_PREAMBLE_LENGTH: usize = 128;
