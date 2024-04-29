use crate::app::{render_element_tag, render_value};
use cursive::traits::{Boxable, Identifiable};
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use cursive_table_view::{TableView, TableViewItem};
use dcmpipe_dict::dict::lookup::TAG_BY_VALUE;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmparser::{Parser, ParserBuilder};
use dcmpipe_lib::defn::tag::Tag;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct CursiveApp {
    openpath: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DicomElementColumn {
    Expand,
    Tag,
    Name,
    VR,
    Value,
}

impl DicomElementColumn {
    fn as_str(&self) -> &str {
        match *self {
            DicomElementColumn::Expand => "+",
            DicomElementColumn::Tag => "Tag",
            DicomElementColumn::Name => "Name",
            DicomElementColumn::VR => "VR",
            DicomElementColumn::Value => "Value",
        }
    }
}

impl TableViewItem<DicomElementColumn> for DicomElement {
    fn to_column(&self, column: DicomElementColumn) -> String {
        match column {
            DicomElementColumn::Expand => if self.is_seq() { "+" } else { "" }.to_owned(),
            DicomElementColumn::Tag => Tag::format_tag_to_display(self.tag),
            DicomElementColumn::Name => if let Some(tag) = TAG_BY_VALUE.get(&self.tag) {
                tag.ident
            } else {
                "<Private Tag>"
            }
            .to_owned(),
            DicomElementColumn::VR => self.vr.ident.to_owned(),
            DicomElementColumn::Value => {
                if let Ok(value) = render_value(&self) {
                    value
                } else {
                    "<Error Parsing Value>".to_owned()
                }
            }
        }
    }

    fn cmp(&self, other: &Self, column: DicomElementColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            DicomElementColumn::Expand => Ordering::Equal,
            DicomElementColumn::Tag => self.tag.cmp(&other.tag),
            DicomElementColumn::Name => Ordering::Equal,
            DicomElementColumn::VR => Ordering::Equal,
            DicomElementColumn::Value => Ordering::Equal,
        }
    }
}

impl CursiveApp {
    pub fn new(openpath: String) -> CursiveApp {
        CursiveApp { openpath }
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
        let dicom_iter: Parser<File> = ParserBuilder::new(file).build();

        let mut total_name_size: usize = 0;
        let mut items: Vec<DicomElement> = Vec::new();
        for elem in dicom_iter {
            let elem: DicomElement = elem?;
            if elem.get_sequence_path().is_empty() {
                let name: String = if let Some(tag) = TAG_BY_VALUE.get(&elem.tag) {
                    tag.ident
                } else {
                    "<Private Tag>"
                }
                .to_owned();

                total_name_size = name.len().max(total_name_size);

                items.push(elem);
            }
        }

        let mut cursive: Cursive = Cursive::default();
        cursive.add_global_callback('q', Cursive::quit);

        let mut table = TableView::<DicomElement, DicomElementColumn>::new()
            .column(
                DicomElementColumn::Expand,
                DicomElementColumn::Expand.as_str(),
                |c| c.width(5),
            )
            .column(
                DicomElementColumn::Tag,
                DicomElementColumn::Tag.as_str(),
                |c| c.width(12).ordering(Ordering::Greater),
            )
            .column(
                DicomElementColumn::Name,
                DicomElementColumn::Name.as_str(),
                |c| c.width(total_name_size),
            )
            .column(
                DicomElementColumn::VR,
                DicomElementColumn::VR.as_str(),
                |c| c.width(5),
            )
            .column(
                DicomElementColumn::Value,
                DicomElementColumn::Value.as_str(),
                |c| c,
            );

        table.set_items(items);

        table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
            let value = siv
                .call_on_id(
                    "table",
                    move |table: &mut TableView<DicomElement, DicomElementColumn>| {
                        render_element_tag(table.borrow_item(index).unwrap())
                    },
                )
                .unwrap();

            siv.add_layer(
                Dialog::around(TextView::new(value))
                    .title(format!("Removing row # {}", row))
                    .button("Close", move |s| {
                        s.call_on_id(
                            "table",
                            |table: &mut TableView<DicomElement, DicomElementColumn>| {
                                table.remove_item(index);
                            },
                        );
                        s.pop_layer();
                    }),
            );
        });

        cursive
            .add_layer(Dialog::around(table.with_id("table").full_screen()).title("DICOM Viewer"));

        cursive.run();

        Ok(())
    }
}
