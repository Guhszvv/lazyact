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
    let horizontal =
        Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).split(frame.area());
    let left =
        Layout::vertical([Constraint::Length(20), Constraint::Fill(1)]).split(horizontal[0]);
    workflows::draw(frame, app, left[0]);
    history::draw(frame, app, left[1]);
    logs::draw(frame, app, horizontal[1]);
}
