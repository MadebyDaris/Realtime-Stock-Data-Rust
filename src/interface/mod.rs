use std::{borrow::Borrow, io};

use crossterm::{event::{self, EnableMouseCapture, Event, KeyCode}, execute, terminal::{self, enable_raw_mode, EnterAlternateScreen}};
use tui::{backend::{self, Backend, CrosstermBackend}, layout::{Constraint, Direction, Layout, Rect}, widgets::Widget, Frame, Terminal};
use ui::InputEditorWidget;

pub mod ui;
pub mod parsing;

// Our client in question
pub struct app<'a> {
    term:  Terminal<CrosstermBackend<io::Stdout>> ,
    chunk: Vec<Rect>,
    editor: InputEditorWidget<'a>,
}

impl app<'_> {
    // Initiate a new Client
    pub fn new(editor: InputEditorWidget, chunk: Vec<Rect>) -> Result<app, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut term: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
        
        Ok(app {term, chunk, editor})
    }
    pub fn draw(mut self, ui: impl Fn(&mut Frame<'_, CrosstermBackend<std::io::Stdout>>, &app)) {
        self.term.draw(|f| ui(f, &self));
    } 
}

impl Drop for app<'_> {
    fn drop(&mut self) {
        // Disable raw mode and restore the original terminal state
        terminal::disable_raw_mode().unwrap();
        execute!(self.term.backend_mut(), terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture).unwrap();
    }
}