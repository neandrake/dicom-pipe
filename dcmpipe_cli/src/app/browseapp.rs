//! The browse command opens a TUI for navigating through a DICOM data set.

use std::{
    collections::HashMap,
    fs::File,
    io::{stdout, Stdout},
    ops::Sub,
    path::Path,
    time::Duration,
};

use anyhow::{anyhow, Result};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture,
        Event::Key,
        Event::Mouse,
        KeyCode::{self, Char},
        KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dcmpipe_lib::{
    core::{
        dcmobject::{DicomObject, DicomRoot},
        defn::{
            constants,
            tag::{Tag, TagNode, TagPath},
        },
        read::Parser,
    },
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

use crate::{app::CommandApplication, args::BrowseArgs};

use super::{ElementWithLineFmt, TagCategory, TagValue};

pub struct BrowseApp {
    args: BrowseArgs,
}

#[derive(Debug)]
enum BrowseError {
    InvalidTagPath(TagPath),
}

impl std::fmt::Display for BrowseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrowseError::InvalidTagPath(tagpath) => {
                let rendered = if tagpath.is_empty() {
                    "<empty>".to_owned()
                } else {
                    TagPath::format_tagpath_to_display(tagpath, Some(&STANDARD_DICOM_DICTIONARY))
                };
                write!(f, "No model for path {rendered}")
            }
        }
    }
}

impl std::error::Error for BrowseError {}

/// The result of parsing all elements in a DICOM data set.
struct DicomDocumentModel<'app> {
    /// The file path the DICOM dataset was loaded from.
    file_path: &'app Path,
    /// The mapping of DICOM nodes to its respective data model. The empty path represents the root
    /// of the DICOM data set, whose model contains all the top-level DICOM elements.
    node_models: HashMap<TagPath, DicomNodeModel<'app>>,
    node_views: HashMap<TagPath, DicomNodeViewState>,
}

/// The data model for a node within a DICOM document. The model for a given node are the rows
/// representing the immediate child elements of that node.
/// This model only contains the data necessary for rendering, so all DICOM element values are
/// rendered in order to build this struct.
#[derive(Clone)]
struct DicomNodeModel<'model> {
    /// The ordered values parsed from the DICOM elements at this level.
    rows: Vec<Row<'model>>,
    /// For each row, the maximum length of DICOM tag name, which aside from DICOM value will be
    /// the only other column of variable width.
    max_name_width: u16,
}

/// The view state of what's displayed on screen for a `DicomNodeModel`. This should remain
/// separate from the data model as it will be cloned every frame render.
#[derive(Clone)]
struct DicomNodeViewState {
    /// Title to show in top-left of table
    dataset_title: String,
    /// The number of rows of the current model.
    num_rows: usize,
    /// The maximum width of all DICOM tag names of the current model.
    max_name_width: u16,
    /// The Ratatui table state which contains offset and selection.
    table_state: TableState,
}

/// User keyboard/mouse events are translated into actions that modify the application state.
#[derive(Clone)]
enum UserAction {
    None,
    Quit,
    NavIntoLevel(usize),
    NavUpLevel,
}

impl CommandApplication for BrowseApp {
    fn run(&mut self) -> Result<()> {
        let path: &Path = self.args.file.as_path();
        let mut parser: Parser<'_, File> = super::parse_file(path, true)?;
        let parse_result = DicomRoot::parse(&mut parser);

        let dcmroot = match parse_result {
            Ok(Some(dcmroot)) => dcmroot,
            Ok(None) => return Err(anyhow!("Not valid DICOM.")),
            Err(err) => return Err(anyhow!(err)),
        };

        let doc_model = DicomDocumentModel::parse(path, &dcmroot);

        let mut terminal = self.init()?;

        let app_result = self.run_loop(&mut terminal, &dcmroot, doc_model);

        self.close(terminal)?;

        app_result?;

        Ok(())
    }
}

