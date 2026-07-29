use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::app::Focus;

pub fn draw(frame: &mut Frame, focus: &Focus, area: Rect) {
    let text = match focus {
        Focus::Workflows => " [q] Quit │ [r] Run │ [↑/↓] Select",
        Focus::History => " [q] Quit │ [↑/↓] Select │ [Enter] Expand",
        Focus::Logs => " [q] Quit │ [↑/↓] Scroll │ [PgUp/Dn] Page │ [Home/End] Jump | [c] Clear",
    };
    // ponytail: single-line text footer, no styling/abstractions
    frame.render_widget(Paragraph::new(text), area);
}
