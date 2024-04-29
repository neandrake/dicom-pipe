extern crate dcmpipe_lib;

use dcmpipe_lib::core::dict::dicom_elements as tags;
use dcmpipe_lib::core::tag::TagRef;
use dcmpipe_lib::read::dcmstream::{DicomStream};
use dcmpipe_lib::read::tagstop::TagStop;

use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Must specify dicom file to open");
    }
    let path: &Path = Path::new(&args[1]);

    let mut dstream: DicomStream<File> = DicomStream::new_from_path(path)
        .expect(&format!("Invalid path: {:?}", path));

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
