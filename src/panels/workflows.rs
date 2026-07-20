use super::Panel;
use crate::act::act_run_workflow;
use crate::app_event::EventSender;
use crate::github::Workflow;
use crate::input::Command;
use ratatui::widgets::ListState;

pub struct WorkflowPanel {
    pub workflows: Vec<Workflow>,
    pub list_state: ListState,
    pub tx: EventSender,
}

impl WorkflowPanel {
    pub fn new(workflows: Vec<Workflow>, list_state: ListState, tx: EventSender) -> Self {
        Self {
            workflows,
            list_state,
            tx,
        }
    }

    fn run_workflow(&mut self) {
        if let Some(item) = self.list_state.selected() {
            let Some(path) = self.workflows.get(item).map(|w| w.path.clone()) else {
                return;
            };
            let tx = self.tx.clone();
            tokio::spawn(async move {
                act_run_workflow(&path, tx).await;
            });
        }
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }
}

impl Panel for WorkflowPanel {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::RunWorkflow => self.run_workflow(),
            Command::SelectNext => self.select_next(),
            Command::SelectPrevious => self.select_previous(),
            _ => {}
        }
    }
}
