use crate::app::screen;
use crate::app::screens::home::HomeScreen;
use crate::app::screens::login::LoginScreen;
use crate::app::screens::prompt::{PromptScreen, Selection};
use crate::app::screens::{Screen, ScreenAction};
use crate::requests::check_auth::check_auth;
use crate::requests::get_user_details::get_user_details;
use crate::utils::config::UserCredentials;
use async_trait::async_trait;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountSelectorScreen {
    pub selected_account_idx: usize,
    pub accounts: Vec<UserCredentials>
}

#[async_trait]
impl Screen for AccountSelectorScreen {
    async fn handle_event(&mut self, app: &mut crate::app::App, event: Option<crossterm::event::Event>) -> ScreenAction {
        if self.accounts.len() == 0 {
            let accounts: Vec<_> = {
                let cfg = app.config.lock().unwrap();
                cfg.users.clone().into_iter().collect()
            };
            let accounts = accounts.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
            if accounts.len() == 0 {
                return ScreenAction::ChangeScreen(screen(PromptScreen {
                    header: "Deysis TUI".to_string(),
                    prompt: "Giriş bilginiz bulunamadı, giriş yapmak ister misiniz?".to_string(),
                    title: "Giriş Yap".to_string(),
                    selection: Selection::Yes,
                    yes_label: "Evet".to_string(),
                    no_label: "Hayır".to_string(),
                    yes_action: Box::new(|| {
                        ScreenAction::ChangeScreen(screen(LoginScreen::default()))
                    }),
                    no_action: Box::new(|| ScreenAction::ExitApp),
                }))
            }
            self.accounts = accounts;
        }
        match event {
            Some(crossterm::event::Event::Key(key)) => {
                match key.code {
                    crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => {
                        ScreenAction::ExitApp
                    }
                    crossterm::event::KeyCode::Char('n') => {
                        ScreenAction::ChangeScreen(
                            screen(
                                LoginScreen::default()
                            )
                        )
                    }
                    crossterm::event::KeyCode::Char('d') => {
                        let current_screen = self.clone();
                        let accounts = self.accounts.clone();
                        let selected_account_idx = self.selected_account_idx.clone();
                        let selected_account_email = self.accounts.get(selected_account_idx).unwrap().email.clone();
                        let config = app.config.clone();
                        ScreenAction::ChangeScreen(
                            screen(
                                PromptScreen {
                                    header: "Deysis TUI".to_string(),
                                    prompt: "Bu hesabı silmek istediğinize emin misiniz?".to_string(),
                                    title: "Hesabı Sil".to_string(),
                                    selection: Selection::No,
                                    yes_label: "Evet".to_string(),
                                    no_label: "Hayır".to_string(),
                                    yes_action: Box::new(move || {
                                        let mut accounts = accounts.clone();
                                        accounts.remove(selected_account_idx);
                                        {
                                            let mut config = config.lock().unwrap();
                                            config.users.remove(&selected_account_email);
                                            config.save_to_file()
                                        }
                                        ScreenAction::ChangeScreen(screen(Self {
                                            selected_account_idx: 0,
                                            accounts
                                        }))
                                    }),
                                    no_action: Box::new(move || {
                                        let current_screen = current_screen.clone();
                                        ScreenAction::ChangeScreen(screen(current_screen))
                                    }),
                                }
                            )
                        )
                    }
                    crossterm::event::KeyCode::Up => {
                        if self.selected_account_idx == 0 {
                            self.selected_account_idx = self.accounts.len() - 1;
                        }
                        else {
                            self.selected_account_idx = self.selected_account_idx - 1;
                        }
                        ScreenAction::None
                    }
                    crossterm::event::KeyCode::Down => {
                        if self.selected_account_idx == self.accounts.len() - 1 {
                            self.selected_account_idx = 0;
                        }
                        else {
                            self.selected_account_idx = self.selected_account_idx + 1;
                        }
                        ScreenAction::None
                    }
                    crossterm::event::KeyCode::Enter => {
                        let selected_account = self.accounts.get(self.selected_account_idx).cloned().expect("Account not found");
                        let creds = app.switch_to_user(&selected_account.email).await;
                        let auth_check_http_client = app.http_client.clone();
                        ScreenAction::ChangeScreenAsync {
                            message: "Giriş bilgileri doğrulanıyor...".to_string(),
                            future: Box::pin(async move {
                                let auth_check = check_auth(&auth_check_http_client).await;
                                match auth_check {
                                    true => {
                                        let user_info = get_user_details(&auth_check_http_client).await;
                                        match user_info {
                                            Ok(details) => {
                                                let user_details = Arc::new(details);
                                                screen(HomeScreen {
                                                    user_details,
                                                    ..HomeScreen::default()
                                                })
                                            },
                                            Err(_e) => {
                                                screen(PromptScreen {
                                                    header: "Deysis TUI".to_string(),
                                                    prompt: "Giriş bilginiz geçersiz, giriş yapmak ister misiniz?".to_string(),
                                                    title: "Giriş Yap".to_string(),
                                                    selection: Selection::Yes,
                                                    yes_label: "Evet".to_string(),
                                                    no_label: "Hayır".to_string(),
                                                    yes_action: Box::new(move || {
                                                        ScreenAction::ChangeScreen(screen(LoginScreen {
                                                            user_creds: creds.clone(),
                                                            ..Default::default()
                                                        }))
                                                    }),
                                                    no_action: Box::new(|| ScreenAction::ExitApp),
                                                })
                                            }
                                        }
                                    }
                                    false => {
                                        screen(PromptScreen {
                                            header: "Deysis TUI".to_string(),
                                            prompt: "Giriş bilginiz geçersiz, giriş yapmak ister misiniz?".to_string(),
                                            title: "Giriş Yap".to_string(),
                                            selection: Selection::Yes,
                                            yes_label: "Evet".to_string(),
                                            no_label: "Hayır".to_string(),
                                            yes_action: Box::new(move || {
                                                ScreenAction::ChangeScreen(screen(LoginScreen {
                                                    user_creds: selected_account.clone(),
                                                    error_message: None,
                                                    ..LoginScreen::default()
                                                }))
                                            }),
                                            no_action: Box::new(|| ScreenAction::ExitApp),
                                        })
                                    }
                                }
                            })
                        }

                    }
                    _ => {
                        // Ignore other events
                        ScreenAction::None
                    }
                }
            },
            _ => {
                // Ignore other events
                ScreenAction::None
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),      // Header
                Constraint::Min(0),         // Account list
                Constraint::Length(3),      // Footer
            ])
            .split(area);

