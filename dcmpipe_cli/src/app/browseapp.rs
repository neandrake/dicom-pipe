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
use dcmpipe_lib::defn::tag::Tag;
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

struct BrowseAppState<'app> {
    is_running: bool,
    path: &'app Path,
    dcmroot: DicomRoot<'app>,
    table_state: TableState,
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

        let mut table_state = TableState::default();
        table_state.select(Some(0));
        let state = BrowseAppState {
            is_running: true,
            path,
            dcmroot,
            table_state,
        };
        let mut terminal = self.init()?;

        let app_result = self.run_loop(&mut terminal, state);

        self.close(terminal)?;

        app_result?;

        Ok(())
    }
}

impl BrowseApp {
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

    fn run_loop(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        mut state: BrowseAppState,
    ) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(&mut state, frame))?;
            self.update_state(&mut state)?;
            if !state.is_running {
                break;
            }
        }
        Ok(())
    }

    fn update_state(&self, state: &mut BrowseAppState) -> Result<()> {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Key(key) => match key.kind {
                    KeyEventKind::Press => self.event_keypress(state, key),
                    KeyEventKind::Release => self.event_keyrelease(state, key),
                    _ => {}
                },
                Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(button) | MouseEventKind::Drag(button) => {
                        self.event_mouse_down(state, mouse, button)
                    }
                    MouseEventKind::ScrollDown => self.event_mouse_scroll_down(state, mouse),
                    MouseEventKind::ScrollUp => self.event_mouse_scroll_up(state, mouse),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn event_keypress(&self, state: &mut BrowseAppState, event: KeyEvent) {
        match event.code {
            Char('q') => state.is_running = false,
            KeyCode::Esc => state.is_running = false,
            Char('j') | KeyCode::Down => self.table_select_next(state, 1),
            Char('k') | KeyCode::Up => self.table_select_next(state, -1),
            _ => {}
        }
    }

    fn event_keyrelease(&self, _state: &mut BrowseAppState, _event: KeyEvent) {}

    fn event_mouse_down(&self, state: &mut BrowseAppState, event: MouseEvent, button: MouseButton) {
        if button != MouseButton::Left {
            return;
        }

        // Convert the event row (all widgets on screen) into the table row.
        // Subtract 2, 1 for the table border, 1 for the table header row.
        let row_index = event.row.saturating_sub(2) as usize;

        let index = Some(state.table_state.offset().saturating_add(row_index));
        if state.table_state.selected() == index {
            state.table_state.select(None)
        } else {
            state.table_state.select(index);
        }
    }

    fn event_mouse_scroll_up(&self, state: &mut BrowseAppState, _event: MouseEvent) {
        self.table_scroll_next(state, -1);
    }

    fn event_mouse_scroll_down(&self, state: &mut BrowseAppState, _event: MouseEvent) {
        self.table_scroll_next(state, 1);
    }

    fn table_scroll_next(&self, state: &mut BrowseAppState, modifier: isize) {
        let i = state
            .table_state
            .offset()
            .saturating_add_signed(modifier)
            .min(state.dcmroot.get_child_count() + 1)
            .max(0);
        *state.table_state.offset_mut() = i;
    }

    fn table_select_next(&self, state: &mut BrowseAppState, modifier: isize) {
        let i = match state.table_state.selected() {
            None => 0,
            Some(i) => state
                .dcmroot
                .get_child_count()
                .sub(1)
                .min(i.saturating_add_signed(modifier))
                .max(0),
        };
        state.table_state.select(Some(i));
    }

    fn render(&self, state: &mut BrowseAppState, frame: &mut Frame) {
        let mut max_name_width: u16 = 0;
        let mut rows: Vec<Row> = Vec::with_capacity(state.dcmroot.get_child_count());
        for (tag, dcmobj) in state.dcmroot.iter_child_nodes() {
            let tag_render: TagName = dcmobj.get_element().into();
            let elem_name = tag_render.to_string();
            max_name_width = max_name_width.max(elem_name.len() as u16);
            let elem_value: TagValue = ElementWithLineFmt(dcmobj.get_element(), false).into();

            let mut cells: Vec<Cell> = Vec::with_capacity(5);
            cells.push(
                Cell::from(if dcmobj.get_child_count() > 0 {
                    "+"
                } else {
                    ""
                })
                .style(Style::default().fg(Color::DarkGray)),
            );

            cells.push(
                Cell::from(Tag::format_tag_to_display(*tag))
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
                Cell::from(dcmobj.get_element().get_vr().ident)
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

        let column_widths = [
            Constraint::Length(1),
            Constraint::Length(11),
            Constraint::Length(max_name_width),
            Constraint::Length(2),
            Constraint::Max(1024),
        ];

        let table = Table::new(rows)
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
                            format!("[{}]", state.path.display()),
                            Style::default().fg(Color::LightBlue),
                        )))
                        .alignment(Alignment::Right),
                    )
                    .borders(Borders::all()),
            )
            .widths(&column_widths)
            .highlight_style(Style::default().bg(Color::LightBlue));

        let sections = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(frame.size());

        frame.render_stateful_widget(table, sections[0], &mut state.table_state);
    }
}
