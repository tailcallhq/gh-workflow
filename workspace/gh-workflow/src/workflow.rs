#![allow(clippy::needless_update)]

use std::fmt::Display;

use derive_setters::Setters;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Result;
use crate::generate::Generate;
use crate::{Cargo, Event, EventValue, RustFlags, ToolchainStep};

#[derive(Debug, Default, Setters, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Workflow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub on: Option<EventValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub jobs: IndexMap<String, Job>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct EventAction {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches_ignore: Vec<String>,
}

impl Workflow {
    pub fn new<T: ToString>(name: T) -> Self {
        Self { name: Some(name.to_string()), ..Default::default() }
    }
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    pub fn add_job<T: ToString, J: Into<Job>>(mut self, id: T, job: J) -> Self {
        let key = id.to_string();

        self.jobs.insert(key, job.into());
        self
    }

    pub fn parse(yml: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yml)?)
    }

    pub fn generate(self) -> Result<()> {
        Generate::new(self).generate()
    }

    pub fn on<T: SetEvent>(self, a: T) -> Self {
        a.apply(self)
    }

    pub fn env<T: SetEnv<Self>>(self, env: T) -> Self {
        env.apply(self)
    }

    pub fn setup_rust() -> Self {
        let build_job = Job::new("Build and Test")
            .add_step(Step::checkout())
            .add_step(
                Step::setup_rust()
                    .with_stable_toolchain()
                    .with_nightly_toolchain()
                    .with_clippy()
                    .with_fmt(),
            )
            .add_step(Step::cargo(Cargo::test().all_features().workspace()).name("Cargo Test"))
            .add_step(Step::cargo_nightly(Cargo::fmt().check()).name("Cargo Fmt"))
            .add_step(
                Step::cargo_nightly(
                    Cargo::clippy()
                        .all_features()
                        .workspace()
                        .add_arg("--")
                        .add_arg("-D warnings"),
                )
                .name("Cargo Clippy"),
            );

        let push_event = Event::push().branch("main");

        let pr_event = Event::pull_request_target()
            .open()
            .synchronize()
            .reopen()
            .branch("main");

        let rust_flags = RustFlags::deny("warnings");

        Workflow::new("Build and Test")
            .env(rust_flags)
            .permissions(Permissions::read())
            .on(push_event)
            .on(pr_event)
            .add_job("build", build_job)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub if_condition: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub runs_on: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<AnyStep>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
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
    #[setters(skip)]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl Job {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: Some(name.to_string()),
            runs_on: Some(Value::from("ubuntu-latest")),
            ..Default::default()
        }
    }

    pub fn add_step<S: AddStep>(self, step: S) -> Self {
        step.apply(self)
    }

    pub fn runs_on<T: SetEnv<Self>>(self, a: T) -> Self {
        a.apply(self)
    }

    pub fn env<T: SetEnv<Self>>(self, env: T) -> Self {
        env.apply(self)
    }
}

