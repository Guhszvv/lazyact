use std::{
    env,
    fs::{self, File},
    io::{self, Result},
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WorkflowYaml {
    name: Option<String>,
}

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    pub path: PathBuf,
}

pub fn work_dir() -> io::Result<Option<PathBuf>> {
    let dir = env::current_dir()?;
    let workflows_dir = dir.join(".github").join("workflows");

    if workflows_dir.is_dir() {
        Ok(Some(workflows_dir))
    } else {
        Ok(None)
    }
}

pub fn list_actions(dir: &Path) -> Result<Vec<Workflow>> {
    let mut workflows = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path
            .extension()
            .is_some_and(|ext| ext == "yml" || ext == "yaml")
        {
            let file = File::open(&path)?;
            let wf: WorkflowYaml = serde_yaml::from_reader(file)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let name = wf
                .name
                .unwrap_or_else(|| path.file_stem().unwrap_or_default().to_string_lossy().into_owned());

            workflows.push(Workflow { name, path });
        }
    }

    Ok(workflows)
}
