use derive_setters::Setters;

use crate::error::{Error, Result};
use crate::Model;

#[derive(Setters)]
pub struct Worker {
    workflow: Model,
    file: String,
}

impl Worker {
    pub fn new(workflow: Model) -> Self {
        Self {
            workflow,
            file: "./.github/workflows/ci.yml".to_string(),
        }
    }

    fn modify(&self, workflow: Model) -> Model {
        workflow
    }

    pub fn generate(&self) -> Result<String> {
        let workflow = self.modify(self.workflow.clone());
        Ok(serde_yaml::to_string(&workflow)?)
    }

    pub async fn compare(&self, actual: Model) -> Result<()> {
        let expected = self.generate()?;
        let actual = serde_yaml::to_string(&actual)?;

        if actual != expected {
            Err(Error::WorkflowMismatch)
        } else {
            Ok(())
        }
    }
}
