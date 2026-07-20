use std::path::Path;
use tokio::process::Command;

pub async fn act_run_workflow(path: &Path) -> std::io::Result<()> {
    let output = Command::new("act").arg("-W").arg(path).output().await?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
