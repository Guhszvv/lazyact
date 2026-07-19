use crate::app::App;
use ratatui::{Frame, layout::Rect};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget("[Space] Interact", frame.area());
}
