use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
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

    let spinner = app.history_panel.spinner_char();

    let items: Vec<ListItem> = app
        .history_panel
        .entries
        .iter()
        .map(|entry| {
            let (icon, style) = match &entry.status {
                HistoryStatus::Running => (spinner.as_str(), Style::default().fg(Color::Yellow)),
                HistoryStatus::Success => ("✓", Style::default().fg(Color::Green)),
                HistoryStatus::Failed => ("✗", Style::default().fg(Color::Red)),
            };
            let mut lines = vec![Line::from(vec![
                Span::styled(icon, style),
                Span::raw(" "),
                Span::raw(&entry.name),
            ])];
            if entry.expanded {
                let steps = &entry.steps;
                let len = steps.len();
                for (idx, step) in steps.iter().enumerate() {
                    let connector = if idx == len - 1 { "╰─" } else { "├─" };
                    let (step_icon, step_style) = match &step.status {
                        StepStatus::Pending => ("·", Style::default()),
                        StepStatus::Running => {
                            (spinner.as_str(), Style::default().fg(Color::Yellow))
                        }
                        StepStatus::Success => ("✓", Style::default().fg(Color::Green)),
                        StepStatus::Failed => ("✗", Style::default().fg(Color::Red)),
                    };
                    lines.push(Line::from(vec![
                        Span::raw("  "),                     // indent
                        Span::raw(connector),                // ├─ or ╰─
                        Span::raw(" "),                      // gap after connector
                        Span::styled(step_icon, step_style), // status icon
                        Span::raw(" "),                      // gap before name
                        Span::raw(&step.name),
                    ]));
                }
            }
            ListItem::new(Text::from(lines))
        })
        .collect();

    let list = List::new(items).block(block).highlight_symbol("> ");
    frame.render_stateful_widget(list, area, &mut app.history_panel.list_state);
}
