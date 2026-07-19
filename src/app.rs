use crate::github::Workflow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    History,
    Workflow,
    Popup,
}

pub struct App {
    pub workflows: Vec<Workflow>,
    pub focus: Focus,
}

impl App {
    pub fn new() -> Self {
        let workflows = match crate::github::work_dir() {
            Ok(Some(path)) => crate::github::list_actions(&path).unwrap_or_default(),
            _ => Vec::new(),
        };
        Self {
            workflows,
            focus: Focus::Workflow,
        }
    }
}
