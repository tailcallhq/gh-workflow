use std::path::PathBuf;
use std::process::Command;

use derive_setters::Setters;

use crate::error::{Error, Result};
use crate::Workflow;

#[derive(Setters, Clone)]
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
                .args(&["rev-parse", "--show-toplevel"])
                .output()?
                .stdout,
        )?;

        let path = PathBuf::from(root_dir.trim())
            .join(".github")
            .join("workflows")
            .join(self.name.as_str());

        let content = format!("{}\n{}", comment, self.workflow.to_string()?);

        std::fs::write(path.clone(), content).map_err(Error::IO)?;

        println!(
            "Generated workflow file: {}",
            path.canonicalize()?.display()
        );
        Ok(())
    }
}
