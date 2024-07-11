use std::{cell::RefCell, fs::ReadDir};

use tokio::io::stdout;
use tui::{backend::Backend,
    layout::{Constraint, Direction, Layout}, 
    style::{self, Color, Modifier, Style}, 
    text::{Span, Spans, Text}, 
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget}, Frame};
use unicode_width::UnicodeWidthStr;

pub trait CliWidget{
    fn draw(&self) -> Result<Box<dyn tui::widgets::Widget>, Box<dyn std::error::Error>>;
}


// 
// Title Widget
// 
pub struct TitleWidget {
    title: String
}
// Initialization
impl TitleWidget {
    fn new(title:String) -> Self{
        TitleWidget {title: title}
    }
}
impl CliWidget for TitleWidget {
    fn draw(&self) -> Result<Box<(dyn Widget + 'static)>, Box<dyn std::error::Error>> {
        let title = Paragraph::new("test").style(Style::default())
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(style::Color::White))
            .border_type(BorderType::Plain),);

        Ok(Box::new(title))
    }
}



// 
// Input Editor Widget
// 
enum TermMode {
    Normal,
    Editing
}
pub struct InputEditorWidget<'a> {
    input: String,
    mode: TermMode,
    messages: Vec<ListItem<'a>>
}

impl InputEditorWidget<'_> {
    pub fn new() -> Self {
        InputEditorWidget { input: "".to_string(), mode: TermMode::Normal, messages: vec![] }
    }
}
impl CliWidget for InputEditorWidget<'_> {
     fn draw(&self) -> Result<Box<(dyn Widget + 'static)>, Box<dyn std::error::Error>> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        let (msg, style) = match self.mode {
            TermMode::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            TermMode::Editing => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ],
                Style::default(),
            ),
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);

        let input = Paragraph::new(self.input.as_ref())
            .style(match self.mode {
                TermMode::Normal => Style::default(),
                TermMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));
        match self.mode {
            TermMode::Normal => {}

                TermMode::Editing => {
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[1].x + self.input.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                )
            }
        }

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{:?}: {:?}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let messages =
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));

        f.render_widget(help_message, chunks[0]);
        f.render_widget(input, chunks[1]);
        f.render_widget(messages, chunks[2]);
    
        Ok(())
    }
}