use crate::input::Command;

use super::Panel;

pub struct HistoryPanel;

impl HistoryPanel {
    pub fn new() -> Self {
        Self
    }
}

impl Panel for HistoryPanel {
    fn handle_command(&mut self, _command: Command) {
        // TODO: implementar histórico de execuções
    }
}
