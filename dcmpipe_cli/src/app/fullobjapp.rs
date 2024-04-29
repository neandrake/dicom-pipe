use crate::app::{CommandApplication, render_element, parse_file};
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomObject, DicomRoot};
use dcmpipe_lib::core::dcmparser::Parser;
use dcmpipe_lib::core::dcmparser_util::parse_into_object;
use dcmpipe_lib::defn::ts::TSRef;
use std::fs::File;
use std::io::{self, Error, StdoutLock, Write};
use std::path::{Path, PathBuf};

pub struct FullObjApp {
    openpath: PathBuf,
}

impl FullObjApp {
    pub fn new(openpath: PathBuf) -> FullObjApp {
        FullObjApp { openpath }
    }

    fn render_objects(
        &mut self,
        dcmnode: &impl DicomNode,
        mut prev_was_file_meta: bool,
        ts: TSRef,
        stdout: &mut StdoutLock<'_>,
    ) -> Result<(), Error> {
        for (tag, obj) in dcmnode.iter_child_nodes() {
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
                self.render_objects(obj, prev_was_file_meta, ts, stdout)?;
            }
            for index in 0..obj.get_item_count() {
                let child_obj: &DicomObject = obj.get_item(index).unwrap();
                let child_elem: &DicomElement = child_obj.as_element();
                if let Some(printed) = render_element(child_elem)? {
                    stdout.write_all(format!("{}\n", printed).as_ref())?;
                }
                self.render_objects(child_obj, prev_was_file_meta, ts, stdout)?;
            }
        }

        Ok(())
    }
}

impl CommandApplication for FullObjApp {
    fn run(&mut self) -> Result<(), Error> {
        let path_buf: PathBuf = self.openpath.clone();
        let path: &Path = path_buf.as_path();
        let mut parser: Parser<'_, File> = parse_file(path)?;

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(format!(
            "\n# Dicom-File-Format File: {:#?}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
            path,
            parser.get_ts().uid.ident).as_ref()
        )?;

        let dcmroot: DicomRoot<'_> = parse_into_object(&mut parser)?;
        self.render_objects(&dcmroot, true, parser.get_ts(), &mut stdout)?;
        Ok(())
    }
}
