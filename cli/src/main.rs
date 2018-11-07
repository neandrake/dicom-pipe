extern crate dcmpipe;
extern crate walkdir;

use dcmpipe::core::dict::dicom_elements as tags;
use dcmpipe::core::tag::TagRef;

use dcmpipe::read::dcmstream::{DicomStream};
use dcmpipe::read::tagstop::TagStop;

use std::fs::File;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

static FIXTURE_DATASET1_FOLDER: &'static str = "fixtures/dataset1";

fn main() {
    let mut dstream: DicomStream<File> = get_first_file_stream();

    println!("\n# Dicom-File-Format File\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}", dstream.get_ts().uid.ident);
    dstream.read_file_meta_on_each(|ds: &mut DicomStream<File>, element_tag: u32| {
            if let Ok(strval) = ds.print_element(element_tag) {
                println!("{}", strval);
            } else {
                panic!("Unable to print element");
            }
        })
        .expect("Unable to read FileMetaInformation");

    // Ability to read dicom elements after FileMetaInformation
    // means that we interpret the transfer syntax properly, as
    // the fixtures are implicit VR (FMI is encoded as explicit)
    println!("\n\n# Dicom-Data-Set\n# Used TransferSyntax: {}", dstream.get_ts().uid.ident);

    let read_until_before_tag: TagRef = &tags::PixelData;
    dstream.read_until_on_each(
        TagStop::BeforeTag(read_until_before_tag.tag),
        |ds: &mut DicomStream<File>, element_tag: u32| {
            if let Ok(strval) = ds.print_element(element_tag) {
                println!("{}", strval);
            } else {
                panic!("Unable to print element");
            }
        })
        .expect("Error reading elements");
}


fn get_first_file_stream() -> DicomStream<File> {
    let dirwalker: WalkDir = WalkDir::new(FIXTURE_DATASET1_FOLDER)
        .min_depth(1)
        .max_depth(1);

    for entry_res in dirwalker.into_iter() {
        let entry: DirEntry = entry_res.unwrap();
        let path: &Path = entry.path();

        let dstream: DicomStream<File> = DicomStream::new_from_path(path)
            .expect(&format!("Unable to read file: {:?}", path));
        
        return dstream;
    }

    panic!("No DICOM files found");
}
