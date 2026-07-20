use crate::app::{App, Focus};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    text::Line,
};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let focuses = app.focus == Focus::Logs;
    let title = "[3] Logs";
    let container = Block::bordered().title_top(title).border_style(if focuses {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    });

    let lines: Vec<Line> = app
        .logs_panel
        .lines
        .iter()
        .map(|l| Line::from(l.as_str()))
        .collect();

    let paragraph = Paragraph::new(lines).block(container);
    frame.render_widget(paragraph, area);
}
