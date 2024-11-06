use convert_case::{Case, Casing};
use derive_setters::Setters;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt::Display, path::Path, time::Duration};

use crate::{
    error::{Error, Result},
    RustToolchainStep,
};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    BranchProtectionRule,
    CheckRun,
    CheckSuite,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Discussion,
    DiscussionComment,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    MergeGroup,
    Milestone,
    PageBuild,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequest,
    PullRequestReview,
    PullRequestReviewComment,
    PullRequestTarget,
    #[default]
    Push,
    RegistryPackage,
    Release,
    Status,
    Watch,
    WorkflowCall,
    WorkflowDispatch,
    WorkflowRun,
    RepositoryDispatch,
}

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
    pub on: Option<OneOrManyOrObject<String>>,
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

    pub fn add_job<T: ToString, J: Into<Job>>(mut self, id: T, job: J) -> Result<Self> {
        let key = id.to_string();
        if self.jobs.contains_key(&key) {
            return Err(Error::JobIdAlreadyExists(key.as_str().to_string()));
        }

        self.jobs.insert(key, job.into());
        Ok(self)
    }

    pub fn parse(yml: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yml)?)
    }

    pub fn generate<T: AsRef<Path>>(self, path: T) -> Result<()> {
        let path = path.as_ref();
        path.parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .map_err(Error::Io)?;

        std::fs::write(path, self.to_string()?).map_err(Error::Io)?;
        println!(
            "Generated workflow file: {}",
            path.canonicalize()?.display()
        );
        Ok(())
    }

    pub fn on<T: SetEvent>(self, a: T) -> Self {
        a.apply(self)
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout_minutes = Some(duration.as_secs() as u32 / 60);
        self
    }

    pub fn env<T: SetEnv<Self>>(self, env: T) -> Self {
        env.apply(self)
    }
}

// TODO: inline this conversion in actual usage
impl Into<OneOrManyOrObject<String>> for &str {
    fn into(self) -> OneOrManyOrObject<String> {
        OneOrManyOrObject::Single(self.to_string())
    }
}

// TODO: inline this conversion in actual usage
impl Into<OneOrManyOrObject<String>> for Vec<&str> {
    fn into(self) -> OneOrManyOrObject<String> {
        OneOrManyOrObject::Multiple(self.into_iter().map(|s| s.to_string()).collect())
    }
}

// TODO: inline this conversion in actual usage
impl<V: Into<OneOrManyOrObject<String>>> Into<OneOrManyOrObject<String>> for Vec<(&str, V)> {
    fn into(self) -> OneOrManyOrObject<String> {
        let mut map = IndexMap::new();
        for (key, value) in self {
            map.insert(key.to_string(), value.into());
        }
        OneOrManyOrObject::KeyValue(map)
    }
}

impl<S: Display, W: Into<OneOrManyOrObject<String>>> SetEvent for Vec<(S, W)> {
    fn apply(self, mut workflow: Workflow) -> Workflow {
        let val = self
            .into_iter()
            .map(|(s, w)| (s.to_string(), w.into()))
            .collect();
        workflow.on = Some(OneOrManyOrObject::KeyValue(val));
        workflow
    }
}

impl SetEvent for Vec<&str> {
    fn apply(self, workflow: Workflow) -> Workflow {
        let on = self.into_iter().map(|s| s.to_string()).collect();
        Workflow { on: Some(OneOrManyOrObject::Multiple(on)), ..workflow }
    }
}

impl SetEvent for &str {
    fn apply(self, workflow: Workflow) -> Workflow {
        let on = self.to_string();
        Workflow { on: Some(OneOrManyOrObject::Single(on)), ..workflow }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Default, Setters, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct EventConfig {
    pub event: Event,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<ActivityType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths_ignore: Option<Vec<String>>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub if_condition: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub runs_on: Option<OneOrManyOrObject<String>>,
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
            name: Some(name.to_string().to_case(Case::Title)),
            runs_on: Some(OneOrManyOrObject::Single("ubuntu-latest".to_string())),
            ..Default::default()
        }
    }

    pub fn add_step<S: AddStep>(self, step: S) -> Self {
        step.apply(self)
    }

    pub fn runs_on<T: SetEnv<Self>>(self, a: T) -> Self {
        a.apply(self)
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout_minutes = Some(duration.as_secs() as u32 / 60);
        self
    }

    pub fn env<T: SetEnv<Self>>(self, env: T) -> Self {
        env.apply(self)
    }
}

impl<T: ToString> SetRunner for T {
    fn apply(self, mut job: Job) -> Job {
        job.runs_on = Some(OneOrManyOrObject::Single(self.to_string()));
        job
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum OneOrManyOrObject<T> {
    Single(T),
    Multiple(Vec<T>),
    KeyValue(IndexMap<String, OneOrManyOrObject<T>>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum OneOrMany<T> {
    Single(T),
    Multiple(Vec<T>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
        self.name = Some(name.to_string().to_case(Case::Title));
        self
    }

    pub fn env<R: SetEnv<Self>>(self, env: R) -> Self {
        env.apply(self)
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
}

impl Step<Use> {
    pub fn uses<Owner: ToString, Repo: ToString>(owner: Owner, repo: Repo, version: u64) -> Self {
        Step {
            uses: Some(format!(
                "{}/{}@v{}",
                owner.to_string(),
                repo.to_string(),
                version.to_string()
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

    pub fn setup_rust() -> RustToolchainStep {
        RustToolchainStep::default()
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

impl Into<Step<AnyStep>> for Step<Use> {
    fn into(self) -> Step<AnyStep> {
        Step {
            id: self.id,
            name: self.name,
            if_condition: self.if_condition,
            uses: self.uses,
            with: self.with,
            run: self.run,
            env: self.env,
            timeout_minutes: self.timeout_minutes,
            continue_on_error: self.continue_on_error,
            working_directory: self.working_directory,
            retry: self.retry,
            artifacts: self.artifacts,
            marker: Default::default(),
        }
    }
}

impl Into<Step<AnyStep>> for Step<Run> {
    fn into(self) -> Step<AnyStep> {
        Step {
            id: self.id,
            name: self.name,
            if_condition: self.if_condition,
            uses: self.uses,
            with: self.with,
            run: self.run,
            env: self.env,
            timeout_minutes: self.timeout_minutes,
            continue_on_error: self.continue_on_error,
            working_directory: self.working_directory,
            retry: self.retry,
            artifacts: self.artifacts,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_specific: Option<IndexMap<Event, PermissionLevel>>,
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
    pub matrix: Option<OneOrManyOrObject<String>>,
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
