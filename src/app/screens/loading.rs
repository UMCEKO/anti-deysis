mod modern;

use crate::app::screens::{Screen, ScreenAction};
use async_trait::async_trait;
use futures::task::noop_waker;
use ratatui::Frame;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct GenericLoadingScreen {
    message: String,
    future: Pin<Box<dyn Future<Output = Box<dyn Screen>> + Send>>,
    spinner_state: usize,
}

impl GenericLoadingScreen {
    pub fn new(
        message: String,
        future: Pin<Box<dyn Future<Output = Box<dyn Screen>> + Send>>,
    ) -> Self {
        Self {
            message,
            future,
            spinner_state: 0,
        }
    }
}

#[async_trait]
impl Screen for GenericLoadingScreen {
    fn render(&self, frame: &mut Frame) {
        modern::render_modern(frame, &self.message, self.spinner_state);
    }

    async fn handle_event(
        &mut self,
        _app: &mut crate::app::App,
        _event: Option<crossterm::event::Event>,
    ) -> ScreenAction {

        // Increment spinner
        self.spinner_state += 1;

        // Poll the future (non-blocking)
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        match self.future.as_mut().poll(&mut context) {
            Poll::Ready(screen) => {
                // Future completed - transition to the screen
                ScreenAction::ChangeScreen(screen)
            }
            Poll::Pending => {
                // Still loading - stay on this screen
                ScreenAction::None
            }
        }
    }
}