#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Command {
    Quit,
    RunWorkflow,
    FocusWorkflow,
    FocusHistory,
    FocusLogs,
    SelectNext,
    SelectPrevious,
    ScrollUp,
    ScrollDown,
    ScrollPageUp,
    ScrollPageDown,
    ScrollTop,
    ScrollBottom,
}
