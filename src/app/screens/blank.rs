use crate::app::screens::Screen;

pub(crate) struct BlankScreen;

#[async_trait::async_trait]
impl Screen for BlankScreen {
    async fn handle_event(&mut self, _app: &mut crate::app::App, _event: Option<crossterm::event::Event>) -> crate::app::screens::ScreenAction {
        crate::app::screens::ScreenAction::None
    }

    fn render(&self, f: &mut ratatui::Frame) {
        let size = f.area();
        let block = ratatui::widgets::Block::default()
            .title("Blank Screen")
            .borders(ratatui::widgets::Borders::ALL);
        f.render_widget(block, size);
    }
}