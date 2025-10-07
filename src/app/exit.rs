use crate::app::App;
use std::process::exit;

impl App {
    pub fn exit(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
        crossterm::execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        ).expect("Failed to leave alternate screen");
        self.terminal.show_cursor().expect("Failed to show cursor");
        exit(0)
    }
}