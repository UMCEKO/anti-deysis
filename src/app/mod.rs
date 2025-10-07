mod event_loop;
mod exit;
pub mod screens;
mod init;

use crate::app::screens::Screen;
use crate::requests::get_user_details::UserDetails;
use crate::utils::config::Config;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reqwest_cookie_store::CookieStoreMutex;
use std::io::Stdout;
use std::sync::{Arc, Mutex};

pub struct App {
    screen: Box<dyn Screen>,
    cookie_store: Arc<CookieStoreMutex>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    http_client: reqwest::Client,
    user_details: Option<Arc<UserDetails>>,
    config: Arc<Mutex<Config>>
}

fn screen<S: Screen + 'static>(screen: S) -> Box<dyn Screen> {
    Box::new(screen)
}