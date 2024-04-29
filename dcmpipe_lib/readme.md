# dcmpipe_lib #

This crate is the core DICOM reading/writing library. This crate is focused only on basic
reading/writing of DICOM datasets and should have few dependencies. It does not depend on the DICOM
standard dictionary, instead able to parse through most dicom based only on knowing a minimal set of
DICOM constants.

This crate should eventually compile for WASM targets.

### Example ###

Using the parser to loop over elements as they are read from the DICOM dataset stream
```rust
let mut parser: Parser<'_, File> = ParserBuilder::default()
    .dictionary(&STANDARD_DICOM_DICTIONARY)
    // PixelData is 0x7FE0_0010, could also use tags::PixelData.tag.into()
    .stop(TagStop::BeforeTagValue(tags::PixelData.tag.into()))
    .build(file);

for element_res in parser {
    let element: DicomElement = element_res?;
    let value: RawValue = element.parse_value()?;
}
```

Reading the DICOM dataset into `DicomObject` and pulling an element by `TagPath`
```rust
let mut parser: Parser<'_, File> = ParserBuilder::default()
    .dictionary(&STANDARD_DICOM_DICTIONARY)
    .stop(TagStop::BeforeTagValue(tags::PixelData.tag.into()))
    .build(file);

// Parsing into object returns Result<Option<DicomObject>>.
// If the dataset isn't actually dicom it returns Ok(None) rather than error.
let dcmroot: DicomRoot = DicomRoot::parse(parser)??;

// Builds a path for
//    ReferencedFrameOfReferenceSequence[1]
//      .RTReferencedStudySequence[1]
//        .RTReferencedSeriesSequence[1]
//          .ContourImageSequence[11]
//            .ReferencedSOPInstanceUID
//
// There are convenience implementations of `From` of common types into `TagNode` as well as into
// `TagPath`. The `From` implementations for `TagPath` require all elements of `Vec`/slice be the
// same type. Since not all item indexes are `1`, this uses `.into()` to create a `Vec<TagNode>`
// which is then converted to a `TagPath` with another `.into()`.
let tagpath: TagPath = vec![
        (&tags::ReferencedFrameofReferenceSequence).into(),
        (&tags::RTReferencedStudySequence).into(),
        (&tags::RTReferencedSeriesSequence).into(),
        TagNode::from((&tags::ContourImageSequence, Some(11))),
        (&tags::ReferencedSOPInstanceUID).into(),
    ]
    .into();

// If all desired indexes are `1` then this can be slightly simplified:
let tagpath: TagPath = vec![
        &tags::ReferencedFrameofReferenceSequence,
        &tags::RTReferencedStudySequence,
        &tags::RTReferencedSeriesSequence,
        &tags::ContourImageSequence,
        &tags::ReferencedSOPInstanceUID,
    ]
    .into();

// Get the value by tagpath, parse the element value from its raw bytes.
let ref_sopuid: RawValue = dcmroot
    .get_child_by_tagpath(tagpath)?
    .get_element()
    .parse_value()?;

if let RawValue::Uid(uid) = ref_sopuid {
    println!("Referenced SOP Instance UID: {}", uid);
}
```
