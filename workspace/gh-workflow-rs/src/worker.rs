use derive_setters::Setters;

use crate::error::{Error, Result};
use crate::Workflow;

#[derive(Setters, Debug)]
pub struct Worker {
    workflow: Workflow,
    file: String,
}

impl Worker {
    pub fn new(workflow: Workflow) -> Self {
        Self {
            workflow,
            file: "./.github/workflows/ci.yml".to_string(),
        }
    }

    fn modify(&self, workflow: Workflow) -> Workflow {
        workflow
    }

    pub fn generate(&self) -> Result<String> {
        let workflow = self.modify(self.workflow.clone());
        Ok(serde_yaml::to_string(&workflow)?)
    }

    pub async fn compare(&self, actual: Workflow) -> Result<()> {
        let expected = self.generate()?;
        let actual = serde_yaml::to_string(&actual)?;

        if actual != expected {
            Err(Error::WorkflowMismatch)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Branches, Job, Jobs, On};

    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_worker_new() {
        let workflow = Workflow {
            name: "Test Workflow".to_string(),
            on: On {
                push: Branches {
                    branches: vec!["main".to_string()],
                },
                pull_request: Branches {
                    branches: vec!["main".to_string()],
                },
            },
            jobs: Jobs {
                build: Job {
                    runs_on: "ubuntu-latest".to_string(),
                    steps: vec![],
                },
            },
        };
        let worker = Worker::new(workflow.clone());
        let generated = worker.generate().unwrap();
        assert_snapshot!(generated);
    }

    #[test]
    fn test_worker_generate() {
        let workflow = Workflow {
            name: "Test Workflow".to_string(),
            on: On {
                push: Branches {
                    branches: vec!["main".to_string()],
                },
                pull_request: Branches {
                    branches: vec!["main".to_string()],
                },
            },
            jobs: Jobs {
                build: Job {
                    runs_on: "ubuntu-latest".to_string(),
                    steps: vec![],
                },
            },
        };
        let worker = Worker::new(workflow.clone());
        let generated = worker.generate().unwrap();
        assert_snapshot!(generated);
    }

    #[tokio::test]
    async fn test_worker_compare() {
        let workflow = Workflow {
            name: "Test Workflow".to_string(),
            on: On {
                push: Branches {
                    branches: vec!["main".to_string()],
                },
                pull_request: Branches {
                    branches: vec!["main".to_string()],
                },
            },
            jobs: Jobs {
                build: Job {
                    runs_on: "ubuntu-latest".to_string(),
                    steps: vec![],
                },
            },
        };
        let worker = Worker::new(workflow.clone());
        let result = worker.compare(workflow).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_worker_compare_mismatch() {
        let workflow_1 = Workflow {
            name: "Test Workflow".to_string(),
            on: On {
                push: Branches { branches: vec!["main".to_string()] },
                pull_request: Branches { branches: vec!["main".to_string()] },
            },
            jobs: Jobs {
                build: Job {
                    runs_on: "ubuntu-latest".to_string(),
                    steps: vec![],
                },
            },
        };
    
        let workflow_2 = Workflow {
            name: "Different Workflow".to_string(),
            on: On {
                push: Branches { branches: vec!["main".to_string()] },
                pull_request: Branches { branches: vec!["main".to_string()] },
            },
            jobs: Jobs {
                build: Job {
                    runs_on: "ubuntu-latest".to_string(),
                    steps: vec![],
                },
            },
        };
    
        let worker = Worker::new(workflow_1.clone());
        let result = worker.compare(workflow_2).await;
        assert!(result.is_err());
    }
    
}
