//!
//! The serde representation of Github Actions Workflow.

use std::fmt::Display;

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Result;
use crate::generate::Generate;
use crate::{private, Event};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Jobs(pub(crate) IndexMap<String, Job>);
impl Jobs {
    pub fn insert(&mut self, key: String, value: Job) {
        self.0.insert(key, value);
    }
}

/// Represents the configuration for a GitHub workflow.
///
/// A workflow is a configurable automated process made up of one or more jobs.
/// This struct defines the properties that can be set in a workflow YAML file
/// for GitHub Actions, including the name, environment variables, permissions,
/// jobs, concurrency settings, and more.
#[derive(Debug, Default, Setters, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Workflow {
    /// The name of the workflow. GitHub displays the names of your workflows
    /// under your repository's "Actions" tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Environment variables that can be used in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    /// The name for workflow runs generated from the workflow.
    /// GitHub displays the workflow run name in the list of workflow runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,

    /// The event that triggers the workflow. This can include events like
    /// `push`, `pull_request`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<Event>,

    /// Permissions granted to the `GITHUB_TOKEN` for the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,

    /// The jobs that are defined in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Jobs>,

    /// Concurrency settings for the workflow, allowing control over
    /// how jobs are executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,

    /// Default settings for jobs in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,

    /// Secrets that can be used in the workflow, such as tokens or passwords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,

    /// The maximum number of minutes a job can run before it is canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
}

/// Represents an action that can be triggered by an event in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct EventAction {
    /// A list of branches that trigger the action.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches: Vec<String>,

    /// A list of branches that are ignored for the action.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches_ignore: Vec<String>,
}

impl Workflow {
    /// Creates a new `Workflow` with the specified name.
    pub fn new<T: ToString>(name: T) -> Self {
        Self { name: Some(name.to_string()), ..Default::default() }
    }

    /// Converts the `Workflow` to a YAML string representation.
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    /// Adds a job to the workflow with the specified ID and job configuration.
    pub fn add_job<T: ToString, J: Into<Job>>(mut self, id: T, job: J) -> Self {
        let key = id.to_string();
        let mut jobs = self.jobs.unwrap_or_default();

        jobs.insert(key, job.into());

        self.jobs = Some(jobs);
        self
    }

    /// Parses a YAML string into a `Workflow`.
    pub fn parse(yml: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yml)?)
    }

    /// Generates the workflow using the `Generate` struct.
    pub fn generate(self) -> Result<()> {
        Generate::new(self).generate()
    }

    /// Adds an event to the workflow.
    pub fn add_event<T: Into<Event>>(mut self, that: T) -> Self {
        if let Some(mut this) = self.on.take() {
            this.merge(that.into());
            self.on = Some(this);
        } else {
            self.on = Some(that.into());
        }
        self
    }

    /// Adds an environment variable to the workflow.
    pub fn add_env<T: Into<Env>>(mut self, new_env: T) -> Self {
        let mut env = self.env.unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.env = Some(env);
        self
    }

    /// Performs a reverse lookup to get the ID of a job.
    pub fn get_id(&self, job: &Job) -> Option<&str> {
        self.jobs
            .as_ref()?
            .0
            .iter()
            .find(|(_, j)| *j == job)
            .map(|(id, _)| id.as_str())
    }
}

/// Represents the type of activity in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Created,
    Edited,
    Deleted,
}

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
        RunsOn(value.into())
    }
}

/// Represents a job in the workflow.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub(crate) needs: Option<Vec<String>>,

    #[serde(skip)]
    pub(crate) tmp_needs: Option<Vec<Job>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub cond: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_on: Option<RunsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<IndexMap<String, Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl Job {
    /// Creates a new `Job` with the specified name and default settings.
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: Some(name.to_string()),
            runs_on: Some(RunsOn(Value::from("ubuntu-latest"))),
            ..Default::default()
        }
    }

    /// Adds a step to the job.
    pub fn add_step<S: Into<Step<T>>, T: StepType>(mut self, step: S) -> Self {
        let mut steps = self.steps.unwrap_or_default();
        let step: Step<T> = step.into();
        let step: StepValue = T::to_value(step);
        steps.push(step);
        self.steps = Some(steps);
        self
    }

    /// Adds an environment variable to the job.
    pub fn add_env<T: Into<Env>>(mut self, new_env: T) -> Self {
        let mut env = self.env.unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.env = Some(env);
        self
    }

    pub fn add_needs<J: Into<Job>>(mut self, needs: J) -> Self {
        let job: Job = needs.into();
        let mut needs = self.tmp_needs.unwrap_or_default();
        needs.push(job);
        self.tmp_needs = Some(needs);
        self
    }
}

/// Represents a step in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Step<A> {
    /// The value of the step.
    value: StepValue,
    #[serde(skip)]
    marker: std::marker::PhantomData<A>,
}

