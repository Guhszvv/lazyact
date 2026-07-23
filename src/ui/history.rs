use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListItem},
};

use crate::app::{App, Focus};
use crate::panels::history::{HistoryStatus, StepStatus};

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focus == Focus::History;
    let title = "[2] History";
    let block = Block::bordered()
        .title_top(title)
        .border_style(if focused {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        })
        .border_type(BorderType::Rounded);

    app.history_panel.rattle.tick();

    let items: Vec<ListItem> = app
        .history_panel
        .entries
        .iter()
        .map(|entry| {
            let icon = match &entry.status {
                HistoryStatus::Running => app.history_panel.spinner_char(),
                HistoryStatus::Success => "✓",
                HistoryStatus::Failed => "✗",
            };
            let mut lines = vec![Line::from(format!("{} {}", icon, entry.name))];
            if entry.expanded {
                for step in &entry.steps {
                    let step_icon = match &step.status {
                        StepStatus::Pending => "·",
                        StepStatus::Running => app.history_panel.spinner_char(),
                        StepStatus::Success => "✓",
                        StepStatus::Failed => "✗",
                    };
                    lines.push(Line::from(format!("  {} {}", step_icon, step.name)));
                }
            }
            ListItem::new(Text::from(lines))
        })
        .collect();

    let list = List::new(items).block(block).highlight_symbol("> ");
    frame.render_stateful_widget(list, area, &mut app.history_panel.list_state);
}
