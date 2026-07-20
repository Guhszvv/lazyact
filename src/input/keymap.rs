use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use super::command::Command;

/// Centraliza o mapeamento de teclas para comandos.
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
            _ => None,
        }
    }
}
