use crate::input::Command;

use super::Panel;

pub struct LogsPanel;

impl LogsPanel {
    pub fn new() -> Self {
        Self
    }
}

impl Panel for LogsPanel {
    fn handle_command(&mut self, _command: Command) {
        // TODO: implementar lógica de logs
    }
}
