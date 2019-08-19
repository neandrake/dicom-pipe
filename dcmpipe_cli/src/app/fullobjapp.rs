use crate::app::render_element;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::DicomObject;
use dcmpipe_lib::core::dcmparser::{DicomParserBuilder, DicomStreamParser};
use dcmpipe_lib::core::dcmreader::parse_stream;
use dcmpipe_lib::defn::ts::TSRef;
use std::collections::btree_map::IterMut;
use std::fs::File;
use std::io::{self, Error, ErrorKind, StdoutLock, Write};
use std::path::Path;

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
        let mut dicom_iter: DicomStreamParser<File> = DicomParserBuilder::new(file).build();

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(format!(
            "\n# Dicom-File-Format File: {:#?}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
            path,
            dicom_iter.get_ts().uid.ident).as_ref()
        )?;

        let mut dcmobj: DicomObject = parse_stream(&mut dicom_iter)?;
        let obj_iter: IterMut<u32, DicomObject> = dcmobj.iter_mut();
        self.render_objects(obj_iter, true, dicom_iter.get_ts(), &mut stdout)?;
        Ok(())
    }

    fn render_objects(
        &self,
        obj_iter: IterMut<u32, DicomObject>,
        mut prev_was_file_meta: bool,
        ts: TSRef,
        stdout: &mut StdoutLock,
    ) -> Result<(), Error> {
        for (tag, obj) in obj_iter {
            let mut elem: &mut DicomElement = obj.as_element().ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "DicomObject is not also element")
            })?;

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

            let printed: Option<String> = render_element(&mut elem)?;
            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }

            if obj.get_object_count() > 0 {
                self.render_objects(obj.iter_mut(), prev_was_file_meta, ts, stdout)?;
            }
        }

        Ok(())
    }
}
