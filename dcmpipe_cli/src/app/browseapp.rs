use std::collections::HashMap;
use std::fs::File;
use std::io::{stdout, Stdout};
use std::ops::Sub;
use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Result};

use crossterm::event::{self, Event::Key, Event::Mouse, KeyCode::Char};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind, MouseButton,
    MouseEvent, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomRoot};
use dcmpipe_lib::core::read::Parser;
use dcmpipe_lib::defn::tag::{Tag, TagNode, TagPath};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use ratatui::{Frame, Terminal};

use crate::app::CommandApplication;
use crate::args::BrowseArgs;

use super::{ElementWithLineFmt, TagName, TagValue};

pub struct BrowseApp {
    args: BrowseArgs,
}

/// The result of parsing all elements in a DICOM data set.
struct DicomDocumentModel<'app> {
    /// The file path the DICOM dataset was loaded from.
    path: &'app Path,
    /// The mapping of some `TagPath` to that path's parsed child nodes. The empty path represents
    /// the root of the DICOM data set, whose model contains all the top-level DICOM elements. If
    /// the data set includes sequences then additional entries for each sequence element will be
    /// present to include its parsed sub-elements.
    map: HashMap<TagPath, DicomElementModel<'app>>,
}

/// The data model for an element. This represents one "level" within a DICOM document model, where
/// the rows for this model are the first-level child elements of some other `DicomNode`.
/// This model only contains the data necessary for rendering, so all DICOM element values are
/// parsed in order to build this struct.
#[derive(Clone)]
struct DicomElementModel<'model> {
    /// The ordered values parsed from the DICOM elements at this level.
    rows: Vec<Row<'model>>,
    /// For each row, the maximum length of DICOM tag name, which aside from DICOM value will be
    /// the only other column of variable width.
    max_name_width: u16,
}

/// The ViewState of what's displayed on screen. This should remain minimal (i.e. not include the
/// data model), as it will be cloned every frame render. This contains both view-level information
/// about the current model being displayed as well as view state from user input.
#[derive(Clone)]
struct ViewState {
    /// The number of rows of the current model.
    num_rows: usize,
    /// The maximum width of all DICOM tag names of the current model.
    max_name_width: u16,
    /// The Ratatui table state which contains offset and selection.
    table_state: TableState,
    /// Whether the user has requested to quit/close.
    user_quit: bool,
    /// The user selected a row to dive deeper into.
    user_selected: UserNav,
    /// The current path of elements to display.
    current_root_element: TagPath,
}

/// Actions the user can take to navigate the DICOM document.
#[derive(Clone)]
enum UserNav {
    None,
    IntoLevel(usize),
    UpLevel,
}

impl<'app> CommandApplication for BrowseApp {
    fn run(&mut self) -> Result<()> {
        let path: &Path = self.args.file.as_path();
        let mut parser: Parser<'_, File> = super::parse_file(path, true)?;
        let parse_result = DicomRoot::parse(&mut parser);

        let dcmroot = match parse_result {
            Ok(Some(dcmroot)) => dcmroot,
            Ok(None) => return Err(anyhow!("Not valid DICOM.")),
            Err(err) => return Err(anyhow!(err)),
        };

        let mut doc_model = DicomDocumentModel::parse(path, &dcmroot);

        let mut terminal = self.init()?;

        let app_result = self.run_loop(&mut terminal, &dcmroot, &mut doc_model);

        self.close(terminal)?;

        app_result?;

        Ok(())
    }
}

impl<'app> DicomDocumentModel<'app> {
    fn parse<'dict>(path: &'app Path, dcmroot: &DicomRoot<'dict>) -> DicomDocumentModel<'app> {
        let map = DicomElementModel::parse(dcmroot);
        DicomDocumentModel { path, map }
    }
}

