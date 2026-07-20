use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::app_event::{AppEvent, EventSender};

pub async fn act_run_workflow(name: String, path: &Path, tx: EventSender) {
    tx.send(AppEvent::WorkflowStarted(name)).ok();

    let child = Command::new("act")
        .arg("-W")
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            tx.send(AppEvent::WorkflowError(e.to_string())).ok();
            return;
        }
    };

    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");

    let tx_stdout = tx.clone();
    let stdout_task = tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if tx_stdout.send(AppEvent::Stdout(line)).is_err() {
                break;
            }
        }
    });

    let tx_stderr = tx.clone();
    let stderr_task = tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if tx_stderr.send(AppEvent::Stderr(line)).is_err() {
                break;
            }
        }
    });

    let _ = tokio::join!(stdout_task, stderr_task);

    match child.wait().await {
        Ok(status) => {
            tx.send(AppEvent::WorkflowFinished(status.code().unwrap_or(-1)))
                .ok();
        }
        Err(e) => {
            tx.send(AppEvent::WorkflowError(e.to_string())).ok();
        }
    }
}