impl<'app> DicomDocumentModel<'app> {
    fn parse(path: &'app Path, dcmroot: &DicomRoot) -> DicomDocumentModel<'app> {
        let node_models = DicomNodeModel::parse(dcmroot.as_obj());
        let count = node_models.len();
        DicomDocumentModel {
            file_path: path,
            node_models,
            node_views: HashMap::with_capacity(count),
        }
    }
}

impl<'model> DicomNodeModel<'model> {
    fn parse(dcmobj: &DicomObject) -> HashMap<TagPath, DicomNodeModel<'model>> {
        let total_sub_items = dcmobj.item_count() + dcmobj.child_count();
        let mut map: HashMap<TagPath, DicomNodeModel<'model>> =
            HashMap::with_capacity(total_sub_items);
        let mut rows: Vec<Row<'model>> = Vec::with_capacity(total_sub_items);
        let mut max_name_width: u16 = 0;
        for item in dcmobj.iter_items() {
            let (row, child_map, name_len) = DicomNodeModel::parse_dcmobj(item);
            rows.push(row);
            map.extend(child_map);
            max_name_width = max_name_width.max(name_len);
        }
        for (_child_tag, child) in dcmobj.iter_child_nodes() {
            let (row, child_map, name_len) = DicomNodeModel::parse_dcmobj(child);
            rows.push(row);
            map.extend(child_map);
            max_name_width = max_name_width.max(name_len);
        }

        let elem_tbl = DicomNodeModel {
            rows,
            max_name_width,
        };

        let tagpath = get_tagpath(dcmobj);
        map.insert(tagpath, elem_tbl);

        map
    }

    fn parse_dcmobj(
        child: &DicomObject,
    ) -> (Row<'model>, HashMap<TagPath, DicomNodeModel<'model>>, u16) {
        let mut map: HashMap<TagPath, DicomNodeModel<'model>> = HashMap::new();
        let child_tag = child.element().tag();
        if child.item_count() > 0 || child.child_count() > 0 {
            let child_map = DicomNodeModel::parse(child);
            map.extend(child_map);
        }

        let tag_render: TagCategory = child.element().into();
        let elem_name = tag_render.to_string();
        let name_len = u16::try_from(elem_name.len()).unwrap_or(u16::MAX);
        let elem_value: TagValue = ElementWithLineFmt(child.element(), false).into();

        let mut cells: Vec<Cell> = Vec::with_capacity(5);
        cells.push(
            Cell::from(if child.child_count() > 0 || child.item_count() > 0 {
                "+"
            } else {
                ""
            })
            .style(Style::default().fg(Color::DarkGray)),
        );

        cells.push(
            Cell::from(Tag::format_tag_to_display(child_tag))
                .style(Style::default().fg(Color::DarkGray)),
        );

        match tag_render {
            TagCategory::Known(_, _) => {
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
            Cell::from(child.element().vr().ident).style(Style::default().fg(Color::DarkGray)),
        );

        let cell = match elem_value {
            TagValue::Sequence => Cell::from(""),
            TagValue::Error(_err_str) => {
                Cell::from("<InvalidValue>").style(Style::default().fg(Color::Red))
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

        (Row::new(cells), map, name_len)
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

    fn run_loop<'d>(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        dcmroot: &'d DicomRoot,
        mut doc_model: DicomDocumentModel<'app>,
    ) -> Result<()> {
        let root_path = TagPath::empty();
        let default_table_state = TableState::new().with_selected(Some(0));
        let filename = doc_model
            .file_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let mut current_tagpath = root_path.clone();
        let mut user_action = UserAction::None;

        loop {
            if let UserAction::Quit = user_action {
                break;
            }

            let Some(table_model) = doc_model.node_models.get(&current_tagpath) else {
                return Err(BrowseError::InvalidTagPath(current_tagpath).into());
            };

            let mut view_state = doc_model
                .node_views
                .remove(&current_tagpath)
                .unwrap_or_else(|| DicomNodeViewState {
                    num_rows: table_model.rows.len(),
                    max_name_width: table_model.max_name_width,
                    table_state: default_table_state.clone(),
                    dataset_title: if current_tagpath.is_empty() {
                        filename.clone()
                    } else {
                        TagPath::format_tagpath_to_display(
                            &current_tagpath,
                            Some(dcmroot.dictionary()),
                        )
                    },
                });

            // Ratatui's Table requires an iterator over owned Rows, so the model must be cloned
            // every render, apparently. The render_stateful_widget() function requires moving a
            // Table into it, so even if the Table was lifted up into view_state or similar, some
            // sort of clone would have to be passed into rendering.
            let render_model = table_model.clone();
            // The view_state is small and intended to be cloned every iteration.
            let render_view_state = view_state.clone();

            terminal.draw(|frame| self.render(render_model, render_view_state, frame))?;

            // Check for user event. If the user event would modify the ViewState it will also be
            // updated (table offset/selection).
            user_action = self.process_user_input(&mut view_state)?;

            // Update the table state after updating offset/selection from user input, but prior to
            // updating/modifying the path if the user navigated away from the current node.
            doc_model
                .node_views
                .insert(current_tagpath.clone(), view_state);

            current_tagpath = self.get_tagpath_from_user_action(
                dcmroot,
                &doc_model,
                &user_action,
                current_tagpath,
            )?;
        }
        Ok(())
    }

    /// Polls for user input events and updates `ViewState` based on the user's interaction.
    fn process_user_input(&self, view_state: &mut DicomNodeViewState) -> Result<UserAction> {
        let user_action = if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Key(key) => match key.kind {
                    KeyEventKind::Press => self.event_keypress(view_state, key),
                    KeyEventKind::Release => self.event_keyrelease(view_state, key),
                    _ => UserAction::None,
                },
                Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(button) | MouseEventKind::Drag(button) => {
                        self.event_mouse_down(view_state, mouse, button)
                    }
                    MouseEventKind::ScrollDown => self.event_mouse_scroll_down(view_state, mouse),
                    MouseEventKind::ScrollUp => self.event_mouse_scroll_up(view_state, mouse),
                    _ => UserAction::None,
                },
                _ => UserAction::None,
            }
        } else {
            UserAction::None
        };

        Ok(user_action)
    }

