use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
};

use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focus == Focus::History;
    let title = "[2] History";
    let block = Block::bordered().title_top(title).border_style(if focused {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    });
    frame.render_widget(block, area);
}
