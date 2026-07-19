use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, List, ListItem},
};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .workflows
        .iter()
        .map(|wf| ListItem::new(wf.name.clone()))
        .collect();
    let block = Block::bordered().title_top("Workflows");
    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
