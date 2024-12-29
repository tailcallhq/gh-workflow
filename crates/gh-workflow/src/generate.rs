//! This module provides functionality to customize generation of the GitHub
//! Actions workflow files.

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

/// Organizes job dependencies within a given `Workflow`.
///
/// This function iterates over all jobs in the provided `Workflow` and ensures
/// that each job's dependencies are correctly set up. If a job has dependencies
/// specified in `tmp_needs`, it checks if those dependencies are already
/// defined in the workflow. If not, it creates new job IDs for the missing
/// dependencies and inserts them into the workflow. The function then updates
/// the `needs` field of each job with the appropriate job IDs.
fn organize_job_dependency(mut workflow: Workflow) -> Workflow {
    let mut job_id = 0;
    let mut new_jobs = IndexMap::<String, Job>::new();
    let empty_map = IndexMap::default();

    let old_jobs: &IndexMap<String, Job> = workflow
        .jobs
        .as_ref()
        .map(|jobs| &jobs.0)
        .unwrap_or(&empty_map);

    // Iterate over all jobs
    for (id, mut job) in workflow.jobs.clone().unwrap_or_default().0.into_iter() {
        // If job has dependencies
        if let Some(dep_jobs) = &job.tmp_needs {
            // Prepare the job_ids
            let mut job_ids = Vec::<String>::new();
            for job in dep_jobs.iter() {
                // If the job is already available
                if let Some(id) = find_value(job, &new_jobs).or(find_value(job, old_jobs)) {
                    job_ids.push(id.to_owned());
                } else {
                    // Create a job-id for the job
                    let id = format!("job-{}", job_id);

                    // Add job id as the dependency
                    job_ids.push(id.clone());

                    // Insert the missing job into the new_jobs
                    new_jobs.insert(format!("job-{}", job_id), job.clone());

                    job_id += 1;
                }
            }
            job.needs = Some(job_ids);
        }

        new_jobs.insert(id.clone(), job.clone());
    }

    workflow.jobs = Some(Jobs(new_jobs));

    workflow
}

/// Find a job in the new_jobs or old_jobs
fn find_value<'a, K, V: PartialEq>(job: &V, map: &'a IndexMap<K, V>) -> Option<&'a K> {
    map.iter()
        .find_map(|(k, v)| if v == job { Some(k) } else { None })
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn add_needs_job() {
        let base_job = Job::new("Base job");

        let job1 =
            Job::new("The first job that has dependency for base_job").add_needs(base_job.clone());
        let job2 =
            Job::new("The second job that has dependency for base_job").add_needs(base_job.clone());

        let workflow = Workflow::new("All jobs were added to workflow")
            .add_job("base_job", base_job)
            .add_job("with-dependency-1", job1)
            .add_job("with-dependency-2", job2);

        let workflow = Generate::new(workflow).workflow;

        assert_snapshot!(workflow.to_string().unwrap());
    }

    #[test]
    fn missing_add_job() {
        let base_job = Job::new("Base job");

        let job1 =
            Job::new("The first job that has dependency for base_job").add_needs(base_job.clone());
        let job2 =
            Job::new("The second job that has dependency for base_job").add_needs(base_job.clone());

        let workflow = Workflow::new("base_job was not added to workflow jobs")
            .add_job("with-dependency-1", job1)
            .add_job("with-dependency-2", job2);

        let workflow = Generate::new(workflow).workflow;

        assert_snapshot!(workflow.to_string().unwrap());
    }
}
