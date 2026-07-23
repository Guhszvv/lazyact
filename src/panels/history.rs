use std::collections::HashMap;

use super::Panel;
use crate::app_event::AppEvent;
use crate::github::Workflow;
use crate::input::Command;
use ratatui::widgets::ListState;
use rattles::TickedRattler;
use rattles::presets::braille as presets;

pub enum HistoryStatus {
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub enum StepStatus {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct StepEntry {
    pub name: String,
    pub status: StepStatus,
}

pub struct HistoryEntry {
    pub name: String,
    pub status: HistoryStatus,
    pub steps: Vec<StepEntry>,
    pub expanded: bool,
}

pub struct HistoryPanel {
    pub entries: Vec<HistoryEntry>,
    pub rattle: TickedRattler<presets::Dots>,
    pub list_state: ListState,
    step_map: HashMap<String, Vec<String>>,
}

impl HistoryPanel {
    pub fn new(list_state: ListState, workflows: Vec<Workflow>) -> Self {
        let step_map = workflows
            .into_iter()
            .filter_map(|wf| wf.steps.map(|s| (wf.name, s)))
            .collect();

        Self {
            entries: Vec::new(),
            rattle: presets::dots().into_ticked(),
            list_state,
            step_map,
        }
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn spinner_char(&self) -> &str {
        self.rattle.current_frame()
    }

    pub fn push_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::WorkflowStarted(name) => {
                let steps = self
                    .step_map
                    .get(&name)
                    .map(|steps| {
                        steps
                            .iter()
                            .map(|s| StepEntry {
                                name: s.clone(),
                                status: StepStatus::Pending,
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                self.entries.push(HistoryEntry {
                    name,
                    status: HistoryStatus::Running,
                    steps,
                    expanded: false,
                });
            }
            AppEvent::StepStarted(_) => {
                if let Some(entry) = self
                    .entries
                    .iter_mut()
                    .rev()
                    .find(|e| matches!(e.status, HistoryStatus::Running))
                    && let Some(step) = entry
                        .steps
                        .iter_mut()
                        .find(|s| matches!(s.status, StepStatus::Pending))
                {
                    step.status = StepStatus::Running;
                }
            }
            AppEvent::StepFinished(_) => {
                if let Some(entry) = self
                    .entries
                    .iter_mut()
                    .rev()
                    .find(|e| matches!(e.status, HistoryStatus::Running))
                    && let Some(step) = entry
                        .steps
                        .iter_mut()
                        .find(|s| matches!(s.status, StepStatus::Running))
                {
                    step.status = StepStatus::Success;
                }
            }
            AppEvent::StepError(_) => {
                if let Some(entry) = self
                    .entries
                    .iter_mut()
                    .rev()
                    .find(|e| matches!(e.status, HistoryStatus::Running))
                    && let Some(step) = entry
                        .steps
                        .iter_mut()
                        .find(|s| matches!(s.status, StepStatus::Running))
                {
                    step.status = StepStatus::Failed;
                }
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

    fn expand_selected(&mut self) {
        if let Some(item) = self.list_state.selected()
            && let Some(entry) = self.entries.get_mut(item)
        {
            entry.expanded = !entry.expanded;
        }
    }
}

impl Panel for HistoryPanel {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::SelectNext => self.select_next(),
            Command::SelectPrevious => self.select_previous(),
            Command::Expand => self.expand_selected(),
            _ => {}
        }
    }
}
