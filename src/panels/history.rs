use crate::app_event::AppEvent;
use crate::input::Command;
use ratatui::widgets::ListState;
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
    pub list_state: ListState,
}

impl HistoryPanel {
    pub fn new(list_state: ListState) -> Self {
        Self {
            entries: Vec::new(),
            rattle: presets::dots().into_ticked(),
            list_state,
        }
    }
    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
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
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::SelectNext => self.select_next(),
            Command::SelectPrevious => self.select_previous(),
            _ => {}
        }
    }
}
