use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

mod footer;
mod history;
mod workflows;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let vertical =
        Layout::vertical([Constraint::Length(20), Constraint::Fill(1)]).split(frame.area());
    /*
    let horizontal =
        Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).split(vertical[1]);
    */
    //footer::draw(frame, app, vertical[1]);
    history::draw(frame, app, vertical[1]);
    workflows::draw(frame, app, vertical[0]);
}
