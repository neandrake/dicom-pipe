#[rustfmt::skip]

pub const STANDARD_HEADER: &'static [u8] = &[
    // Preamble: 128 null bytes
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

    // Prefix: DICM
    0x44, 0x49, 0x43, 0x4D,

    // FileMetaInformationGroupLength   VR: UL      VL: 4
    0x02, 0x00, 0x00, 0x00,             0x55, 0x4C, 0x04, 0x00,
    0xE2, 0x00, 0x00, 0x00,

    // FileMetaInformationVersion VR: OB      VL: 2
    0x02, 0x00, 0x01, 0x00,       0x4F, 0x42, 0x00, 0x00, 0x02, 0x00,
    0x00, 0x00, 0x00, 0x01,

    // MediaStorageSOPClassUID    VR: UI      VL: 26
    0x02, 0x00, 0x02, 0x00,       0x55, 0x49, 0x1A, 0x00,
    0x31, 0x2E, 0x32, 0x2E, 0x38, 0x34, 0x30, 0x2E, 0x31, 0x30, 0x30, 0x30, 0x38, 0x2E, 0x35, 0x2E,
    0x31, 0x2E, 0x34, 0x2E, 0x31, 0x2E, 0x31, 0x2E, 0x32, 0x00,

    // MediaStorageSOPInstanceUID VR: UI      VL: 52
    0x02, 0x00, 0x03, 0x00,       0x55, 0x49, 0x34, 0x00,
    0x31, 0x2E, 0x32, 0x2E, 0x32, 0x37, 0x36, 0x2E, 0x30, 0x2E, 0x37, 0x32, 0x33, 0x30, 0x30, 0x31,
    0x30, 0x2E, 0x33, 0x2E, 0x31, 0x2E, 0x34, 0x2E, 0x31, 0x37, 0x38, 0x37, 0x32, 0x30, 0x35, 0x34,
    0x32, 0x38, 0x2E, 0x32, 0x33, 0x34, 0x35, 0x2E, 0x31, 0x30, 0x37, 0x31, 0x30, 0x34, 0x38, 0x31,
    0x34, 0x36, 0x2E, 0x31,

    // TransferSyntaxUID    VR: UI      VL: 20
    0x02, 0x00, 0x10, 0x00, 0x55, 0x49, 0x14, 0x00,
    0x31, 0x2E, 0x32, 0x2E, 0x38, 0x34, 0x30, 0x2E, 0x31, 0x30, 0x30, 0x30, 0x38, 0x2E, 0x31, 0x2E,
    0x32, 0x2E, 0x35, 0x00,

    // ImplementationClassUID     VR: UI      VL: 48
    0x02, 0x00, 0x12, 0x00,       0x55, 0x49, 0x30, 0x00,
    0x31, 0x2E, 0x32, 0x2E, 0x38, 0x32, 0x36, 0x2E, 0x30, 0x2E, 0x31, 0x2E, 0x33, 0x36, 0x38, 0x30,
    0x30, 0x34, 0x33, 0x2E, 0x32, 0x2E, 0x31, 0x31, 0x34, 0x33, 0x2E, 0x31, 0x30, 0x37, 0x2E, 0x31,
    0x30, 0x34, 0x2E, 0x31, 0x30, 0x33, 0x2E, 0x31, 0x31, 0x35, 0x2E, 0x32, 0x2E, 0x31, 0x2E, 0x30,

    // ImplementationVersionName  VR: SH      VL: 10
    0x02, 0x00, 0x13, 0x00,       0x53, 0x48, 0x0A, 0x00,
    0x47, 0x44, 0x43, 0x4D, 0x20, 0x32, 0x2E, 0x31, 0x2E, 0x30,

    // SourceApplicationEntityTitle     VR: AE      VL: 8
    0x02, 0x00, 0x16, 0x00,             0x41, 0x45, 0x08, 0x00,
    0x67, 0x64, 0x63, 0x6D, 0x63, 0x6F, 0x6E, 0x76,

    // SpecificCharacterSet VR: CS      VL: 10
    0x08, 0x00, 0x05, 0x00, 0x43, 0x53, 0x0A, 0x00,
    0x49, 0x53, 0x4F, 0x5F, 0x49, 0x52, 0x20, 0x31, 0x30, 0x30,
 ];

pub const INVALID_VR_ELEMENT: &'static [u8] = &[
    // SOPClasUID           VR: XX      VL: 2
    0x08, 0x00, 0x16, 0x00, 0x57, 0x57, 0x02, 0x00, 0x00, 0x00,
];

pub const NULL_ELEMENT: &'static [u8] = &[
    // INVALID              VR: INVALID VL: 0
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
