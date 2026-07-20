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
use ratatui::crossterm::event::{self, Event, poll, read};
use ratatui::crossterm::execute;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    execute!(std::io::stdout(), event::EnableMouseCapture)?;
    let result = app(&mut terminal);
    execute!(std::io::stdout(), event::DisableMouseCapture)?;
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
            match read()? {
                Event::Key(key) => {
                    app.handle_key(key);
                }
                Event::Mouse(mouse) if app.focus == app::Focus::Logs => {
                    app.handle_mouse(mouse);
                }
                _ => {}
            }
        }

        while let Ok(event) = app.event_rx.try_recv() {
            app.handle_event(event);
        }
    }
    Ok(())
}
