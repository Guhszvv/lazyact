use ratatui::crossterm::event::{KeyEvent, MouseEvent};

use crate::app_event::{AppEvent, EventReceiver};
use crate::input::{Command, KeyMap};
use crate::panels::{HistoryPanel, LogsPanel, Panel, WorkflowPanel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Workflows,
    History,
    Logs,
}

pub struct App {
    pub running: bool,
    pub keymap: KeyMap,
    pub focus: Focus,
    pub workflow_panel: WorkflowPanel,
    pub history_panel: HistoryPanel,
    pub logs_panel: LogsPanel,
    pub event_rx: EventReceiver,
}

impl App {
    pub fn new() -> Self {
        let workflows = match crate::github::work_dir() {
            Ok(Some(path)) => crate::github::list_actions(&path).unwrap_or_default(),
            _ => Vec::new(),
        };
        let state = ratatui::widgets::ListState::default().with_selected(Some(0));
        let (tx, rx) = crate::app_event::new_event_channel();
        Self {
            running: true,
            keymap: KeyMap::new(),
            focus: Focus::Workflows,
            workflow_panel: WorkflowPanel::new(workflows, state, tx),
            history_panel: HistoryPanel::new(state),
            logs_panel: LogsPanel::new(),
            event_rx: rx,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let Some(command) = self.keymap.resolve(key) else {
            return;
        };
        self.handle_command(command);
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent) {
        use ratatui::crossterm::event::MouseEventKind;
        let command = match mouse.kind {
            MouseEventKind::ScrollUp => Command::ScrollUp,
            MouseEventKind::ScrollDown => Command::ScrollDown,
            _ => return,
        };
        self.handle_command(command);
    }

    pub fn handle_event(&mut self, event: AppEvent) {
        self.history_panel.push_event(event.clone());
        self.logs_panel.push_event(event);
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Quit => self.running = false,
            Command::FocusWorkflow => self.focus = Focus::Workflows,
            Command::FocusHistory => self.focus = Focus::History,
            Command::FocusLogs => self.focus = Focus::Logs,
            cmd => self.focused_panel_mut().handle_command(cmd),
        }
    }

    fn focused_panel_mut(&mut self) -> &mut dyn Panel {
        match self.focus {
            Focus::Workflows => &mut self.workflow_panel,
            Focus::History => &mut self.history_panel,
            Focus::Logs => &mut self.logs_panel,
        }
    }
}
