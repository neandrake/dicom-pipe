# dcmpipe #

## About ##
The `dcmpipe` library provides baseline functionality for managing DICOM,
including reading and writing DICOM files, decoding the PixelData element, and
the DIMSE network protocol (C-ECHO, C-FIND, C-STORE, C-MOVE, C-GET).

See the `dcmpipe_cli` sub-crate for example command-line utilities built using
the library:

![Screenshot](mdassets/screenshot.png "Screenshot")

## Quick Examples ##

```rust
// Set up a parser for a DICOM file.
let parser: Parser<'_, File> = ParserBuilder::default()
    // Stops parsing once the PixelData element is seen.
    .stop(TagStop::BeforeTagValue(&PixelData))
    // The dictionary is used during parsing for Implicit VR transfer syntaxes,
    // and associates the resolved VR to the resulting elements for parsing the
    // element values.
    .build(file, &STANDARD_DICOM_DICTIONARY);

// The parser is an iterator over elements.
for element_res in parser {
    let element = element_res?;
    let value = element.parse_value()
        // Parsing returns a Result<RawValue>
        .ok()
        // RawValue::string() returns Option<String>
        .and_then(|v| v.string())
        .unwrap();
}
```

```rust
let parser: Parser<'_, File> = ParserBuidler::default()
    .build(file, &STANDARD_DICOM_DICTIONARY);
let pixeldata_buffer = PixelDataInfo::process_dcm_parser(parser)?
    .load_pixel_data()?;

// There are U8, U16, U32, I8, I16, and I32 variants, depending on the DICOM's
// encoded values. See the comment on `PixelDataBuffer::load_pixel_data()` for
// details on when to expect which variants.
if let PixelDataBuffer::U16(pdbuf) = pixeldata_buffer {
    let (width, height) = (pdbuf.info().cols(), pdbuf.info().rows());

    // pixel_iter() provides an iterator over pixels converted for display.
    for PixelU16 { x, y, r, g, b } in pdbuf.pixel_iter() {
        // The x, y are usize, within the width and height of the image.
        // RGB: r, g, b are the respective components of the pixel color.
        // MONOCHROME1/2: r, g, b are the same value.
    }
}
```

Refer to the `readme.md` in `dcmpipe_lib/` folder for further examples, and also
the `dcmpipe_cli` sample applications.

## Design Goals ##
The APIs are not designed to encode the DICOM Information Object Definitions
within the type system. It will allow both reading and writing structures which
are valid DICOM binary protocol but it is up to the user of the API to ensure
that IODs are structured appropriately, i.e. that all necessary DICOM elements
are present and valid for a CT, MR, etc.

While this design puts the burden on the API user to interpret and create well-
formed DICOM structures, it grants greater flexibility, especially for working
with existing malformed DICOM datasets.

The DICOM standard dictionary of tags, UIDs, transfer syntaxes, etc. are
available as an optional feature of the crate. Reading and writing DICOM does
not require the DICOM standard dictionary and can be excluded to minimize the
resulting binary size if desired.

The core crate has minimal dependencies, two required and two optional.

- `encoding_rs` (required) for properly handling text encoding supported by
  DICOM.
- `thiserror` (required) for deriving errors.
- `phf` (optional) the DICOM standard dictionary components are encoded in a
  lookup map using perfect hash maps.
- `libflate` (optional) for reading and writing deflated datasets.

The API is also focused on enabling efficient operations:

- DICOM datasets are parsed in a stream-like manner allowing the API user to
  decide what is necessary to retain in-memory.
- DICOM element values themselves are not parsed during parsing of the dataset.
- Flexible options for limiting how much of a DICOM dataset to parse, e.g.
  stopping before the PixelData element if only metadata about the DICOM SOP is
  needed.
- DIMSE handling of DICOM dataset communication does not require the entire
  dataset to be loaded into memory at once.
- At the moment, decoding PixelData does require the entire element value to be
  loaded into meomry.

## Crates ##

- `dcmpipe_cli`: Command-line tools utilizing the `dcmpipe` library to show the
  usage of the `dcmpipe_lib` create. See the `readme.md` within that sub-folder
  for more information.
- `dcmpipe_dict_builder`: Parses the DICOM Standard XML files for producing the
  standard DICOM dictionary. This is intended to be used by `build.rs` scripts.
- `dcmpipe_lib`: The core API for reading and writing DICOM, decoding images,and
  optional support for the DICOM Message Exchange network protocol.

