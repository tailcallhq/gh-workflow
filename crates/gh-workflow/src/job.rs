//!
//! Job-related structures and implementations for GitHub workflow jobs.

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::concurrency::Concurrency;
use crate::step::{Step, StepType, StepValue};
use crate::{
    private, Artifacts, Container, Defaults, Env, Expression, Input, Permissions, RetryStrategy,
    Secrets, Strategy,
};

/// Represents the environment in which a job runs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct RunsOn(Value);

impl<T> From<T> for RunsOn
where
    T: Into<Value>,
{
    /// Converts a value into a `RunsOn` instance.
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// A trait implemented by the different kinds of jobs (`RunJob` and
/// `UsesJob`) to convert them into a serializable `JobValue`.
pub trait JobType: Default + private::Sealed {
    /// Converts a job to its value representation.
    fn to_value(j: Job<Self>) -> JobValue;
}

/// Configuration specific to jobs that run steps.
#[derive(Debug, Clone, Setters, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Job<RunJob>", field = "config")
)]
pub struct RunJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_on: Option<RunsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<crate::Environment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<IndexMap<String, Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl Default for RunJob {
    /// Creates a default `RunJob` with `runs_on` set to "ubuntu-latest".
    fn default() -> Self {
        Self {
            runs_on: Some(RunsOn(Value::from("ubuntu-latest"))),
            environment: None,
            outputs: None,
            env: None,
            defaults: None,
            timeout_minutes: None,
            continue_on_error: None,
            container: None,
            services: None,
            steps: None,
            retry: None,
            artifacts: None,
        }
    }
}

/// Configuration specific to jobs that call a reusable workflow.
#[derive(Debug, Clone, Default, Setters, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Job<UsesJob>", field = "config")
)]
pub struct UsesJob {
    #[setters(skip)]
    pub uses: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
}

impl private::Sealed for RunJob {}
impl private::Sealed for UsesJob {}

impl JobType for RunJob {
    fn to_value(j: Job<Self>) -> JobValue {
        let Self {
            runs_on,
            environment,
            outputs,
            env,
            defaults,
            timeout_minutes,
            continue_on_error,
            container,
            services,
            steps,
            retry,
            artifacts,
        } = j.config;
        JobValue {
            runs_on,
            environment,
            outputs,
            env,
            defaults,
            timeout_minutes,
            continue_on_error,
            container,
            services,
            steps,
            retry,
            artifacts,
            ..j.value
        }
    }
}

impl JobType for UsesJob {
    fn to_value(j: Job<Self>) -> JobValue {
        let Self { uses, with, secrets } = j.config;
        JobValue { uses: Some(uses), with, secrets, ..j.value }
    }
}

/// Represents a job in the workflow.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job<J: JobType = RunJob> {
    #[serde(flatten)]
    config: J,
    #[serde(flatten)]
    value: JobValue,
}

/// The serializable value of a job in the workflow.
/// Field order matches GitHub Actions YAML structure for better readability.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Job<T>", generics = "<T: JobType>", field = "value")
)]
pub struct JobValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub cond: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub runs_on: Option<RunsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub environment: Option<crate::Environment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub outputs: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub services: Option<IndexMap<String, Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub steps: Option<Vec<StepValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub secrets: Option<Secrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub artifacts: Option<Artifacts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub with: Option<Input>,
}

impl Job {
    /// Creates a new `Job` with the specified name and default settings.
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            value: JobValue { name: Some(name.to_string()), ..Default::default() },
            ..Default::default()
        }
    }

    /// Creates a new `Job` that calls a reusable workflow.
    pub fn uses<Owner: ToString, Repo: ToString, Path: ToString, Version: ToString>(
        self,
        owner: Owner,
        repo: Repo,
        path: Path,
        version: Version,
    ) -> Job<UsesJob> {
        Job {
            config: UsesJob {
                uses: format!(
                    "{}/{}/{}@{}",
                    owner.to_string(),
                    repo.to_string(),
                    path.to_string(),
                    version.to_string()
                ),
                ..Default::default()
            },
            value: self.value,
        }
    }
}

impl<J: JobType> Job<J> {
    pub fn add_needs<T: ToString>(mut self, job_id: T) -> Self {
        if let Some(needs) = self.value.needs.as_mut() {
            needs.push(job_id.to_string());
        } else {
            self.value.needs = Some(vec![job_id.to_string()]);
        }
        self
    }
}

impl Job<RunJob> {
    /// Adds an output to the job.
    pub fn add_output<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        let mut outputs = self.config.outputs.take().unwrap_or_default();
        outputs.insert(key.to_string(), value.to_string());
        self.config.outputs = Some(outputs);
        self
    }

    /// Adds a service to the job.
    pub fn add_service<K: ToString, V: Into<Container>>(mut self, key: K, service: V) -> Self {
        let mut services = self.config.services.take().unwrap_or_default();
        services.insert(key.to_string(), service.into());
        self.config.services = Some(services);
        self
    }

    /// Adds a step to the job.
    pub fn add_step<S: Into<Step<T>>, T: StepType>(mut self, step: S) -> Self {
        let mut steps = self.config.steps.take().unwrap_or_default();
        let step: Step<T> = step.into();
        let step: StepValue = T::to_value(step);
        steps.push(step);
        self.config.steps = Some(steps);
        self
    }

    /// Adds an environment variable to the job.
    pub fn add_env<E: Into<Env>>(mut self, new_env: E) -> Self {
        let mut env = self.config.env.take().unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.config.env = Some(env);
        self
    }
}

impl Job<UsesJob> {
    /// Adds a new input to the job.
    pub fn add_with<I: Into<Input>>(mut self, new_with: I) -> Self {
        let mut with = self.config.with.take().unwrap_or_default();
        with.merge(new_with.into());
        if with.0.is_empty() {
            self.config.with = None;
        } else {
            self.config.with = Some(with);
        }

        self
    }

    /// Changes job to inherit secrets.
    /// Will silently drop any secrets added.
    /// Mutually exclusive with `add_secret`
    pub fn inherit_secrets(mut self) -> Self {
        self.config.secrets = Some(Secrets::Inherit);
        self
    }

    /// Adds a secret to the job.
    /// Will silently drop/override 'inherit_secrets' if previously called.
    /// Mutually exclusive with `inherit_secrets`
    pub fn add_secret<K: ToString, V: Into<String>>(mut self, key: K, secret: V) -> Self {
        let mut secrets = match self
            .config
            .secrets
            .take()
            .unwrap_or(Secrets::Values(IndexMap::default()))
        {
            Secrets::Inherit => IndexMap::default(),
            Secrets::Values(values) => values,
        };
        secrets.insert(key.to_string(), secret.into());
        self.config.secrets = Some(Secrets::Values(secrets));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_default_sets_runs_on() {
        let value = RunJob::to_value(Job::default());
        assert!(value.runs_on.is_some());

        // Verify it's set to "ubuntu-latest"
        if let Some(runs_on) = value.runs_on {
            assert_eq!(
                runs_on.0,
                serde_json::Value::String("ubuntu-latest".to_string())
            );
        }
    }

    #[test]
    fn test_uses_job_has_no_runs_on() {
        let job = Job::new("Reusable Job").uses("owner", "repo", ".github/workflows/ci.yml", "v1");
        let value = UsesJob::to_value(job);

        assert_eq!(
            value.uses.as_deref(),
            Some("owner/repo/.github/workflows/ci.yml@v1")
        );
        assert!(value.runs_on.is_none());
    }
}
