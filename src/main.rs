mod act;
mod app_event;
mod app;
mod github;
mod input;
mod panels;
mod ui;

use std::time::Duration;

use app::App;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{Event, poll, read};

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

        if poll(Duration::from_millis(50))? {
            if let Event::Key(key) = read()? {
                app.handle_key(key);
            }
        }

        while let Ok(event) = app.event_rx.try_recv() {
            app.handle_event(event);
        }
    }
    Ok(())
}