impl<'model> DicomElementModel<'model> {
    fn parse<'dict>(dcmnode: &'dict dyn DicomNode) -> HashMap<TagPath, DicomElementModel<'model>> {
        let mut map: HashMap<TagPath, DicomElementModel<'model>> = HashMap::new();

        let mut rows: Vec<Row<'model>> = Vec::with_capacity(dcmnode.get_child_count());
        let mut max_name_width: u16 = 0;
        for (child_tag, child) in dcmnode.iter_child_nodes() {
            if child.get_item_count() > 0 || child.get_child_count() > 0 {
                let child_map = DicomElementModel::parse(child);
                map.extend(child_map.into_iter());
            }

            let tag_render: TagName = child.as_element().into();
            let elem_name = tag_render.to_string();
            max_name_width = max_name_width.max(elem_name.len() as u16);
            let elem_value: TagValue = ElementWithLineFmt(child.as_element(), false).into();

            let mut cells: Vec<Cell> = Vec::with_capacity(5);
            cells.push(
                Cell::from(if child.get_child_count() > 0 { "+" } else { "" })
                    .style(Style::default().fg(Color::DarkGray)),
            );

            cells.push(
                Cell::from(Tag::format_tag_to_display(*child_tag))
                    .style(Style::default().fg(Color::DarkGray)),
            );

            match tag_render {
                TagName::Known(_, _) => {
                    cells.push(Cell::from(elem_name));
                }
                _ => {
                    cells.push(
                        Cell::from(elem_name).style(
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::ITALIC),
                        ),
                    );
                }
            }

            cells.push(
                Cell::from(child.as_element().get_vr().ident)
                    .style(Style::default().fg(Color::DarkGray)),
            );

            let cell = match elem_value {
                TagValue::Sequence => Cell::from(""),
                TagValue::Error(err_str) => {
                    Cell::from(err_str).style(Style::default().bg(Color::Red))
                }
                TagValue::Uid(uid, name) => Cell::from(Line::from(vec![
                    Span::styled(uid, Style::default()),
                    Span::styled(
                        format!(" {}", name),
                        Style::default().fg(Color::LightYellow),
                    ),
                ])),
                TagValue::Stringified(str_val) => Cell::from(str_val),
            };
            cells.push(cell);

            rows.push(Row::new(cells));
        }

        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let elem_tbl = DicomElementModel {
            rows,
            max_name_width,
        };

        let tagpath = dcmnode.get_element().map_or_else(
            || TagPath {
                nodes: Vec::with_capacity(0),
            },
            |e| e.get_tagpath(),
        );
        map.insert(tagpath, elem_tbl);

        map
    }
}

