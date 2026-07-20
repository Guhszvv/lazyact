use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, ListItem},
};

use crate::app::{App, Focus};
use crate::panels::history::HistoryStatus;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focus == Focus::History;
    let title = "[2] History";
    let block = Block::bordered().title_top(title).border_style(if focused {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    });

    app.history_panel.rattle.tick();

    let items: Vec<ListItem> = app
        .history_panel
        .entries
        .iter()
        .map(|entry| {
            let icon = match &entry.status {
                HistoryStatus::Running => app.history_panel.rattle.current_frame(),
                HistoryStatus::Success => "",
                HistoryStatus::Failed => "",
            };
            ListItem::new(format!("{} {}", icon, entry.name))
        })
        .collect();

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let list = List::new(items);
    frame.render_widget(list, inner);
}
