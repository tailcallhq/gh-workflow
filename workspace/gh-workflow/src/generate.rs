use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::Workflow;

pub struct Generate {
    workflow: Workflow,
    path: PathBuf,
}

impl Generate {
    pub fn new<P: AsRef<Path>>(workflow: Workflow, path: P) -> Self {
        Self { workflow, path: path.as_ref().to_path_buf() }
    }

    pub fn generate(&self) -> Result<()> {
        let comment = include_str!("./comment.yml");
        let path = self.path.as_path();

        path.parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .map_err(Error::IO)?;

        let content = format!("{}\n{}", comment, self.workflow.to_string()?);

        #[cfg(feature = "validate")]
        crate::validate::validate(&content, path)?;

        std::fs::write(path, content).map_err(Error::IO)?;

        println!(
            "Generated workflow file: {}",
            path.canonicalize()?.display()
        );
        Ok(())
    }
}
