mod app;
mod github;
mod ui;
use crate::app::App;
use ratatui::{
    DefaultTerminal,
    crossterm::{self, event::KeyCode},
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = app(&mut terminal);
    ratatui::restore();
    result
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|frame| {
            ui::draw(frame, &mut app);
        })?;
        if crossterm::event::read()?
            .as_key_press_event()
            .is_some_and(|key| key.code == KeyCode::Char('q'))
        {
            break Ok(());
        }
    }
}