impl From<Step<Run>> for StepValue {
    /// Converts a `Step<Run>` into a `StepValue`.
    fn from(step: Step<Run>) -> Self {
        step.value
    }
}

impl From<Step<Use>> for StepValue {
    /// Converts a `Step<Use>` into a `StepValue`.
    fn from(step: Step<Use>) -> Self {
        step.value
    }
}

/// Represents a step that uses an action.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Use;

/// Represents a step that runs a command.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Run;

/// A trait to convert `Step<Run>` and `Step<Use>` to `StepValue`.
pub trait StepType: Sized + private::Sealed {
    /// Converts a step to its value representation.
    fn to_value(s: Step<Self>) -> StepValue;
}

impl private::Sealed for Run {}
impl private::Sealed for Use {}

impl StepType for Run {
    /// Converts a `Step<Run>` to `StepValue`.
    fn to_value(s: Step<Self>) -> StepValue {
        s.into()
    }
}

impl StepType for Use {
    /// Converts a `Step<Use>` to `StepValue`.
    fn to_value(s: Step<Self>) -> StepValue {
        s.into()
    }
}

/// Represents environment variables in the workflow.
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Env(IndexMap<String, Value>);

impl From<IndexMap<String, Value>> for Env {
    /// Converts an `IndexMap` into an `Env`.
    fn from(value: IndexMap<String, Value>) -> Self {
        Env(value)
    }
}

impl Env {
    /// Sets the `GITHUB_TOKEN` environment variable.
    pub fn github() -> Self {
        let mut map = IndexMap::new();
        map.insert(
            "GITHUB_TOKEN".to_string(),
            Value::from("${{ secrets.GITHUB_TOKEN }}"),
        );
        Env(map)
    }

    /// Creates a new `Env` with a specified key-value pair.
    pub fn new<K: ToString, V: Into<Value>>(key: K, value: V) -> Self {
        Env::default().add(key, value)
    }

    /// Adds an environment variable to the `Env`.
    pub fn add<T1: ToString, T2: Into<Value>>(mut self, key: T1, value: T2) -> Self {
        self.0.insert(key.to_string(), value.into());
        self
    }
}

/// Represents input parameters for a step.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Input(#[serde(skip_serializing_if = "IndexMap::is_empty")] IndexMap<String, Value>);

impl From<IndexMap<String, Value>> for Input {
    /// Converts an `IndexMap` into an `Input`.
    fn from(value: IndexMap<String, Value>) -> Self {
        Input(value)
    }
}

impl Merge for Input {
    /// Merges another `Input` into this one.
    fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl Input {
    /// Adds a new input parameter to the `Input`.
    pub fn add<S: ToString, V: Into<Value>>(mut self, key: S, value: V) -> Self {
        self.0.insert(key.to_string(), value.into());
        self
    }

    /// Checks if the `Input` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Represents a step value in the workflow.
#[allow(clippy::duplicated_attributes)]
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Step<Run>", field = "value"),
    generate_delegates(ty = "Step<Use>", field = "value")
)]
pub struct StepValue {
    /// The ID of the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The condition under which the step runs.
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub if_condition: Option<Expression>,

    /// The action to use in the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub uses: Option<String>,

    /// Input parameters for the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Input>,

    /// The command to run in the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub run: Option<String>,

    /// Environment variables for the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    /// The timeout for the step in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,

    /// Whether to continue on error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,

    /// The working directory for the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,

    /// The retry strategy for the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,

    /// Artifacts produced by the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl StepValue {
    /// Creates a new `StepValue` that runs a command.
    pub fn run<T: ToString>(cmd: T) -> Self {
        StepValue { run: Some(cmd.to_string()), ..Default::default() }
    }

    /// Creates a new `StepValue` that uses an action.
    pub fn uses<Owner: ToString, Repo: ToString, Version: ToString>(
        owner: Owner,
        repo: Repo,
        version: Version,
    ) -> Self {
        StepValue {
            uses: Some(format!(
                "{}/{}@v{}",
                owner.to_string(),
                repo.to_string(),
                version.to_string()
            )),
            ..Default::default()
        }
    }
}

/// Represents a step in the workflow.
impl<T> Step<T> {
    /// Adds an environment variable to the step.
    pub fn add_env<R: Into<Env>>(mut self, new_env: R) -> Self {
        let mut env = self.value.env.unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.value.env = Some(env);
        self
    }
}

/// Represents a step that runs a command.
impl Step<Run> {
    /// Creates a new `Step<Run>` that runs a command.
    pub fn run<T: ToString>(cmd: T) -> Self {
        Step { value: StepValue::run(cmd), marker: Default::default() }
    }
}

/// Represents a step that uses an action.
impl Step<Use> {
    /// Creates a new `Step<Use>` that uses an action.
    pub fn uses<Owner: ToString, Repo: ToString, Version: ToString>(
        owner: Owner,
        repo: Repo,
        version: Version,
    ) -> Self {
        Step {
            value: StepValue::uses(owner, repo, version),
            marker: Default::default(),
        }
    }

