use crate::app::{parse_file, render_element, CommandApplication};
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmparser::Parser;
use std::fs::File;
use std::io::{self, Error, Write};
use std::path::{Path, PathBuf};

pub struct LowMemApp {
    openpath: PathBuf,
}

impl LowMemApp {
    pub fn new(openpath: PathBuf) -> LowMemApp {
        LowMemApp { openpath }
    }
}

impl CommandApplication for LowMemApp {
    fn run(&mut self) -> Result<(), Error> {
        let path: &Path = self.openpath.as_path();
        let mut parser: Parser<'_, File> = parse_file(path)?;

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(format!(
            "\n# Dicom-File-Format File: {:#?}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
            path,
            parser.get_ts().uid.ident).as_ref()
        )?;

        let mut prev_was_file_meta: bool = true;

        while let Some(elem) = parser.next() {
            let elem: DicomElement = elem?;
            if prev_was_file_meta && elem.tag > 0x0002_FFFF {
                stdout.write_all(
                    format!(
                        "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                        parser.get_ts().uid.ident
                    )
                    .as_ref(),
                )?;
                prev_was_file_meta = false;
            }

            let printed: Option<String> = render_element(&elem)?;

            if let Some(printed) = printed {
                stdout.write_all(format!("{}\n", printed).as_ref())?;
            }
        }

        Ok(())
    }
}
