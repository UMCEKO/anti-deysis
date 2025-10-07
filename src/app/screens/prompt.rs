use crate::app::screens::{Screen, ScreenAction};
use async_trait::async_trait;
use crossterm::event::KeyCode;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;


pub struct PromptScreen {
    pub selection: Selection,
    pub header: String,
    pub title: String,
    pub prompt: String,
    pub yes_label: String,
    pub no_label: String,
    pub yes_action: Box<dyn Fn() -> ScreenAction + Send>,
    pub no_action: Box<dyn Fn() -> ScreenAction + Send>,
}

#[derive(Clone)]
pub enum Selection {
    Yes,
    No
}

impl Default for PromptScreen {
    fn default() -> Self {
        Self {
            selection: Selection::Yes,
            header: "Dummy".into(),
            title: "Dummy".into(),
            prompt: "Dummy".into(),
            yes_label: "Yes".into(),
            no_label: "No".into(),
            yes_action: Box::new(|| ScreenAction::None),
            no_action: Box::new(|| ScreenAction::None),
        }
    }
}

#[async_trait]
impl Screen for PromptScreen {
    async fn handle_event(&mut self, _app: &mut crate::app::App, event: Option<crossterm::event::Event>) -> ScreenAction {
        match event {
            Some(crossterm::event::Event::Key(key)) => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                        ScreenAction::ExitApp
                    }
                    KeyCode::Enter => {
                        match self.selection {
                            Selection::Yes => {
                                (self.yes_action)()
                            }
                            Selection::No => {
                                (self.no_action)()
                            }
                        }
                    }
                    KeyCode::Right | KeyCode::Left => {
                        self.selection = match self.selection {
                            Selection::Yes => Selection::No,
                            Selection::No => Selection::Yes,
                        };
                        ScreenAction::None
                    }
                    _ => {
                        ScreenAction::None
                    }
                }
            }
            _ => {
                // Ignore other events
                ScreenAction::None
            }
        }
    }
    fn render(&self, f: &mut Frame) {
        let area = f.area();
        // Create a centered container with better spacing
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Percentage(30),
            ])
            .split(area);

        // ASCII Art Logo
        let logo = vec![
            format!("╔═══{}═══╗", "═".repeat(self.header.len())),
            format!("║   {}   ║", self.header),
            format!("╚═══{}═══╝", "═".repeat(self.header.len())),
        ];
        let logo_text = logo.join("\n");
        let logo_widget = Paragraph::new(logo_text)
            .style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(logo_widget, vertical_chunks[0]);

        // Prompt message with styled a border
        let prompt = Paragraph::new(self.prompt.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta))
                    .border_type(BorderType::Rounded)
                    .title(self.title.as_str())
                    .title_style(Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD))
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(prompt, vertical_chunks[1]);

        // Yes/No options with a centered layout
        let options_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Length(14),
                Constraint::Length(2),
                Constraint::Length(14),
                Constraint::Percentage(35),
            ])
            .split(vertical_chunks[2]);

        // Yes button
        let yes_style = if matches!(self.selection, Selection::Yes) {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::DIM)
        };

        let yes_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if matches!(self.selection, Selection::Yes) {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let yes_widget = Paragraph::new(format!(" ✔ {}", self.yes_label))
            .block(yes_block)
            .style(yes_style)
            .alignment(Alignment::Center);
        f.render_widget(yes_widget, options_layout[1]);

        // No button
        let no_style = if matches!(self.selection, Selection::No) {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::DIM)
        };

        let no_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if matches!(self.selection, Selection::No) {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let no_widget = Paragraph::new(format!(" ✘ {}", self.no_label))
            .block(no_block)
            .style(no_style)
            .alignment(Alignment::Center);
        f.render_widget(no_widget, options_layout[3]);

        // Hint text
        let hint = Paragraph::new("← → Ok tuşları ile seçim yapın | Enter ile onayla")
            .style(Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC))
            .alignment(Alignment::Center);
        f.render_widget(hint, vertical_chunks[3]);
    }
}
