use crate::app::render_element;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::dcmparser::{Parser, ParserBuilder};
use dcmpipe_lib::core::dcmparser_util::parse_into_object;
use dcmpipe_lib::defn::ts::TSRef;
use std::collections::btree_map::Iter;
use std::fs::File;
use std::io::{self, Error, ErrorKind, StdoutLock, Write};
use std::path::Path;
use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;

pub struct FullObjApp {
    openpath: String,
}

impl FullObjApp {
    pub fn new(openpath: String) -> FullObjApp {
        FullObjApp { openpath }
    }

    pub fn run(&self) -> Result<(), Error> {
        let path: &Path = Path::new(&self.openpath);

        if !path.is_file() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("invalid file: {}", path.display()),
            ));
        }

        let file: File = File::open(path)?;
        let mut parser: Parser<File> = ParserBuilder::new(file)
            .dictionary(&STANDARD_DICOM_DICTIONARY)
            .build();

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(format!(
            "\n# Dicom-File-Format File: {:#?}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
            path,
            parser.get_ts().uid.ident).as_ref()
        )?;

        let dcmroot: DicomRoot = parse_into_object(&mut parser)?;
        let obj_iter: Iter<u32, DicomObject> = dcmroot.iter();
        self.render_objects(obj_iter, true, parser.get_ts(), &mut stdout)?;
        Ok(())
    }

    fn render_objects(
        &self,
        obj_iter: Iter<u32, DicomObject>,
        mut prev_was_file_meta: bool,
        ts: TSRef,
        stdout: &mut StdoutLock,
    ) -> Result<(), Error> {
        for (tag, obj) in obj_iter {
            let elem: &DicomElement = obj.as_element();

            if prev_was_file_meta && *tag > 0x0002_FFFF {
                stdout.write_all(
                    format!(
                        "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                        ts.uid.ident
                    )
                    .as_ref(),
                )?;
                prev_was_file_meta = false;
            }

            let printed: Option<String> = render_element(&elem)?;
            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }

            if obj.get_child_count() > 0 {
                self.render_objects(obj.iter(), prev_was_file_meta, ts, stdout)?;
            }
        }

        Ok(())
    }
}
