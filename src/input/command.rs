#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Command {
    Quit,
    RunWorkflow,
    FocusWorkflow,
    FocusHistory,
    SelectNext,
    SelectPrevious,
}
