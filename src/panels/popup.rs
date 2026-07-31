use crate::input::Command;
use crate::panels::Panel;

pub struct PopupPanel {
    pub close_popup: bool,
}

impl PopupPanel {
    pub fn new() -> Self {
        Self { close_popup: false }
    }
}

impl Panel for PopupPanel {
    fn handle_command(&mut self, command: Command) {
        if self.close_popup || command != Command::ClosePopup {}
    }
}
