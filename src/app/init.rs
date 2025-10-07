use crate::app::screens::account_selector::AccountSelectorScreen;
use crate::app::{screen, App, Config};
use crate::utils::config::UserCredentials;
use crate::utils::initialize_client::initialize_client;
use cookie_store::CookieStore;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reqwest_cookie_store::CookieStoreMutex;
use std::sync::{Arc, Mutex};

impl App {
    pub async fn init() -> Self {
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen).expect("Failed to enter alternate screen");
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("Failed to create terminal");
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new()));
        let http_client = initialize_client(cookie_store.clone()).await.expect("Failed to initialize HTTP client");
        let config = Arc::new(Mutex::new(Config::load_from_file()));
        let accounts = config.lock().unwrap().clone().users.into_iter().map(|(_, v)| {v}).collect();
        Self {
            terminal,
            screen: screen(AccountSelectorScreen {
                accounts,
                selected_account_idx: 0,
            }),
            http_client,
            user_details: None,
            config,
            cookie_store
        }
    }

    pub async fn switch_to_user(&mut self, user_email: &str) -> UserCredentials {
        let user = self
            .config
            .lock()
            .unwrap()
            .users
            .get_mut(user_email)
            .expect("Failed to get user for switchin for switching")
            .clone();
        let cookie_store = Arc::new(CookieStoreMutex::new(user.cookies.clone()));
        self.cookie_store = cookie_store.clone();
        self.http_client = initialize_client(cookie_store.clone()).await.expect("Failed to initialize HTTP client");
        user
    }
}