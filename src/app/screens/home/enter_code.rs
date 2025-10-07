use crate::app::screen;
use crate::app::screens::home::HomeScreen;
use crate::app::screens::prompt::PromptScreen;
use crate::app::screens::ScreenAction;
use crate::requests::join_class::join_class;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub struct EnterCodeScreen {
    code: String,
}

impl EnterCodeScreen {
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }
}

#[async_trait::async_trait]
impl crate::app::screens::Screen for EnterCodeScreen {
    async fn handle_event(&mut self, app: &mut crate::app::App, event: Option<crossterm::event::Event>) -> ScreenAction {
        match event {
            Some(crossterm::event::Event::Key(key)) => {
                match key.code {
                    crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => {
                        let user_details = app.user_details.clone().unwrap();
                        ScreenAction::ChangeScreen(screen(
                            HomeScreen {
                                selection: crate::app::screens::home::HomeScreenSelection::EnterCode,
                                user_details
                            }
                        ))
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        if self.code.len() >= 6 {
                            return ScreenAction::None;
                        }
                        if c.is_ascii_digit() {
                            self.code.push(c);
                        }
                        ScreenAction::None
                    }
                    crossterm::event::KeyCode::Backspace => {
                        self.code.pop();
                        ScreenAction::None
                    }
                    crossterm::event::KeyCode::Enter => {
                        if self.code.len() < 6 {
                            return ScreenAction::None;
                        }
                        let code = self.code.clone();
                        let http_client = app.http_client.clone();
                        let user_details = app.user_details.clone().unwrap();
                        ScreenAction::ChangeScreenAsync {
                            message: "Submitting code...".into(),
                            future: Box::pin(async move {
                                let result = join_class(&http_client, code).await;
                                match result {
                                    Ok(true) => screen(
                                        PromptScreen {
                                            selection: crate::app::screens::prompt::Selection::Yes,
                                            header: "Başarılı!".into(),
                                            title: "Başarıyla yoklamaya katıldınız.".into(),
                                            prompt: "Başarıyla yoklamaya katıldınız, devam etmek ister misiniz?".into(),
                                            yes_label: "Evet".into(),
                                            no_label: "Hayır".into(),
                                            yes_action: Box::new(move || {
                                                let user_details = user_details.clone();
                                                ScreenAction::ChangeScreen(screen(
                                                    HomeScreen {
                                                        selection: crate::app::screens::home::HomeScreenSelection::EnterCode,
                                                        user_details
                                                    }
                                                ))
                                            }),
                                            no_action: Box::new(|| ScreenAction::ExitApp),
                                        }
                                    ),
                                    Ok(false) => screen(PromptScreen {
                                        selection: crate::app::screens::prompt::Selection::Yes,
                                        header: "Hata!".into(),
                                        title: "Yoklamaya katılamadınız.".into(),
                                        prompt: "Geçersiz kod girdiniz, tekrar denemek ister misiniz?".into(),
                                        yes_label: "Evet".into(),
                                        no_label: "Hayır".into(),
                                        yes_action: Box::new(|| {
                                            ScreenAction::ChangeScreen(screen(
                                                EnterCodeScreen::new()
                                            ))
                                        }),
                                        no_action: Box::new(|| ScreenAction::ExitApp),
                                    })
                                    ,
                                    Err(e) => screen(PromptScreen {
                                        selection: crate::app::screens::prompt::Selection::Yes,
                                        header: "Hata!".into(),
                                        title: "Yoklamaya katılamadınız.".into(),
                                        prompt: format!("Bir hata oluştu: {}, tekrar denemek ister misiniz?", e),
                                        yes_label: "Evet".into(),
                                        no_label: "Hayır".into(),
                                        yes_action: Box::new(|| {
                                            ScreenAction::ChangeScreen(screen(
                                                EnterCodeScreen::new()
                                            ))
                                        }),
                                        no_action: Box::new(|| ScreenAction::ExitApp),
                                    })
                                }
                            })
                        }
                    }
                    _ => ScreenAction::None,
                }
            }
            _ => ScreenAction::None,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Length(3),
                Constraint::Length(11),  // Taller for ASCII art
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Percentage(20),
            ])
            .split(area);

