use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend,
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{self, Color, Modifier, Style}, 
    text::{Span, Spans, Text}, 
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget}, Frame};
// use unicode_width::UnicodeWidthStr;



/*
pub trait CliWidget {
    fn draw(&self) -> Result<Box<impl tui::widgets::Widget>, Box<dyn std::error::Error>>;
}

 Notice I used this trait to make the widget system expandable 
 But due to an issue in wrapping a tui::Widget object in a Box 
 was unable to fisplay or render the particular widget.
*/



///////////////////////////////////////////////////////////////////////////////////////////
// 
// Title Widget
// 
pub struct TitleWidget {
    title: String
}
impl TitleWidget {
    pub fn new(title:String) -> Self{
        TitleWidget {title: title}
    }
}
impl TitleWidget {
    pub fn draw(&self) -> Result<Paragraph, Box<dyn std::error::Error>> {
        let title = Paragraph::new("test").style(Style::default())
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(style::Color::White))
            .border_type(BorderType::Plain),);

        Ok(title)
    }
}





///////////////////////////////////////////////////////////////////////////////////////////
// 
// HelpMessage
// 
pub struct HelpMessage {
    mode: TermMode
}
impl HelpMessage {
    pub fn new() -> Self{
        HelpMessage {mode: TermMode::Normal}
    }
}
impl HelpMessage {
    pub fn draw(&self) -> Result<Paragraph, Box<dyn std::error::Error>> {
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
        Ok(help_message)
    }
}





///////////////////////////////////////////////////////////////////////////////////////////
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
    messages: Vec<ListItem<'a>>,
    chunks: Vec<Rect>,
    input_wiget: Box<dyn Widget>,
    message_widget: Box<dyn Widget>,
    character_index: usize
}

// Using the Ratatui Example, https://ratatui.rs/examples/apps/user_input/
impl InputEditorWidget<'_> {
    pub fn new<B:Backend>(f: Frame<B>) -> Self {
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

        InputEditorWidget { 
            input: "".to_string(), 
            mode: TermMode::Normal, 
            messages: vec![], 
            chunks,
            input_wiget: Box::new(Paragraph::new("")),
            message_widget: Box::new(Paragraph::new("")),
            character_index : 0
        }
    }
    
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }
    
    // used to represent an absolute offset within the PST file with respect to the beginning of the file
    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        let message = ListItem::new(self.input.clone());
        self.messages.push(message);
        self.input.clear();
        self.reset_cursor();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    
    pub fn draw<B: Backend>(&mut self, mut f: Frame<'_, B>) -> Result<(), Box<dyn std::error::Error>> {
        let input = Paragraph::new(self.input.as_ref())
            .style(match self.mode {
                TermMode::Normal => Style::default(),
                TermMode::Editing => Style::default().fg(Color::Yellow),
            })
             .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, self.chunks[1]);
        
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{:?}: {:?}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        f.render_widget(messages, self.chunks[2]);

        if let Event::Key(key) = event::read()? {
            match self.mode {
                TermMode::Normal => match key.code {
                    KeyCode::Enter => {
                        self.mode = TermMode::Editing
                    },
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
    
                TermMode::Editing => match key.code {
                    KeyCode::Enter => self.submit_message(),
                    KeyCode::Char(to_insert) => {
                        self.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        self.delete_char();
                    }
                    KeyCode::Left => {
                        self.move_cursor_left();
                    }
                    KeyCode::Right => {
                        self.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        self.mode = TermMode::Normal;
                    }
                    _ => {}
                }
            }
        }

    
        Ok(())
    }
}
