use ratatui::crossterm::event::KeyEvent;
use ratatui::widgets::ListState;

use crate::input::{Command, KeyMap};
use crate::panels::{HistoryPanel, Panel, WorkflowPanel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Workflows,
    History,
}

pub struct App {
    pub running: bool,
    pub keymap: KeyMap,
    pub focus: Focus,
    pub workflow_panel: WorkflowPanel,
    pub history_panel: HistoryPanel,
}

impl App {
    pub fn new() -> Self {
        let workflows = match crate::github::work_dir() {
            Ok(Some(path)) => crate::github::list_actions(&path).unwrap_or_default(),
            _ => Vec::new(),
        };
        let state = ListState::default().with_selected(Some(0));
        Self {
            running: true,
            keymap: KeyMap::new(),
            focus: Focus::Workflows,
            workflow_panel: WorkflowPanel::new(workflows, state),
            history_panel: HistoryPanel::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let Some(command) = self.keymap.resolve(key) else {
            return;
        };
        self.handle_command(command);
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Quit => self.running = false,
            Command::FocusWorkflow => self.focus = Focus::Workflows,
            Command::FocusHistory => self.focus = Focus::History,
            cmd => self.focused_panel_mut().handle_command(cmd),
        }
    }

    fn focused_panel_mut(&mut self) -> &mut dyn Panel {
        match self.focus {
            Focus::Workflows => &mut self.workflow_panel,
            Focus::History => &mut self.history_panel,
        }
    }
}
