use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use derive_setters::Setters;
use indexmap::IndexMap;

use crate::error::{Error, Result};
use crate::{Job, Jobs, Workflow};

#[derive(Setters, Clone)]
#[setters(into)]
pub struct Generate {
    workflow: Workflow,
    name: String,
}

impl Generate {
    pub fn new(workflow: Workflow) -> Self {
        let workflow = organize_job_dependency(workflow);
        Self { workflow, name: "ci.yml".to_string() }
    }

    fn check_file(&self, path: &PathBuf, content: &str) -> Result<()> {
        if let Ok(prev) = std::fs::read_to_string(path) {
            if content != prev {
                Err(Error::OutdatedWorkflow)
            } else {
                Ok(())
            }
        } else {
            Err(Error::MissingWorkflowFile(path.clone()))
        }
    }

    pub fn generate(&self) -> Result<()> {
        let comment = include_str!("./comment.yml");

        let root_dir = String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "--show-toplevel"])
                .output()?
                .stdout,
        )?;

        let path = PathBuf::from(root_dir.trim())
            .join(".github")
            .join("workflows")
            .join(self.name.as_str());

        let path = path.canonicalize()?;

        let content = format!("{}\n{}", comment, self.workflow.to_string()?);

        let result = self.check_file(&path, &content);

        if std::env::var("CI").is_ok() {
            result
        } else {
            match result {
                Ok(()) => {
                    println!("Workflow file is up-to-date: {}", path.display());
                    Ok(())
                }
                Err(Error::OutdatedWorkflow) => {
                    std::fs::write(path.clone(), content)?;
                    println!("Updated workflow file: {}", path.display());
                    Ok(())
                }
                Err(Error::MissingWorkflowFile(path)) => {
                    std::fs::create_dir_all(path.parent().ok_or(Error::IO(
                        std::io::Error::new(ErrorKind::Other, "Invalid parent dir(s) path"),
                    ))?)?;
                    std::fs::write(path.clone(), content)?;
                    println!("Generated workflow file: {}", path.display());
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    }
}

fn organize_job_dependency(mut workflow: Workflow) -> Workflow {
    let mut job_id = 0;
    let mut new_jobs = IndexMap::<String, Job>::new();

    // Iterate over all jobs
    for (id, mut job) in workflow.jobs.clone().unwrap_or_default().0.into_iter() {
        // If job has dependencies
        if let Some(dep_jobs) = &job.tmp_needs {
            // Prepare the job_ids
            let mut job_ids = Vec::<String>::new();
            for dep_job in dep_jobs.into_iter() {
                // If the job is already defined in the workflow
                if let Some(id) = workflow.get_id(&dep_job) {
                    job_ids.push(id.to_string());
                } else {
                    // Create a job-id for the job
                    let id = format!("job-{}", job_id);
                    job_id += 1;

                    // Add job id as the dependency
                    job_ids.push(id.clone());

                    // Insert the missing job into the new_jobs
                    new_jobs.insert(format!("job-{}", job_id), dep_job.clone());
                }
            }
            job.needs = Some(job_ids);
        }

        new_jobs.insert(id.clone(), job.clone());
    }

    workflow.jobs = Some(Jobs(new_jobs));

    workflow
}