    /// Creates a step pointing to the default GitHub's Checkout Action.
    pub fn checkout() -> Step<Use> {
        Step::uses("actions", "checkout", 4.0).name("Checkout Code")
    }

    /// Adds a new input to the step.
    pub fn add_with<I: Into<Input>>(mut self, new_with: I) -> Self {
        let mut with = self.value.with.unwrap_or_default();
        with.merge(new_with.into());
        if with.0.is_empty() {
            self.value.with = None;
        } else {
            self.value.with = Some(with);
        }

        self
    }
}

/// Represents a key-value pair for inputs.
impl<S1: ToString, S2: ToString> From<(S1, S2)> for Input {
    /// Converts a tuple into an `Input`.
    fn from(value: (S1, S2)) -> Self {
        let mut index_map: IndexMap<String, Value> = IndexMap::new();
        index_map.insert(value.0.to_string(), Value::String(value.1.to_string()));
        Input(index_map)
    }
}

/// Represents environment variables as key-value pairs.
impl<S1: Display, S2: Display> From<(S1, S2)> for Env {
    /// Converts a tuple into an `Env`.
    fn from(value: (S1, S2)) -> Self {
        let mut index_map: IndexMap<String, Value> = IndexMap::new();
        index_map.insert(value.0.to_string(), Value::String(value.1.to_string()));
        Env(index_map)
    }
}

/// Represents the runner environment for jobs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    #[default]
    Linux,
    MacOS,
    Windows,
    Custom(String),
}

/// Represents a container configuration for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Container {
    /// The image to use for the container.
    pub image: String,

    /// Credentials for accessing the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,

    /// Environment variables for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    /// Ports to expose from the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,

    /// Volumes to mount in the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,

    /// Additional options for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,

    /// Hostname for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

/// Represents credentials for accessing a container.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Credentials {
    /// The username for authentication.
    pub username: String,

    /// The password for authentication.
    pub password: String,
}

/// Represents a network port.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Port {
    /// A port specified by its number.
    Number(u16),

    /// A port specified by its name.
    Name(String),
}

/// Represents a volume configuration for containers.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Volume {
    /// The source path of the volume.
    pub source: String,

    /// The destination path of the volume.
    pub destination: String,
}

impl Volume {
    /// Creates a new `Volume` from a string representation.
    pub fn new(volume_str: &str) -> Option<Self> {
        let parts: Vec<&str> = volume_str.split(':').collect();
        if parts.len() == 2 {
            Some(Volume {
                source: parts[0].to_string(),
                destination: parts[1].to_string(),
            })
        } else {
            None
        }
    }
}

/// Represents concurrency settings for workflows.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Concurrency {
    /// The group name for concurrency.
    pub group: String,

    /// Whether to cancel in-progress jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_in_progress: Option<bool>,

    /// The limit on concurrent jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents permissions for the `GITHUB_TOKEN`.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Permissions {
    /// Permissions for actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Level>,

    /// Permissions for repository contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Level>,

    /// Permissions for issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Level>,

    /// Permissions for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<Level>,

    /// Permissions for deployments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Level>,

    /// Permissions for checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Level>,

    /// Permissions for statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Level>,

    /// Permissions for packages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Level>,

    /// Permissions for pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Level>,

    /// Permissions for ID tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<Level>,
}

/// Represents the level of permissions.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    Read,
    Write,
    #[default]
    None,
}

/// Represents the strategy for running jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Strategy {
    /// The matrix for job execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Value>,

    /// Whether to fail fast on errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,

    /// The maximum number of jobs to run in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<u32>,
}

/// Represents an environment for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Environment {
    /// The name of the environment.
    pub name: String,

    /// The URL associated with the environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Represents default settings for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Defaults {
    /// Default settings for running jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunDefaults>,

    /// Default retry settings for jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryDefaults>,

    /// Default concurrency settings for jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
}

/// Represents default settings for running commands.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct RunDefaults {
    /// The shell to use for running commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    /// The working directory for running commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// Represents default settings for retries.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RetryDefaults {
    /// The maximum number of retry attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

/// Represents an expression used in conditions.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Expression(String);

impl Expression {
    /// Creates a new `Expression` from a string.
    pub fn new<T: ToString>(expr: T) -> Self {
        Self(expr.to_string())
    }
}

/// Represents a secret required for the workflow.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Secret {
    /// Indicates if the secret is required.
    pub required: bool,

    /// A description of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Represents a strategy for retrying jobs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RetryStrategy {
    /// The maximum number of retry attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

/// Represents artifacts produced by jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Artifacts {
    /// Artifacts to upload after the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Vec<Artifact>>,

    /// Artifacts to download before the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<Vec<Artifact>>,
}

/// Represents an artifact produced by a job.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Artifact {
    /// The name of the artifact.
    pub name: String,

    /// The path to the artifact.
    pub path: String,

    /// The number of days to retain the artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}
