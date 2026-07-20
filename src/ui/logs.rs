use crate::app::{App, Focus};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{
        Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
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

    let inner = container.inner(area);
    let chunks = Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)]).split(inner);

    let total_lines = app.logs_panel.lines.len();
    let view_height = (chunks[0].height as usize).saturating_sub(1);

    app.logs_panel.visible_height = view_height;
    app.logs_panel.clamp_offset();

    let offset = app.logs_panel.scroll_offset;
    let end = (offset + view_height).min(total_lines);

    let visible_lines: Vec<Line> = app.logs_panel.lines[offset..end]
        .iter()
        .map(|l| Line::from(l.as_str()))
        .collect();

    let paragraph = Paragraph::new(visible_lines).block(container);
    frame.render_widget(paragraph, area);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut state = ScrollbarState::new(total_lines.saturating_sub(view_height))
        .position(offset)
        .viewport_content_length(view_height);
    frame.render_stateful_widget(scrollbar, chunks[1], &mut state);
}
