use crate::app::screens::home::HomeScreen;
use crate::app::screens::loading::GenericLoadingScreen;
use crate::app::screens::login::LoginScreen;
use crate::app::screens::prompt::{PromptScreen, Selection};
use crate::app::screens::ScreenAction;
use crate::app::{screen, App, Config};
use crate::requests::check_auth::check_auth;
use crate::requests::get_user_details::get_user_details;
use crate::utils::initialize_client::initialize_client;
use cookie_store::CookieStore;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reqwest_cookie_store::CookieStoreMutex;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};

impl App {
    pub async fn init() -> Self {
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen).expect("Failed to enter alternate screen");
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("Failed to create terminal");
        let cookie_path = "cookies.json";
        let cookie_store = match Path::new(cookie_path).exists() {
            true => {
                let f = File::open(cookie_path).map(BufReader::new).expect("Failed to open cookies file");
                let load_result = cookie_store::serde::json::load(f);
                match load_result {
                    Ok(v) => v,
                    Err(_e) => {
                        std::fs::remove_file(cookie_path).expect("Failed to delete corrupted cookies file");
                        CookieStore::default()
                    }
                }
            }
            false => CookieStore::default(),
        };
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        let http_client = initialize_client(cookie_store.clone()).await.expect("Failed to initialize HTTP client");
        let auth_check_http_client = http_client.clone();
        let config = Arc::new(Mutex::new(Config::load_from_file()));
        let loading_screen = GenericLoadingScreen::new(
            "Giriş Bilgileri Kontrol Ediliyor...".to_string(),
            Box::pin(async move {
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
                                    header: "Deysis F*cker".to_string(),
                                    prompt: "Giriş bilginiz geçersiz, giriş yapmak ister misiniz?".to_string(),
                                    title: "Giriş Yap".to_string(),
                                    selection: Selection::Yes,
                                    yes_label: "Evet".to_string(),
                                    no_label: "Hayır".to_string(),
                                    yes_action: Box::new(|| {
                                        ScreenAction::ChangeScreen(screen(LoginScreen::default()))
                                    }),
                                    no_action: Box::new(|| ScreenAction::ExitApp),
                                })
                            }
                        }
                    }
                    false => {
                        screen(PromptScreen {
                            header: "Deysis F*cker".to_string(),
                            prompt: "Giriş bilginiz bulunamadı, giriş yapmak ister misiniz?".to_string(),
                            title: "Giriş Yap".to_string(),
                            selection: Selection::Yes,
                            yes_label: "Evet".to_string(),
                            no_label: "Hayır".to_string(),
                            yes_action: Box::new(|| {
                                ScreenAction::ChangeScreen(screen(LoginScreen::default()))
                            }),
                            no_action: Box::new(|| ScreenAction::ExitApp),
                        })
                    }
                }
            } ),
        );


        Self {
            terminal,
            screen: screen(loading_screen),
            cookie_store,
            cookie_path: cookie_path.to_string(),
            http_client,
            user_details: None,
            config
        }
    }
}