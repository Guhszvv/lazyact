use crate::app::{App, Focus};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let focuses = app.focus == Focus::Logs;
    let title = "[3] Logs";
    let container = Block::bordered().title_top(title).border_style(if focuses {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    });
    frame.render_widget(container, area);
}
