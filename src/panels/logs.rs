use crate::app_event::AppEvent;
use crate::input::Command;

use super::Panel;

#[derive(Default)]
pub struct LogsPanel {
    pub lines: Vec<String>,
}

impl LogsPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::WorkflowStarted => self.lines.push("▶ Workflow started".into()),
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
}

impl Panel for LogsPanel {
    fn handle_command(&mut self, _command: Command) {}
}
