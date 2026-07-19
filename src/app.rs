use crate::github::Workflow;

pub struct App {
    pub workflows: Vec<Workflow>,
}

impl App {
    pub fn new() -> Self {
        let workflows = match crate::github::work_dir() {
            Ok(Some(path)) => crate::github::list_actions(&path).unwrap_or_default(),
            _ => Vec::new(),
        };
        Self { workflows }
    }
}
