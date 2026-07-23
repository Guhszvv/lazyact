use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, ErrorKind, Result},
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StepYaml {
    name: Option<String>,
    run: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JobYaml {
    steps: Option<Vec<StepYaml>>,
}

#[derive(Debug, Deserialize)]
struct WorkflowYaml {
    name: Option<String>,
    jobs: Option<HashMap<String, JobYaml>>,
}

#[derive(Debug, Clone)]
pub struct Workflow {
    pub name: String,
    pub path: PathBuf,
    pub steps: Option<Vec<String>>,
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
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            let name = wf.name.clone().unwrap_or_else(|| {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            });

            workflows.push(Workflow {
                name,
                path,
                steps: Some(parse_steps(&wf)),
            });
        }
    }

    Ok(workflows)
}

fn parse_steps(wf: &WorkflowYaml) -> Vec<String> {
    let mut steps = Vec::new();

    if let Some(jobs) = &wf.jobs {
        for job in jobs.values() {
            if let Some(step_list) = &job.steps {
                for step in step_list {
                    let step_name = step
                        .name
                        .clone()
                        .or_else(|| step.run.clone())
                        .unwrap_or_else(|| "Unnamed Step".to_string());

                    steps.push(step_name);
                }
            }
        }
    }

    steps
}
