use crate::app::screens::blank::BlankScreen;
use crate::app::screens::loading::GenericLoadingScreen;
use crate::app::screens::ScreenAction;
use crate::app::{screen, App};
use crossterm::event;
use crossterm::event::poll;
use std::time::Duration;

impl App {
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.terminal.draw(|f| {
                self.screen.render(f);
            })?;
            let event = match poll(Duration::from_millis(100)) {
                Ok(true) => Some(event::read()?),
                Ok(false) => None,
                Err(_) => break,
            };
            let mut current_screen = std::mem::replace(&mut self.screen, screen(BlankScreen));
            let screen_action = current_screen.handle_event(self, event).await;
            match screen_action {
                ScreenAction::None => {
                    self.screen = current_screen;
                }
                ScreenAction::ChangeScreen(new_screen) => {
                    self.screen = new_screen;
                }
                ScreenAction::ExitApp => {
                    break;
                }
                ScreenAction::ChangeScreenAsync {
                    future,
                    message
                } => {
                    self.screen = Box::new(GenericLoadingScreen::new(
                        message,
                        future,
                    ));
                }
            }
        }
        self.exit();
        Ok(())
    }
}