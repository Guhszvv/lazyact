use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

mod footer;
mod history;
mod logs;
mod workflows;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let main = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());
    let horizontal =
        Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).split(main[0]);
    let left = Layout::vertical([Constraint::Length(20), Constraint::Fill(1)]).split(horizontal[0]);
    workflows::draw(frame, app, left[0]);
    history::draw(frame, app, left[1]);
    logs::draw(frame, app, horizontal[1]);
    footer::draw(frame, &app.focus, main[1]);
}