impl<T: Into<Value>> SetRunner for T {
    fn apply(self, mut job: Job) -> Job {
        job.runs_on = Some(self.into());
        job
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum AnyStep {
    Run(Step<Run>),
    Use(Step<Use>),
}

impl From<Step<Run>> for AnyStep {
    fn from(step: Step<Run>) -> Self {
        AnyStep::Run(step)
    }
}

impl From<Step<Use>> for AnyStep {
    fn from(step: Step<Use>) -> Self {
        AnyStep::Use(step)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Use;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Run;

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Step<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub if_condition: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    with: Option<IndexMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,

    #[serde(skip)]
    marker: std::marker::PhantomData<T>,
}

impl<T> Step<T> {
    pub fn name<S: ToString>(mut self, name: S) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn env<R: SetEnv<Self>>(self, env: R) -> Self {
        env.apply(self)
    }
    pub fn working_directory<S: ToString>(mut self, working_directory: S) -> Self {
        self.working_directory = Some(working_directory.to_string());
        self
    }
}

impl<T> AddStep for Step<T>
where
    Step<T>: Into<AnyStep>,
{
    fn apply(self, mut job: Job) -> Job {
        let mut steps = job.steps.unwrap_or_default();
        steps.push(self.into());
        job.steps = Some(steps);

        job
    }
}

impl Step<Run> {
    pub fn run<T: ToString>(cmd: T) -> Self {
        Step { run: Some(cmd.to_string()), ..Default::default() }
    }

    fn prepare_args<P: ToString>(params: Vec<P>) -> String {
        params
            .iter()
            .map(|a| a.to_string())
            .reduce(|a, b| format!("{} {}", a, b))
            .unwrap_or_default()
    }
    fn cargo_command<T>(prefix: Option<&str>, cargo: Cargo<T>) -> Self {
        Step::run(format!(
            "cargo{} {} {}",
            prefix.map(|v| format!(" {}", v)).unwrap_or_default(),
            cargo.command.to_string().to_lowercase(),
            Self::prepare_args(cargo.args)
        ))
    }

    pub fn cargo<T>(cargo: Cargo<T>) -> Self {
        Self::cargo_command(None, cargo)
    }

    pub fn cargo_nightly<T>(cargo: Cargo<T>) -> Self {
        Self::cargo_command(Some("+nightly"), cargo)
    }
}

impl Step<Use> {
    pub fn uses<Owner: ToString, Repo: ToString>(owner: Owner, repo: Repo, version: u64) -> Self {
        Step {
            uses: Some(format!(
                "{}/{}@v{}",
                owner.to_string(),
                repo.to_string(),
                version
            )),
            ..Default::default()
        }
    }

    pub fn with<K: SetInput>(self, item: K) -> Self {
        item.apply(self)
    }

    pub fn checkout() -> Self {
        Step::uses("actions", "checkout", 4).name("Checkout Code")
    }

    pub fn setup_rust() -> ToolchainStep {
        ToolchainStep::default()
    }
}

impl SetInput for IndexMap<String, Value> {
    fn apply(self, mut step: Step<Use>) -> Step<Use> {
        let mut with = step.with.unwrap_or_default();
        with.extend(self);
        step.with = Some(with);
        step
    }
}

impl<S1: Display, S2: Display> SetInput for (S1, S2) {
    fn apply(self, mut step: Step<Use>) -> Step<Use> {
        let mut with = step.with.unwrap_or_default();
        with.insert(self.0.to_string(), Value::String(self.1.to_string()));
        step.with = Some(with);
        step
    }
}

impl<S1: Display, S2: Display> SetEnv<Job> for (S1, S2) {
    fn apply(self, mut value: Job) -> Job {
        let mut index_map: IndexMap<String, String> = value.env.unwrap_or_default();
        index_map.insert(self.0.to_string(), self.1.to_string());
        value.env = Some(index_map);
        value
    }
}

impl From<Step<Use>> for Step<AnyStep> {
    fn from(value: Step<Use>) -> Self {
        Step {
            id: value.id,
            name: value.name,
            if_condition: value.if_condition,
            uses: value.uses,
            with: value.with,
            run: value.run,
            env: value.env,
            timeout_minutes: value.timeout_minutes,
            continue_on_error: value.continue_on_error,
            working_directory: value.working_directory,
            retry: value.retry,
            artifacts: value.artifacts,
            marker: Default::default(),
        }
    }
}

impl From<Step<Run>> for Step<AnyStep> {
    fn from(value: Step<Run>) -> Self {
        Step {
            id: value.id,
            name: value.name,
            if_condition: value.if_condition,
            uses: value.uses,
            with: value.with,
            run: value.run,
            env: value.env,
            timeout_minutes: value.timeout_minutes,
            continue_on_error: value.continue_on_error,
            working_directory: value.working_directory,
            retry: value.retry,
            artifacts: value.artifacts,
            marker: Default::default(),
        }
    }
}

/// Set the `env` for Step, Job or Workflows
pub trait SetEnv<Value> {
    fn apply(self, value: Value) -> Value;
}

/// Set the `run` for a Job
pub trait SetRunner {
    fn apply(self, job: Job) -> Job;
}

/// Sets the event for a Workflow
pub trait SetEvent {
    fn apply(self, workflow: Workflow) -> Workflow;
}

/// Sets the input for a Step that uses another action
pub trait SetInput {
    fn apply(self, step: Step<Use>) -> Step<Use>;
}

/// Inserts a step into a job
pub trait AddStep {
    fn apply(self, job: Job) -> Job;
}

impl<S1: Display, S2: Display> SetEnv<Step<Use>> for (S1, S2) {
    fn apply(self, mut step: Step<Use>) -> Step<Use> {
        let mut index_map: IndexMap<String, Value> = step.with.unwrap_or_default();
        index_map.insert(self.0.to_string(), Value::String(self.1.to_string()));
        step.with = Some(index_map);
        step
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    #[default]
    Linux,
    MacOS,
    Windows,
    Custom(String),
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Container {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Port {
    Number(u16),
    Name(String),
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Volume {
    pub source: String,
    pub destination: String,
}

impl Volume {
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

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Concurrency {
    pub group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_in_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Permissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<PermissionLevel>,
}

impl Permissions {
    pub fn read() -> Self {
        Self { contents: Some(PermissionLevel::Read), ..Default::default() }
    }

    pub fn write() -> Self {
        Self { contents: Some(PermissionLevel::Write), ..Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionLevel {
    Read,
    Write,
    #[default]
    None,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Strategy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Environment {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RunDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RetryDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Expression(String);

impl Expression {
    pub fn new<T: ToString>(expr: T) -> Self {
        Self(expr.to_string())
    }
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Secret {
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RetryStrategy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Artifacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Vec<Artifact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<Vec<Artifact>>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Artifact {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}
