use crate::app_event::AppEvent;
use crate::input::Command;
use rattles::TickedRattler;
use rattles::presets::braille as presets;

use super::Panel;

pub enum HistoryStatus {
    Running,
    Success,
    Failed,
}

pub struct HistoryEntry {
    pub name: String,
    pub status: HistoryStatus,
}

pub struct HistoryPanel {
    pub entries: Vec<HistoryEntry>,
    pub rattle: TickedRattler<presets::Dots>,
}

impl HistoryPanel {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            rattle: presets::dots().into_ticked(),
        }
    }

    pub fn push_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::WorkflowStarted(name) => {
                self.entries.push(HistoryEntry {
                    name,
                    status: HistoryStatus::Running,
                });
            }
            AppEvent::WorkflowFinished(code) => {
                if let Some(entry) = self
                    .entries
                    .iter_mut()
                    .rev()
                    .find(|e| matches!(e.status, HistoryStatus::Running))
                {
                    entry.status = if code == 0 {
                        HistoryStatus::Success
                    } else {
                        HistoryStatus::Failed
                    }
                }
            }
            AppEvent::WorkflowError(_) => {
                if let Some(entry) = self
                    .entries
                    .iter_mut()
                    .rev()
                    .find(|e| matches!(e.status, HistoryStatus::Running))
                {
                    entry.status = HistoryStatus::Failed;
                }
            }
            _ => {}
        }
    }
}

impl Panel for HistoryPanel {
    fn handle_command(&mut self, _command: Command) {}
}
