pub mod history;
pub mod workflows;
pub use history::HistoryPanel;
pub use workflows::WorkflowPanel;

use crate::input::Command;

/// Interface comum para todos os painéis da TUI.
pub trait Panel {
    fn handle_command(&mut self, command: Command);
}
