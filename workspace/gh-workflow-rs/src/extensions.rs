use super::schema::*;
use crate::error::{Error, Result};

impl Workflow {
    pub fn add_job(mut self, id: String, job: crate::Job) -> Result<Self> {
        if self.jobs.contains_key(&id) {
            return Err(Error::JobIdAlreadyExists(id));
        }

        self.jobs.insert(id, job);
        Ok(self)
    }
}

impl Job {
    pub fn add_step(mut self, step: crate::Step) -> Self {
        self.steps.push(step);
        self
    }
}

impl Step {
    pub fn new(step: String) -> Self {
        Self {
            run: Some(step),
            name: None,
            ..Default::default()
        }
    }
}
