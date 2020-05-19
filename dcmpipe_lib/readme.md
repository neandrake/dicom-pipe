# dcmpipe_lib #

This crate is the core DICOM reading/writing library. This crate is focused only on basic reading/writing of DICOM
datasets and should have few dependencies. It does not depend on the DICOM standard dictionary, instead able to parse
through most dicom based only on knowing a minimal set of DICOM constants.

This crate should eventually compile with `#[no_std]` and be compatible for WASM targets.

### Example ###

Using the parser to loop over elements as they are read from the DICOM dataset stream
```rust
let mut parser: Parser<'_, File> = ParserBuilder::default()
    .dictionary(&STANDARD_DICOM_DICTIONARY)
    // PixelData is 0x7FE0_0010, could also use tags::PixelData.tag.into()
    .tagstop(TagStop::BeforeTag(tags::PixelData.tag.into()))
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
    .tagstop(TagStop::BeforeTag(tags::PixelData.tag.into()))
    .build(file);

// parsing into object returns Result<Option<DicomObject>>
// if the dataset isn't actually dicom it returns Ok(None) rather than error
let dcmroot: DicomRoot = dcmparser_util::parse_into_object(parser)??;

// builds a path for ReferencedFrameOfReferenceSequence[1].RTReferencedStudySequence[1].RTReferencedSeriesSequence[1].ContourImageSequence[11].ReferencedSOPInstanceUID
let tagpath: TagPath = vec![
        TagNode::new(tags::ReferencedFrameofReferenceSequence.tag, Some(1)),
        TagNode::new(tags::RTReferencedStudySequence.tag, Some(1)),
        TagNode::new(tags::RTReferencedSeriesSequence.tag, Some(1)),
        TagNode::new(tags::ContourImageSequence.tag, Some(11)),
        tags::ReferencedSOPInstanceUID.tag.into(),
    ]
    .into();

// get the value by tagpath, parse the element value from its raw bytes
let ref_sopuid: RawValue = dcmroot
    .get_child_by_tagpath(tagpath)?
    .get_element()
    .parse_value()?;

if let RawValue::Uid(uid) = ref_sopuid {
    println!("Referenced SOP Instance UID: {}", uid);
}
```