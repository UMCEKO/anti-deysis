use ratatui::Frame;
use std::pin::Pin;

pub mod prompt;
pub mod login;
pub mod loading;
pub mod blank;
pub mod home;
pub mod account_selector;

#[async_trait::async_trait]
pub trait Screen: Send {
    async fn handle_event(&mut self, app: &mut crate::app::App, event: Option<crossterm::event::Event>) -> ScreenAction;
    fn render(&self, f: &mut Frame);
}

pub enum ScreenAction {
    None,
    ChangeScreen(Box<dyn Screen>),
    ChangeScreenAsync{
        future: Pin<Box<dyn Future<Output = Box<dyn Screen>> + Send>>,
        message: String
    },
    ExitApp,
}
