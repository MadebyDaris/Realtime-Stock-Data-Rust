use std::borrow::Borrow;

use crossterm::{event::{self, Event, KeyCode}, terminal};
use tui::{backend::{self, Backend, CrosstermBackend}, layout::{Constraint, Direction, Layout}, Frame, Terminal};

pub mod ui;

// Editing mode or viewing mode
// Trait given to all editing Cli interfaces
pub trait CliClient {
    fn run_app<B:Backend>(&mut self, terminal: &mut Terminal<B>);
    fn start_ui<B:Backend>(f: &mut Frame<B>, app: Client) -> Result<(), Box<dyn std::error::Error>>;
}   


// Our client in question
pub struct Client {
    widgets: Vec<Box<dyn ui::CliWidget>>,
}

impl Client{
    // Initiate a new Client
    fn new() -> Client {
        Client { 
            widgets: vec![] 
        }
    }
}
impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CliClient for Client {

    fn start_ui<B:Backend>(f: &mut Frame<B>, app: Client) -> Result<(), Box<dyn std::error::Error>>{

        let stdout = std::io::stdout();
        crossterm::terminal::enable_raw_mode()?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        
        loop {
            let app = app.borrow();
            // Render
            terminal.draw(|rect| ui(rect, &app))?;
            // TODO handle inputs here
        }
    
        // Restore the terminal and close application
        terminal.clear()?;
        terminal.show_cursor()?;
        crossterm::terminal::disable_raw_mode()?;
    }

    // Running the new client
    fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        loop{
            terminal.draw(|f| self.unwrap());

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
                }
            }
        }
    }
}

pub trait IOApp {
    fn info_editor() {}
    fn start_editor<B:Backend>(terminal: &mut Terminal<B>, app: &impl CliClient) {}
    fn exit_editor() {}
    
}

impl IOApp for Client {
    fn info_editor() {
        
    }
    fn start_editor<B:Backend>(terminal: &mut Terminal<B>, app: &impl CliClient) {
        
    }
    
    fn exit_editor() {

    }
}