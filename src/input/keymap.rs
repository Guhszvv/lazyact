use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use super::command::Command;

pub struct KeyMap;

impl KeyMap {
    pub fn new() -> Self {
        Self
    }

    /// Converte um KeyEvent em um Command, se houver bind.
    pub fn resolve(&self, key: KeyEvent) -> Option<Command> {
        if key.kind != KeyEventKind::Press {
            return None;
        }
        match key.code {
            KeyCode::Char('q') => Some(Command::Quit),
            KeyCode::Char('r') => Some(Command::RunWorkflow),
            KeyCode::Char('1') => Some(Command::FocusWorkflow),
            KeyCode::Char('2') => Some(Command::FocusHistory),
            KeyCode::Char('3') => Some(Command::FocusLogs),
            KeyCode::Up => Some(Command::SelectPrevious),
            KeyCode::Down => Some(Command::SelectNext),
            KeyCode::PageUp => Some(Command::ScrollPageUp),
            KeyCode::PageDown => Some(Command::ScrollPageDown),
            KeyCode::Home => Some(Command::ScrollTop),
            KeyCode::End => Some(Command::ScrollBottom),
            KeyCode::Enter => Some(Command::Expand),
            KeyCode::Esc => Some(Command::ClosePopup),
            KeyCode::Char('c') => Some(Command::ClearLogs),
            _ => None,
        }
    }
}
