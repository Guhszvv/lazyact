use crate::app::{App, Focus};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, app: &mut App, version: &str, area: Rect) {
    let area = centered_rect(48, 5, area);

    frame.render_widget(Clear, area);

    let focused = app.focus == Focus::Popup;
    let block = Block::default()
        .title(" Update Available ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        });

    let text = Paragraph::new(format!(
        "Version {version} is available!\nFor update rerun install command\n[esc] Close",
    ))
    .block(block)
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    frame.render_widget(text, area);
}

fn centered_rect(w: u16, h: u16, r: Rect) -> Rect {
    let vert = Layout::vertical([
        Constraint::Length((r.height.saturating_sub(h)) / 2),
        Constraint::Length(h),
        Constraint::Min(0),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Length((r.width.saturating_sub(w)) / 2),
        Constraint::Length(w),
        Constraint::Min(0),
    ])
    .split(vert[1])[1]
}
