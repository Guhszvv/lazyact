use crate::app::{App, Focus};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, ListItem},
};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focus == Focus::Workflow;
    let items: Vec<ListItem> = app
        .workflows
        .iter()
        .map(|wf| ListItem::new(wf.name.clone()))
        .collect();
    let block = Block::bordered()
        .title_top("[1] Workflows")
        .border_style(if focused {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        });
    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
