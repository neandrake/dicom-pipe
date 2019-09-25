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
use std::ops::AddAssign;
use std::path::Path;

pub struct CursiveApp {
    openpath: String,
    elements: Vec<DicomElement>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DicomElementColumn {
    Expand,
    Tag,
    Name,
    VR,
    Value,
}

#[derive(Clone)]
pub struct DicomElementValue<'me> {
    element: &'me DicomElement,
}
impl <'me> DicomElementValue<'me> {
    fn new(element: &'me DicomElement) -> DicomElementValue {
        DicomElementValue {
            element,
        }
    }
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

impl <'me> TableViewItem<DicomElementColumn> for DicomElementValue<'me> {
    fn to_column(&self, column: DicomElementColumn) -> String {
        match column {
            DicomElementColumn::Expand => if self.element.is_seq_like() { "+" } else { "" }.to_owned(),
            DicomElementColumn::Tag => Tag::format_tag_to_display(self.element.tag),
            DicomElementColumn::Name => if let Some(tag) = TAG_BY_VALUE.get(&self.element.tag) {
                tag.ident
            } else {
                "<Private Tag>"
            }
            .to_owned(),
            DicomElementColumn::VR => self.element.vr.ident.to_owned(),
            DicomElementColumn::Value => {
                if let Ok(value) = render_value(&self.element) {
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
            DicomElementColumn::Tag => self.element.tag.cmp(&other.element.tag),
            DicomElementColumn::Name => Ordering::Equal,
            DicomElementColumn::VR => Ordering::Equal,
            DicomElementColumn::Value => Ordering::Equal,
        }
    }
}

impl <'me> CursiveApp {
    pub fn new(openpath: String) -> CursiveApp {
        CursiveApp { openpath, elements: Vec::new() }
    }

    pub fn run(&'me mut self) -> Result<(), Error> {
        let path: &Path = Path::new(&self.openpath);

        if !path.is_file() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("invalid file: {}", path.display()),
            ));
        }

        let file: File = File::open(path)?;
        let parser: Parser<File> = ParserBuilder::new(file).build();

        let mut total_name_size: usize = 0;
        for elem in parser {
            let elem: DicomElement = elem?;
            if elem.get_sequence_path().is_empty() {
                let name: String = if let Some(tag) = TAG_BY_VALUE.get(&elem.tag) {
                    tag.ident
                } else {
                    "<Private Tag>"
                }
                .to_owned();

                total_name_size = name.len().max(total_name_size);

                self.elements.push(elem);
            }
        }

        let _items: Vec<DicomElementValue> = self.elements
            .iter()
            .map(DicomElementValue::new)
            .collect::<Vec<DicomElementValue>>();

        let mut cursive: Cursive = Cursive::default();
        cursive.add_global_callback('q', Cursive::quit);

        let mut table = TableView::<DicomElementValue, DicomElementColumn>::new()
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

//        table.set_items(items);

        table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
            let value = siv
                .call_on_id(
                    "table",
                    move |table: &mut TableView<DicomElementValue, DicomElementColumn>| {
                        let tag_info: String =
                            render_element_tag(table.borrow_item(index).unwrap().element);
                        let val_info: String =
                            render_value(table.borrow_item(index).unwrap().element).unwrap();
                        let mut display: String = String::new();
                        display.add_assign(tag_info.as_str());
                        display.add_assign("\n");
                        display.add_assign(val_info.as_str());
                        display
                    },
                )
                .unwrap();

            siv.add_layer(
                Dialog::around(TextView::new(value))
                    .title(format!("Viewing row # {}", row))
                    .button("Close", move |s| {
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
