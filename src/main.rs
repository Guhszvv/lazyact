mod act;
mod app;
mod github;
mod input;
mod panels;
mod ui;

use app::App;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = app(&mut terminal);
    ratatui::restore();
    result
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    while app.running {
        terminal.draw(|frame| {
            ui::draw(frame, &mut app);
        })?;
        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        }
    }
    Ok(())
}