    /// Returns the path the user has navigated to based on `ViewState::user_nav`.
    fn get_tagpath_from_user_action(
        &self,
        dcmroot: &DicomRoot,
        doc_model: &'app DicomDocumentModel<'app>,
        user_action: &'app UserAction,
        current_tagpath: TagPath,
    ) -> Result<TagPath> {
        // Handle user navigation
        match user_action {
            UserAction::None => {}
            UserAction::Quit => {}
            UserAction::NavIntoLevel(sel_idx) => {
                let sel_idx = *sel_idx;
                let next_path = if current_tagpath.is_empty() {
                    get_nth_child(dcmroot.as_obj(), sel_idx)
                        .map(|o| o.element().create_tagpath())
                        .unwrap_or_else(|| current_tagpath.clone())
                } else {
                    if dcmroot.get_child_by_tagpath(&current_tagpath).is_none() {
                        return Err(BrowseError::InvalidTagPath(current_tagpath).into());
                    }
                    dcmroot
                        .get_child_by_tagpath(&current_tagpath)
                        .and_then(|c| {
                            // Check items first and children second. Sequences will have a
                            // single child which is the delimiter at the end.
                            if c.item_count() > 0 {
                                if sel_idx < c.item_count() {
                                    c.get_item_by_index(sel_idx + 1)
                                } else if c.child_count() > 0 {
                                    // Subtract the # items because children appear after items
                                    // when both are present.
                                    get_nth_child(c, sel_idx - c.item_count())
                                } else {
                                    None
                                }
                            } else if c.child_count() > 0 {
                                get_nth_child(c, sel_idx)
                            } else {
                                None
                            }
                        })
                        .map(get_tagpath)
                        .unwrap_or_else(|| current_tagpath.clone())
                };

                if doc_model.node_models.contains_key(&next_path) {
                    return Ok(next_path);
                }
            }
            UserAction::NavUpLevel => {
                return Ok(get_prev_path(current_tagpath));
            }
        }
        Ok(current_tagpath)
    }

    fn event_keyrelease(
        &self,
        _view_state: &mut DicomNodeViewState,
        _event: KeyEvent,
    ) -> UserAction {
        UserAction::None
    }