        // Title
        let title = Paragraph::new("📱 Yoklama Kodu Girişi")
            .style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        frame.render_widget(title, chunks[0]);

        // Instructions
        let instructions = Paragraph::new("6 haneli sayısal kodu girin")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        frame.render_widget(instructions, chunks[1]);

        // Large ASCII digit boxes
        self.render_large_digit_boxes(frame, chunks[2]);

        // Progress indicator
        let progress_text = format!("{}/6", self.code.len());
        let progress_color = if self.code.len() == 6 {
            Color::Green
        } else {
            Color::Yellow
        };

        let progress = Paragraph::new(progress_text)
            .style(Style::default().fg(progress_color))
            .alignment(Alignment::Center);
        frame.render_widget(progress, chunks[3]);

        // Footer
        let footer = Paragraph::new("Enter: Gönder | Backspace: Sil | Esc: Geri")
            .style(Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC))
            .alignment(Alignment::Center);
        frame.render_widget(footer, chunks[4]);
    }


}

impl EnterCodeScreen {
    fn render_large_digit_boxes(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(area);

        let digit_boxes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
            ])
            .split(h_chunks[1]);

        let chars: Vec<char> = self.code.chars().collect();

        for (i, box_area) in digit_boxes.iter().enumerate() {
            let digit_char = if i < chars.len() {
                Some(chars[i])
            } else {
                None
            };

            let is_active = i == chars.len();
            let is_filled = i < chars.len();

            let border_style = if is_active {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else if is_filled {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let text_style = if is_filled {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            // Get large ASCII art for this digit
            let ascii_art = match digit_char {
                Some(c) => get_large_digit(c),
                None => "     \n     \n     \n     \n     ".to_string(),
            };

            let digit_widget = Paragraph::new(ascii_art)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(border_style))
                .style(text_style)
                .alignment(Alignment::Center);

            frame.render_widget(digit_widget, *box_area);
        }
    }

}
fn get_large_digit(digit: char) -> String {
    match digit {
        '0' =>
        concat!(
            " ▄▄▄ \n",
            "█   █\n",
            "█   █\n",
            "█   █\n",
            " ▀▀▀ "
        ),
        '1' =>
            concat!(
            "  █  \n",
            " ██  \n",
            "  █  \n",
            "  █  \n",
            " ███ "
            ),
        '2' =>
        concat!(
            " ▄▄▄ \n",
            "    █\n",
            " ▄▄▄ \n",
            "█    \n",
            "▀▀▀▀ "),
        '3' =>
            concat!(
            " ▄▄▄ \n",
            "    █\n",
            " ▄▄▄ \n",
            "    █\n",
            " ▀▀▀ "),
        '4' =>
            concat!(
            "█   █\n",
            "█   █\n",
            "▀▀▀▀█\n",
            "    █\n",
            "    █"),
        '5' =>
            concat!(
            " ▄▄▄ \n",
            "█    \n",
            "▀▀▀▄ \n",
            "    █\n",
            " ▀▀▀ "),
        '6' =>
            concat!(
            " ▄▄▄ \n",
            "█    \n",
            "█▄▄▄ \n",
            "█   █\n",
            " ▀▀▀ "),
        '7' =>
            concat!(
            " ▄▄▄ \n",
            "    █\n",
            "   █ \n",
            "  █  \n",
            "  █  "),
        '8' =>
            concat!(
            " ▄▄▄ \n",
            "█   █\n",
            " ▄▄▄ \n",
            "█   █\n",
            " ▀▀▀ "),
        '9' =>
            concat!(
            " ▄▄▄ \n",
            "█   █\n",
            " ▀▀▀█\n",
            "    █\n",
            " ▀▀▀ "),
        _ =>
            "
  ?


     ",
    }.to_string()
}
