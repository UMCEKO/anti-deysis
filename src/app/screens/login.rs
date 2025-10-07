use crate::app::screens::home::HomeScreen;
use crate::app::screens::{Screen, ScreenAction};
use crate::app::{screen, App};
use crate::requests::get_user_details::get_user_details;
use crate::requests::login::login;
use crate::utils::save_cookies::save_cookies;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use serde::{Deserialize, Serialize};

pub struct LoginScreen {
    pub active_field: ActiveField,
    error_message: Option<String>,
    user_creds: UserCreds,
}

impl Default for LoginScreen {
    fn default() -> Self {
        Self {
            active_field: ActiveField::Email,
            error_message: None,
            user_creds: UserCreds {
                email: String::new(),
                password: String::new(),
            }
        }
    }
}

#[derive(Clone)]
pub enum ActiveField {
    Email,
    Password
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserCreds {
    pub email: String,
    pub password: String,
}

#[async_trait::async_trait]
impl Screen for LoginScreen {
    async fn handle_event(&mut self, app: &mut App, event: Option<Event>) -> ScreenAction {
        {
            let config = app.config.lock().unwrap();
            if config.email.len() > 0 && config.password.len() > 0 && self.user_creds.email.len() == 0 && self.user_creds.password.len() == 0 {
                self.user_creds.email = config.email.clone();
                self.user_creds.password = config.password.clone();
            }
        }
        match event {
            Some(Event::Key(key)) => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        ScreenAction::ExitApp
                    }
                    KeyCode::Tab |
                    KeyCode::BackTab => {
                        self.active_field = match self.active_field {
                            ActiveField::Email => ActiveField::Password,
                            ActiveField::Password => ActiveField::Email,
                        };
                        ScreenAction::None
                    }
                    KeyCode::Enter => {
                        let http_client = app.http_client.clone();
                        let mut user_creds = self.user_creds.clone();
                        let cookie_store = app.cookie_store.clone();
                        let cookie_path = app.cookie_path.clone();
                        let config = app.config.clone();
                        ScreenAction::ChangeScreenAsync {
                            future: Box::pin(async move {
                                let login_result = login(&http_client, user_creds.clone()).await.map_err(|e| e.to_string());
                                save_cookies(cookie_store, cookie_path);
                                let user_details = match login_result {
                                    Ok(_) => {
                                        let details_result = get_user_details(&http_client).await;
                                        match details_result {
                                            Ok(details) => Some(std::sync::Arc::new(details)),
                                            Err(_) => None,
                                        }
                                    }
                                    Err(e) => {
                                        user_creds.password.clear();
                                        return screen(LoginScreen {
                                            active_field: ActiveField::Email,
                                            error_message: Some(format!("Giriş başarısız: {}", e)),
                                            user_creds
                                        })
                                    },
                                };

                                match user_details {
                                    Some(user_details) => {
                                        {
                                            let mut config = config.lock().unwrap();
                                            config.email = user_creds.email.clone();
                                            config.password = user_creds.password.clone();
                                            config.save_to_file();
                                        }
                                        screen(HomeScreen {
                                            user_details,
                                            ..HomeScreen::default()
                                        })
                                    }
                                    None => {
                                        user_creds.password.clear();
                                        screen(LoginScreen {
                                            active_field: ActiveField::Email,
                                            error_message: Some("Giriş başarısız: Kullanıcı bilgileri alınamadı.".to_string()),
                                            user_creds
                                        })
                                    }
                                }
                            }),
                            message: "Giriş yapılıyor...".to_string(),
                        }
                    }
                    KeyCode::Char(c) => {
                        match self.active_field {
                            ActiveField::Email => self.user_creds.email.push(c),
                            ActiveField::Password => self.user_creds.password.push(c),
                        }
                        ScreenAction::None
                    }
                    KeyCode::Backspace => {
                        match self.active_field {
                            ActiveField::Email => { self.user_creds.email.pop(); },
                            ActiveField::Password => { self.user_creds.password.pop(); },
                        }
                        ScreenAction::None
                    }
                    _ => {
                        ScreenAction::None
                    }
                }
            }
            _ => {
                ScreenAction::None
            }
        }
    }
    fn render(&self, f: &mut Frame) {
        let area = f.area();
        // Create centered layout with title
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Percentage(30),
            ])
            .split(area);

        // Title with the gradient effect
        let title = "🔑 GİRİŞ BİLGİLERİ";
        let title_widget = Paragraph::new(title)
            .style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title_widget, vertical_chunks[0]);

        // Horizontal centering for input boxes (smaller width)
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(vertical_chunks[1]);

        // Username input with active indicator
        let email_border_style = if matches!(self.active_field, ActiveField::Email) {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let email_title = if matches!(self.active_field, ActiveField::Email) {
            " 👤 Email ▸ "
        } else {
            " 👤 Email "
        };

        let email_widget = Paragraph::new(self.user_creds.email.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(email_border_style)
                    .title(email_title)
                    .title_style(Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD))
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(email_widget, h_chunks[1]);

        // Horizontal centering for the password box
        let h_chunks2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(vertical_chunks[2]);

        // Password input with active indicator
        let password_border_style = if matches!(self.active_field, ActiveField::Password) {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let password_title = if matches!(self.active_field, ActiveField::Password) {
            " 🔒 Şifre ▸ "
        } else {
            " 🔒 Şifre "
        };

        let password_display = if self.user_creds.password.is_empty() {
            String::new()
        } else {
            "●".repeat(self.user_creds.password.len())
        };

        let password_widget = Paragraph::new(password_display)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(password_border_style)
                    .title(password_title)
                    .title_style(Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD))
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(password_widget, h_chunks2[1]);

        // Instructions
        let instructions = if matches!(self.active_field, ActiveField::Email) {
            "Tab: Şifreye geç | Enter: Giriş yap | Esc: İptal"
        } else {
            "Tab: Kullanıcı adına dön | Enter: Giriş yap | Esc: İptal"
        };

        let hint_widget = Paragraph::new(instructions)
            .style(Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC))
            .alignment(Alignment::Center);
        f.render_widget(hint_widget, vertical_chunks[4]);
        // Error message display
        if let Some(error) = &self.error_message {
            let error_widget = Paragraph::new(error.as_str())
                .style(Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            f.render_widget(error_widget, vertical_chunks[3]);
        }
    }
}
impl LoginScreen {
}