    fn event_keypress(
        &self,
        view_state: &'app mut DicomNodeViewState,
        event: KeyEvent,
    ) -> UserAction {
        match event.code {
            Char('q') | KeyCode::Esc => UserAction::Quit,

            Char('l') | KeyCode::Right | KeyCode::Enter => {
                if let Some(selected) = view_state.table_state.selected() {
                    UserAction::NavIntoLevel(selected)
                } else {
                    UserAction::None
                }
            }
            Char('h') | KeyCode::Left | KeyCode::Backspace => UserAction::NavUpLevel,
            Char('j') | KeyCode::Down => {
                self.table_select_next(view_state, 1);
                UserAction::None
            }
            Char('k') | KeyCode::Up => {
                self.table_select_next(view_state, -1);
                UserAction::None
            }
            Char('d') => {
                if event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.table_select_next(view_state, 15);
                }
                UserAction::None
            }
            Char('u') => {
                if event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.table_select_next(view_state, -15);
                }
                UserAction::None
            }
            _ => UserAction::None,
        }
    }

    fn event_mouse_down(
        &self,
        view_state: &'app mut DicomNodeViewState,
        event: MouseEvent,
        button: MouseButton,
    ) -> UserAction {
        if button != MouseButton::Left {
            return UserAction::None;
        }

        // Convert the event row (all widgets on screen) into the table row.
        // Subtract 2, 1 for the table border, 1 for the table header row.
        let row_index = usize::from(event.row.saturating_sub(2));

        let index = Some(view_state.table_state.offset().saturating_add(row_index));
        // Only toggle the selection on click, not drag.
        if view_state.table_state.selected() == index
            && event.kind == MouseEventKind::Down(MouseButton::Left)
        {
            view_state.table_state.select(None)
        } else {
            view_state.table_state.select(index);
        }

        UserAction::None
    }

    fn event_mouse_scroll_up(
        &self,
        view_state: &'app mut DicomNodeViewState,
        _event: MouseEvent,
    ) -> UserAction {
        self.table_scroll_next(view_state, -1);
        UserAction::None
    }

    fn event_mouse_scroll_down(
        &self,
        view_state: &'app mut DicomNodeViewState,
        _event: MouseEvent,
    ) -> UserAction {
        self.table_scroll_next(view_state, 1);
        UserAction::None
    }

    fn table_scroll_next(&self, view_state: &'app mut DicomNodeViewState, modifier: isize) {
        let i = view_state
            .table_state
            .offset()
            .saturating_add_signed(modifier)
            .min(view_state.num_rows)
            .max(0);
        *view_state.table_state.offset_mut() = i;
    }

    fn table_select_next(&self, view_state: &'app mut DicomNodeViewState, modifier: isize) {
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

    fn render(&self, model: DicomNodeModel, mut view_state: DicomNodeViewState, frame: &mut Frame) {
        let column_widths = [
            Constraint::Length(1),
            Constraint::Length(11),
            Constraint::Length(view_state.max_name_width),
            Constraint::Length(2),
            Constraint::Max(1024),
        ];

        let table = Table::new(model.rows, column_widths)
            .header(
                Row::new(vec!["+", "Tag", "Name", "VR", "Value"]).style(
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ),
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
                            format!("[{}]", &view_state.dataset_title),
                            Style::default()
                                .fg(Color::LightBlue)
                                .add_modifier(Modifier::BOLD),
                        )))
                        .alignment(Alignment::Right),
                    )
                    .borders(Borders::all()),
            )
            .highlight_style(Style::default().bg(Color::Rgb(64, 64, 64)));

        let sections = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(frame.size());

        frame.render_stateful_widget(table, sections[0], &mut view_state.table_state);
    }
}

/// Treates a DICOM element's children as an ordered list to get a child node based on index. This
/// is only useful for mapping the view-index to the model-index.
fn get_nth_child(dcmobj: &DicomObject, index: usize) -> Option<&DicomObject> {
    dcmobj.iter_child_nodes().skip(index).map(|e| e.1).next()
}

/// Computes the `TagPath` for a given node within a DICOM document. This uses
/// `DicomElement::get_tagpath()` with two modifications:
/// - For elements at the root of the document this returns `TagPath::empty()`.
/// - For items within a sequence the trailing `constants::tags::ITEM` node is removed.
fn get_tagpath(dcmobj: &DicomObject) -> TagPath {
    let tagpath = dcmobj.element().create_tagpath();
    strip_last_item(tagpath)
}

/// Builds the previous path based on the given path.
fn get_prev_path(mut current: TagPath) -> TagPath {
    if current.is_empty() {
        return TagPath::empty();
    }

    // If the last item is indexed, instead of removing the node remove the index.
    if let Some(last) = current.nodes.last_mut() {
        if last.item().is_some() {
            last.item_mut().take();
            return current;
        }
    }

    // Remove the last node.
    current
        .nodes
        .drain(..current.nodes.len().saturating_sub(1))
        .collect::<Vec<TagNode>>()
        .into()
}

/// Removes the last node in the given path if it's an ITEM.
fn strip_last_item(mut tagpath: TagPath) -> TagPath {
    if let Some(last) = tagpath.nodes.last() {
        if last.tag() == constants::tags::ITEM {
            return tagpath
                .nodes
                .drain(..tagpath.nodes.len().saturating_sub(1))
                .collect::<Vec<TagNode>>()
                .into();
        }
    }
    tagpath
}