        // Header
        self.render_header(frame, chunks[0]);

        // Account list (centered)
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(chunks[1]);

        self.render_account_list(frame, h_chunks[1]);

        // Footer instructions
        let footer = Paragraph::new(" ↑↓ Seç | Enter: Giriş Yap | Esc: Çıkış | n: Yeni hesap ekle | d: Hesap sil")
            .style(Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC));
        frame.render_widget(footer, chunks[2]);
    }
}

impl AccountSelectorScreen {

    fn render_header(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let header_text = "━━━━━━━━━━━━━━━━━━━━━━━━━━━\n   👥 Hesap Seçimi   \n━━━━━━━━━━━━━━━━━━━━━━━━━━━";

        let header = Paragraph::new(header_text)
            .style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);

        frame.render_widget(header, area);
    }

    fn render_account_list(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        if self.accounts.is_empty() {
            // Show loading or empty state
            let empty = Paragraph::new("\n\n\n    Hesaplar yükleniyor...")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)));
            frame.render_widget(empty, area);
            return;
        }

        let items: Vec<ListItem> = self.accounts
            .iter()
            .enumerate()
            .map(|(idx, account)| {
                let is_selected = idx == self.selected_account_idx;

                // Format: email and username
                let display_text = if is_selected {
                    format!(" ▶  📧 {}", account.email)
                } else {
                    format!("    📧 {}", account.email)
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::White)
                };

                ListItem::new(display_text).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD))
                .title(format!(" Kayıtlı Hesaplar ({}) ", self.accounts.len()))
                .title_style(Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)));

        frame.render_widget(list, area);
    }
}