impl<'app> BrowseApp {
    pub fn new(args: BrowseArgs) -> BrowseApp {
        BrowseApp { args }
    }

    fn init(&self) -> Result<Terminal<CrosstermBackend<Stdout>>> {
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;
        Ok(terminal)
    }

    fn close(&self, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        terminal.clear()?;
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        disable_raw_mode()?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn run_loop<'dict>(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        dcmroot: &'dict DicomRoot,
        doc_model: &'app mut DicomDocumentModel<'app>,
    ) -> Result<()> {
        let mut view_state = ViewState {
            num_rows: 0,
            max_name_width: 0,
            table_state: TableState::new(),
            user_quit: false,
            user_selected: UserNav::None,
            current_root_element: TagPath {
                nodes: Vec::with_capacity(0),
            },
        };

        let dataset_title = doc_model.path.to_str().unwrap_or_default();

        loop {
            let Some(table_model) = doc_model.map.get(&view_state.current_root_element) else {
                return Ok(());
            };

            // Apply state from current model.
            view_state.num_rows = table_model.rows.len();
            view_state.max_name_width = table_model.max_name_width;
            // Reset user-input state.
            view_state.user_quit = false;
            view_state.user_selected = UserNav::None;

            // Ratatui's Table requires an iterator over owned Rows, so the model must be cloned
            // every render, apparently. The render_stateful_widget() function requires moving a
            // Table into it, so even if the Table was lifted up into view_state or similar, some
            // sort of clone would have to be passed into rendering.
            let render_model = table_model.clone();
            // The view_state is small and intended to be cloned every iteration.
            let render_view_state = view_state.clone();

            terminal.draw(move |frame| {
                self.render(dataset_title, render_model, render_view_state, frame)
            })?;

            view_state = self.update_state_from_user_input(view_state)?;

            // Enact user-input state.
            if view_state.user_quit {
                break;
            }
            match view_state.user_selected {
                UserNav::None => {}
                UserNav::IntoLevel(selected) => {
                    let next_path = if view_state.current_root_element.nodes.is_empty() {
                        dcmroot
                            .get_child_by_index(selected)
                            .map(|o| o.as_element().get_tagpath())
                            .unwrap_or_else(|| view_state.current_root_element.clone())
                    } else {
                        dcmroot
                            .get_child_by_tagpath(&view_state.current_root_element)
                            .and_then(|c| {
                                if c.get_child_count() > 0 {
                                    c.get_child_by_index(selected)
                                } else if c.get_item_count() > 0 {
                                    c.get_item_by_index(selected + 1)
                                } else {
                                    None
                                }
                            })
                            .map(|o| o.as_element().get_tagpath())
                            .unwrap_or_else(|| view_state.current_root_element.clone())
                    };

                    if doc_model.map.contains_key(&next_path) {
                        view_state.current_root_element = next_path;
                    }
                }
                UserNav::UpLevel => {
                    if !view_state.current_root_element.nodes.is_empty() {
                        view_state.current_root_element = TagPath::from(view_state
                            .current_root_element
                            .nodes
                            .drain(..view_state.current_root_element.nodes.len() - 1)
                            .collect::<Vec<TagNode>>());
                    }
                }
            }
        }
        Ok(())
    }

    fn update_state_from_user_input(&self, mut view_state: ViewState) -> Result<ViewState> {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Key(key) => match key.kind {
                    KeyEventKind::Press => self.event_keypress(&mut view_state, key),
                    KeyEventKind::Release => self.event_keyrelease(&mut view_state, key),
                    _ => {}
                },
                Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(button) | MouseEventKind::Drag(button) => {
                        self.event_mouse_down(&mut view_state, mouse, button)
                    }
                    MouseEventKind::ScrollDown => {
                        self.event_mouse_scroll_down(&mut view_state, mouse)
                    }
                    MouseEventKind::ScrollUp => self.event_mouse_scroll_up(&mut view_state, mouse),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(view_state)
    }

    fn event_keyrelease(&self, _view_state: &mut ViewState, _event: KeyEvent) {}

    fn event_keypress(&self, view_state: &'app mut ViewState, event: KeyEvent) {
        match event.code {
            Char('q') => view_state.user_quit = true,
            KeyCode::Esc => view_state.user_quit = true,
            KeyCode::Enter => {
                if let Some(selected) = view_state.table_state.selected() {
                    view_state.user_selected = UserNav::IntoLevel(selected);
                }
            }
            Char('h') | KeyCode::Left | KeyCode::Backspace => {
                view_state.user_selected = UserNav::UpLevel
            }
            Char('j') | KeyCode::Down => self.table_select_next(view_state, 1),
            Char('k') | KeyCode::Up => self.table_select_next(view_state, -1),
            _ => {}
        }
    }

    fn event_mouse_down(
        &self,
        view_state: &'app mut ViewState,
        event: MouseEvent,
        button: MouseButton,
    ) {
        if button != MouseButton::Left {
            return;
        }

        // Convert the event row (all widgets on screen) into the table row.
        // Subtract 2, 1 for the table border, 1 for the table header row.
        let row_index = event.row.saturating_sub(2) as usize;

        let index = Some(view_state.table_state.offset().saturating_add(row_index));
        // Only toggle the selection on click, not drag.
        if view_state.table_state.selected() == index
            && event.kind == MouseEventKind::Down(MouseButton::Left)
        {
            view_state.table_state.select(None)
        } else {
            view_state.table_state.select(index);
        }
    }

    fn event_mouse_scroll_up(&self, view_state: &'app mut ViewState, _event: MouseEvent) {
        self.table_scroll_next(view_state, -1);
    }

    fn event_mouse_scroll_down(&self, view_state: &'app mut ViewState, _event: MouseEvent) {
        self.table_scroll_next(view_state, 1);
    }

    fn table_scroll_next(&self, view_state: &'app mut ViewState, modifier: isize) {
        let i = view_state
            .table_state
            .offset()
            .saturating_add_signed(modifier)
            .min(view_state.num_rows)
            .max(0);
        *view_state.table_state.offset_mut() = i;
    }

    fn table_select_next(&self, view_state: &'app mut ViewState, modifier: isize) {
        let i = match view_state.table_state.selected() {
            None => 0,
            Some(i) => view_state
                .num_rows
                .sub(1)
                .min(i.saturating_add_signed(modifier))
                .max(0),
        };
        view_state.table_state.select(Some(i));
    }

    fn render(
        &self,
        title: &str,
        model: DicomElementModel,
        view_state: ViewState,
        frame: &mut Frame,
    ) {
        let column_widths = [
            Constraint::Length(1),
            Constraint::Length(11),
            Constraint::Length(view_state.max_name_width),
            Constraint::Length(2),
            Constraint::Max(1024),
        ];

        let table = Table::new(model.rows, column_widths)
            .header(
                Row::new(vec!["+", "Tag", "Name", "VR", "Value"])
                    .style(Style::default().fg(Color::LightYellow)),
            )
            .block(
                Block::default()
                    .title(
                        Title::from(Line::from(Span::styled(
                            "[DICOM Browser]".to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        )))
                        .alignment(Alignment::Left),
                    )
                    .title(
                        Title::from(Line::from(Span::styled(
                            format!("[{}]", title),
                            Style::default().fg(Color::LightBlue),
                        )))
                        .alignment(Alignment::Right),
                    )
                    .borders(Borders::all()),
            )
            .highlight_style(Style::default().bg(Color::LightBlue));

        let sections = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(frame.size());

        frame.render_stateful_widget(table, sections[0], &mut view_state.table_state.clone());
    }
}
