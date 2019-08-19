use crate::app::render_element;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmparser::{DicomParserBuilder, DicomStreamParser};
use std::fs::File;
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;

pub struct LowMemApp {
    openpath: String,
}

impl LowMemApp {
    pub fn new(openpath: String) -> LowMemApp {
        LowMemApp { openpath }
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

        let mut prev_was_file_meta: bool = true;

        while let Some(elem) = dicom_iter.next() {
            let mut elem: DicomElement = elem?;
            if prev_was_file_meta && elem.tag > 0x0002_FFFF {
                stdout.write_all(
                    format!(
                        "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                        dicom_iter.get_ts().uid.ident
                    )
                    .as_ref(),
                )?;
                prev_was_file_meta = false;
            }

            let printed: Option<String> = render_element(&mut elem)?;

            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }
        }

        Ok(())
    }
}
