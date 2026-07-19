use ratatui::{Frame, layout::Rect, widgets::Block};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = "History";
    let block = Block::bordered().title_top(title);
    frame.render_widget(block, area);
}
