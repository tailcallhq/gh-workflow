use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use derive_setters::Setters;

use crate::error::{Error, Result};
use crate::Workflow;

#[derive(Setters, Clone)]
#[setters(into)]
pub struct Generate {
    workflow: Workflow,
    name: String,
}

impl Generate {
    pub fn new(workflow: Workflow) -> Self {
        Self { workflow, name: "ci.yml".to_string() }
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

        if std::env::var("CI").is_ok() {
            if let Ok(prev) = std::fs::read_to_string(&path) {
                if content != prev {
                    Err(Error::OutdatedWorkflow)
                } else {
                    println!(
                        "Workflow file is up-to-date: {}",
                        path.canonicalize()?.display()
                    );
                    Ok(())
                }
            } else {
                Err(Error::MissingWorkflowFile(path.clone()))
            }
        } else {
            std::fs::create_dir_all(path.parent().ok_or(Error::IO(std::io::Error::new(
                ErrorKind::Other,
                "Invalid parent dir(s) path",
            )))?)?;

            std::fs::write(path.clone(), content)?;
            println!(
                "Generated workflow file: {}",
                path.canonicalize()?.display()
            );
            Ok(())
        }
    }
}
