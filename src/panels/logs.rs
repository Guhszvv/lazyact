use crate::app_event::AppEvent;
use crate::input::Command;

use super::Panel;

#[derive(Default)]
pub struct LogsPanel {
    pub lines: Vec<String>,
    pub scroll_offset: usize,
    pub auto_scroll: bool,
    pub visible_height: usize,
}

impl LogsPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::WorkflowStarted(name) => {
                self.lines.push(format!("▶ {name} started"));
            }
            AppEvent::Stdout(line) => self.lines.push(line),
            AppEvent::Stderr(line) => self.lines.push(format!("⚠ {line}")),
            AppEvent::WorkflowFinished(code) => {
                self.lines.push(format!("✓ Finished (exit code {code})"));
            }
            AppEvent::WorkflowError(err) => {
                self.lines.push(format!("✗ Error: {err}"));
            }
        }
    }

    fn max_offset(&self) -> usize {
        self.lines.len().saturating_sub(self.visible_height)
    }

    pub fn clamp_offset(&mut self) {
        let max = self.max_offset();
        if self.auto_scroll || self.scroll_offset > max {
            self.scroll_offset = max;
        }
    }
}

impl Panel for LogsPanel {
    fn handle_command(&mut self, command: Command) {
        let max = self.max_offset();
        match command {
            Command::SelectPrevious | Command::ScrollUp => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                    self.auto_scroll = false;
                }
            }
            Command::SelectNext | Command::ScrollDown => {
                if self.scroll_offset < max {
                    self.scroll_offset += 1;
                    self.auto_scroll = self.scroll_offset >= max;
                }
            }
            Command::ScrollPageUp => {
                let amount = self.visible_height.saturating_sub(1);
                self.scroll_offset = self.scroll_offset.saturating_sub(amount);
                self.auto_scroll = false;
            }
            Command::ScrollPageDown => {
                let amount = self.visible_height.saturating_sub(1);
                self.scroll_offset = self.scroll_offset.saturating_add(amount).min(max);
                self.auto_scroll = self.scroll_offset >= max;
            }
            Command::ScrollTop => {
                self.scroll_offset = 0;
                self.auto_scroll = false;
            }
            Command::ScrollBottom => {
                self.scroll_offset = max;
                self.auto_scroll = true;
            }
            _ => {}
        }
    }
}
