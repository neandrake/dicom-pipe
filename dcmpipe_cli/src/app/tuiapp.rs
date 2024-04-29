use crossterm::{self, AlternateScreen, InputEvent, KeyEvent, TerminalInput};
use tui::backend::{CrosstermBackend, Backend};
use tui::{Terminal, Frame};
use std::io::{Error, ErrorKind};
use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::RecvError;
use tui::layout::{Layout, Constraint, Rect};
use tui::widgets::{Tabs, Borders, Block, Widget, Text, Paragraph};
use tui::style::{Style, Color};
use std::path::Path;
use std::fs::File;
use dcmpipe_lib::core::dcmparser::DicomStreamParser;
use dcmpipe_lib::core::dcmobject::DicomObject;
use dcmpipe_lib::core::tagstop::TagStop;
use dcmpipe_lib::core::dcmreader::parse_stream;

pub struct TuiApp {
    openpath: String,
    should_quit: bool,
    tabs: TabState,
}

enum Event<I> {
    Input(I),
}

struct TabState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabState {
    pub fn new(titles: Vec<String>) -> TabState {
        TabState {
            titles,
            index: 0,
        }
    }

    pub fn next(&mut self) {
        if self.index == self.titles.len() - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.index == 0 {
            self.index = self.titles.len() - 1;
        } else {
            self.index -= 1;
        }
    }
}

impl TuiApp {
    pub fn new(openpath: String) -> TuiApp {
        TuiApp {
            openpath,
            should_quit: false,
            tabs: TabState::new(vec!["Viewer".to_owned(), "Editor".to_owned()]),
        }
    }

    fn on_key(&mut self, c: char) {
        match c {
            'q' | 'Q' => self.should_quit = true,
            _ => {}
        }
    }

    fn on_up(&mut self) {
    }

    fn on_down(&mut self) {
    }

    fn on_left(&mut self) {
        self.tabs.prev();
    }

    fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let path: &Path = Path::new(&self.openpath);

        if !path.is_file() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("invalid file: {}", path.display()),
            ));
        }

        let file: File = File::open(path)?;
        let mut dicom_iter: DicomStreamParser<File> =
            DicomStreamParser::new(file, TagStop::EndOfStream);

        let mut dcmobj: DicomObject = parse_stream(&mut dicom_iter)?;

        let screen: AlternateScreen = AlternateScreen::to_alternate(true)?;
        let backend: CrosstermBackend = CrosstermBackend::with_alternate_screen(screen)?;
        let mut terminal: Terminal<CrosstermBackend> = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        // Setup input handling
        let (tx, rx) = mpsc::channel();
        {
            let tx = tx.clone();
            thread::spawn(move || {
                let input: TerminalInput = crossterm::input();
                let reader = input.read_sync();
                for event in reader {
                    match event {
                        InputEvent::Keyboard(key) => {
                            if let Err(_) = tx.send(Event::Input(key.clone())) {
                                return;
                            }
                            if key == KeyEvent::Char('q') {
                                return;
                            }
                        }
                        _ => {}
                    }
                }
            });
        }

        terminal.clear()?;

        loop {
            self.draw(&mut terminal)?;
            match rx.recv().map_err(|err: RecvError| Error::new(ErrorKind::Interrupted, err))? {
                Event::Input(event) => match event {
                    KeyEvent::Char(c) => self.on_key(c),
                    KeyEvent::Left => self.on_left(),
                    KeyEvent::Up => self.on_up(),
                    KeyEvent::Right => self.on_right(),
                    KeyEvent::Down => self.on_down(),
                    _ => {}
                },
            }
            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn draw<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<(), Error> {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(3), Constraint::Max(1)].as_ref())
                .split(f.size());
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("dcmpipe"))
                .titles(&self.tabs.titles)
                .style(Style::default().fg(Color::Gray))
                .highlight_style(Style::default().fg(Color::LightBlue))
                .select(self.tabs.index)
                .render(&mut f, chunks[0]);
            match self.tabs.index {
                0 => self.draw_list(&mut f, chunks[0].inner(3)),
                1 => self.draw_list(&mut f, chunks[0].inner(3)),
                _ => {}
            };
            self.draw_text(&mut f, chunks[1]);
        })
    }

    fn draw_text<B>(&self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let text = [Text::raw("Command: ")];
        Paragraph::new(text.iter())
            .wrap(true)
            .render(f, area);
    }

    fn draw_list<B>(&self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let text = [Text::raw("DICOM Contents: ")];
        Paragraph::new(text.iter())
            .wrap(true)
            .render(f, area);
    }
}