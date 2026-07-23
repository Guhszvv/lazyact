use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum AppEvent {
    WorkflowStarted(String),
    StepStarted(String),
    StepFinished(String),
    StepError(String),
    Stdout(String),
    Stderr(String),
    WorkflowFinished(i32),
    WorkflowError(String),
}

pub type EventSender = mpsc::UnboundedSender<AppEvent>;
pub type EventReceiver = mpsc::UnboundedReceiver<AppEvent>;

pub fn new_event_channel() -> (EventSender, EventReceiver) {
    mpsc::unbounded_channel()
}
