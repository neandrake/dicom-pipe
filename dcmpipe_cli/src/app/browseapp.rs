use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;

use anyhow::{anyhow, Result};
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};
use cursive_table_view::{TableColumn, TableView, TableViewItem};

use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomRoot};
use dcmpipe_lib::core::read::Parser;
use dcmpipe_lib::defn::tag::Tag;

use crate::app::CommandApplication;
use crate::args::BrowseArgs;

pub struct BrowseApp {
    args: BrowseArgs,
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
    fn iter() -> std::vec::IntoIter<DicomElementColumn> {
        vec![
            DicomElementColumn::Expand,
            DicomElementColumn::Tag,
            DicomElementColumn::Name,
            DicomElementColumn::VR,
            DicomElementColumn::Value,
        ]
        .into_iter()
    }

    fn title(&self) -> &str {
        match *self {
            DicomElementColumn::Expand => "+",
            DicomElementColumn::Tag => "Tag",
            DicomElementColumn::Name => "Name",
            DicomElementColumn::VR => "VR",
            DicomElementColumn::Value => "Value",
        }
    }

    fn configure(
        &self,
        max_tagname_size: usize,
        c: TableColumn<DicomElementColumn>,
    ) -> TableColumn<DicomElementColumn> {
        match self {
            DicomElementColumn::Expand => c.width(5),
            DicomElementColumn::Tag => c.width(12).ordering(Ordering::Greater),
            DicomElementColumn::Name => c.width(max_tagname_size),
            DicomElementColumn::VR => c.width(5),
            DicomElementColumn::Value => c,
        }
    }
}

#[derive(Clone)]
pub struct DicomElementRow {
    tag: u32,
    seq: String,
    tag_display: String,
    name: String,
    vr: String,
    row_value: String,
    full_value: String,
}
impl DicomElementRow {
    fn new(element: &DicomElement) -> DicomElementRow {
        let seq: String = if element.is_seq_like() { "+" } else { "" }.to_owned();
        let name: String = super::render_tag_name(element).to_owned();

        let row_value = if let Ok(value) = super::render_value(element, false) {
            value
        } else {
            "<Error Parsing Value>".to_owned()
        };

        let full_value = if let Ok(value) = super::render_value(element, true) {
            value
        } else {
            "<Error Parsing Value>".to_owned()
        };

        DicomElementRow {
            tag: element.get_tag(),
            seq,
            tag_display: Tag::format_tag_to_display(element.get_tag()),
            name,
            vr: element.get_vr().ident.to_owned(),
            row_value,
            full_value,
        }
    }

    fn element_dialog_title(&self) -> String {
        format!("{} {} {}", self.tag_display, self.name, self.vr)
    }
}

impl TableViewItem<DicomElementColumn> for DicomElementRow {
    fn to_column(&self, column: DicomElementColumn) -> String {
        match column {
            DicomElementColumn::Expand => self.seq.clone(),
            DicomElementColumn::Tag => self.tag_display.clone(),
            DicomElementColumn::Name => self.name.clone(),
            DicomElementColumn::VR => self.vr.clone(),
            DicomElementColumn::Value => self.row_value.clone(),
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

impl BrowseApp {
    pub fn new(args: BrowseArgs) -> BrowseApp {
        BrowseApp { args }
    }
}

impl CommandApplication for BrowseApp {
    fn run(&mut self) -> Result<()> {
        let path: &Path = self.args.file.as_path();
        let mut parser: Parser<'_, File> = super::parse_file(path, true)?;
        let parse_result = DicomRoot::parse_into_object(&mut parser);

        let dcmroot = match parse_result {
            Ok(Some(dcmroot)) => dcmroot,
            Ok(None) => return Err(anyhow!("Not valid DICOM.")),
            Err(err) => return Err(anyhow!(err)),
        };

        let mut items: Vec<DicomElementRow> = Vec::new();
        let mut total_name_size: usize = 0;
        for elem in dcmroot.flatten()? {
            if elem.get_sequence_path().is_empty() {
                let name: &str = super::render_tag_name(elem);
                total_name_size = name.len().max(total_name_size);
                items.push(DicomElementRow::new(elem));
            }
        }

        let mut cursive: Cursive = Cursive::default();
        cursive.add_global_callback('q', Cursive::quit);
        cursive.add_global_callback('~', Cursive::toggle_debug_console);

        let mut table = TableView::<DicomElementRow, DicomElementColumn>::new();

        for column in DicomElementColumn::iter() {
            table = table.column(column, column.title(), |c| {
                column.configure(total_name_size, c)
            });
        }

        table.set_items(items);

        table.set_on_submit(|siv: &mut Cursive, _view_index: usize, data_index: usize| {
            let title: String = siv
                .call_on_name(
                    "table",
                    |table: &mut TableView<DicomElementRow, DicomElementColumn>| {
                        table
                            .borrow_item(data_index)
                            .unwrap()
                            .element_dialog_title()
                    },
                )
                .unwrap();

            let value: String = siv
                .call_on_name(
                    "table",
                    |table: &mut TableView<DicomElementRow, DicomElementColumn>| {
                        table.borrow_item(data_index).unwrap().full_value.clone()
                    },
                )
                .unwrap();

            siv.add_layer(Dialog::around(TextView::new(value)).title(title).button(
                "Close",
                move |s| {
                    s.pop_layer();
                },
            ));
        });

        cursive.add_layer(
            Dialog::around(table.with_name("table").full_screen()).title("DICOM Browser"),
        );

        cursive.run();

        Ok(())
    }
}
