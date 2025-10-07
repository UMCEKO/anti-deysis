mod enter_code;

use crate::app::screens::Screen;
use crate::requests::get_user_details::UserDetails;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use std::sync::{Arc, Mutex};
use crate::app::screens::account_selector::AccountSelectorScreen;
use crate::utils::config::Config;

pub struct HomeScreen {
    pub selection: HomeScreenSelection,
    pub user_details: Arc<UserDetails>
}

impl Default for HomeScreen {
    fn default() -> Self {
        Self {
            selection: HomeScreenSelection::EnterCode,
            user_details: Arc::new(UserDetails::default())
        }
    }
}

pub enum HomeScreenSelection {
    EnterCode,
    AccountSelector
}

#[async_trait]
impl Screen for HomeScreen {
    async fn handle_event(&mut self, app: &mut crate::app::App, event: Option<Event>) -> crate::app::screens::ScreenAction {
        if app.user_details.is_none() {
            app.user_details = Some(self.user_details.clone());
        }
        match event {
            Some(Event::Key(key)) => {
                match key.code {
                    KeyCode::Enter => {
                        match self.selection {
                            HomeScreenSelection::EnterCode => {
                                crate::app::screens::ScreenAction::ChangeScreen(crate::app::screen(
                                    enter_code::EnterCodeScreen::new()
                                ))
                            }
                            HomeScreenSelection::AccountSelector => {
                                let config = Arc::new(Mutex::new(Config::load_from_file()));
                                let accounts = config.lock().unwrap().clone().users.into_iter().map(|(_, v)| {v}).collect();
                                crate::app::screens::ScreenAction::ChangeScreen(crate::app::screen(
                                    AccountSelectorScreen {
                                        accounts,
                                        selected_account_idx: 0
                                    }
                                ))
                            }
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        crate::app::screens::ScreenAction::ExitApp
                    }
                    KeyCode::Up => {
                        self.selection = match self.selection {
                            HomeScreenSelection::EnterCode => HomeScreenSelection::AccountSelector,
                            HomeScreenSelection::AccountSelector => HomeScreenSelection::EnterCode,
                        };
                        crate::app::screens::ScreenAction::None
                    }
                    KeyCode::Down => {
                        self.selection = match self.selection {
                            HomeScreenSelection::EnterCode => HomeScreenSelection::AccountSelector,
                            HomeScreenSelection::AccountSelector => HomeScreenSelection::EnterCode,
                        };
                        crate::app::screens::ScreenAction::None
                    }
                    _ => {
                        // Ignore other events
                        crate::app::screens::ScreenAction::None
                    }
                }
            },
            _ => {
                // Ignore other events
                crate::app::screens::ScreenAction::None
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        // Main layout: header, content, footer
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),      // User info header
                Constraint::Min(0),         // Menu content
                Constraint::Length(3),      // Footer
            ])
            .split(area);

        // Render user info header
        self.render_user_header(frame, main_chunks[0]);

        // Content area: split into two columns
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_chunks[1]);

        // Left: Menu
        self.render_menu(frame, content_chunks[0]);

        // Right: Info panel
        self.render_info_panel(frame, content_chunks[1]);

        // Footer instructions
        let footer = Paragraph::new(" ↑↓ Seç | Enter: Onayla | Esc: Çıkış")
            .style(Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC));
        frame.render_widget(footer, main_chunks[2]);
    }

}

impl HomeScreen {
    fn render_user_header(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        // User info with role badge
        let role_display = match self.user_details.rol.as_str() {
            "OGRENCI" => "🎓 Öğrenci",
            "OGRETMEN" => "👨‍🏫 Öğretmen",
            _ => "👤 Kullanıcı",
        };

        let full_name = format!("{} {}", self.user_details.ad, self.user_details.soyad);
        let username = format!("{}", self.user_details.kullanici_adi);

        let header_text = format!(
            " {}  │  {}  │  {}",
            role_display,
            full_name,
            username
        );

        let header = Paragraph::new(header_text)
            .style(Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)));

        frame.render_widget(header, area);
    }

    fn render_menu(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let menu_items = vec![
            ("📝 Kod Gir", HomeScreenSelection::EnterCode),
            ("👤 Hesap Seç", HomeScreenSelection::AccountSelector),
        ];

        let items: Vec<ListItem> = menu_items
            .iter()
            .map(|(label, item)| {
                let is_selected = std::mem::discriminant(&self.selection)
                    == std::mem::discriminant(item);

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::DIM)
                };

                let prefix = if is_selected { " ▶ " } else { "   " };
                let text = format!("{}{}", prefix, label);

                ListItem::new(text).style(style)
            })
            .collect();

        let menu = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Magenta))
                .title(" 📋 İşlemler ")
                .title_style(Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD)));

        frame.render_widget(menu, area);
    }

    fn render_info_panel(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let info_text = match self.user_details.rol.as_str() {
            "OGRENCI" => {
                "📚 Öğrenci Paneli\n\n\
                 Yoklama kodunu girmek için\n\
                 sol menüden \"Kod Gir\"\n\
                 seçeneğini seçin.\n\n\
                 Öğretmeninizin size\n\
                 verdiği kodu girin."
            }
            "teacher" => {
                "👨‍🏫 Öğretmen Paneli\n\n\
                 Öğrencilerinizin yoklama\n\
                 kodlarını girebilecekleri\n\
                 bir sistem.\n\n\
                 Kod oluşturmak için\n\
                 web panelini kullanın."
            }
            _ => {
                "Hoş geldiniz!"
            }
        };

        let info = Paragraph::new(info_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Blue))
                .title(" ℹ️  Bilgi ")
                .title_style(Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)));

        frame.render_widget(info, area);
    }
}