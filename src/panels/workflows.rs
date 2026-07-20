use super::Panel;
use crate::github::Workflow;
use crate::input::Command;
use ratatui::widgets::ListState;

pub struct WorkflowPanel {
    pub workflows: Vec<Workflow>,
    pub list_state: ListState,
}

impl WorkflowPanel {
    pub fn new(workflows: Vec<Workflow>, list_state: ListState) -> Self {
        Self {
            workflows,
            list_state,
        }
    }
    fn run_workflow(&mut self) {
        if let Some(item) = self.list_state.selected() {
            let Some(workflow) = self.workflows.get(item) else {
                return;
            };
            println!("{}", workflow.path.display());
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
