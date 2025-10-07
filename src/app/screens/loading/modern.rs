use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
pub fn render_modern(frame: &mut Frame, message: &str, spinner_state: usize) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(5),
            Constraint::Percentage(40),
        ])
        .split(area);

    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(chunks[1]);

    let spinner = vec!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
    let spinner_char = spinner[spinner_state % spinner.len()];

    let text = format!("\n{}  {}", spinner_char, message);

    let widget = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .title("│ İşleniyor │")
            .title_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    frame.render_widget(widget, h_chunks[1]);
}