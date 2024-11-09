use derive_setters::Setters;
use merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(Default, Setters, Debug, Serialize, Deserialize, Clone, Merge, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[setters(strip_option)]
pub struct AnyEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_target: Option<PullRequestTarget>,
    // TODO: add all more events
}

pub struct Event<A>(A);

impl Event<Push> {
    pub fn push() -> Self {
        Event(Push::default())
    }
}

impl Event<PullRequest> {
    pub fn pull_request() -> Self {
        Event(PullRequest::default())
    }
}

impl Event<PullRequestTarget> {
    pub fn pull_request_target() -> Self {
        Event(PullRequestTarget::default())
    }
}

impl<A: Into<AnyEvent>> From<Event<A>> for AnyEvent {
    fn from(value: Event<A>) -> Self {
        value.0.into()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Push {
    branches: Vec<String>,
}

impl Event<Push> {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        self.0.branches.push(branch.to_string());
        self
    }
}

impl From<Push> for AnyEvent {
    fn from(value: Push) -> Self {
        AnyEvent::default().push(value)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<String>>,
}

impl Event<PullRequest> {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        let mut branches = self.0.branches.unwrap_or_default();
        branches.push(branch.to_string());
        self.0.branches = Some(branches);
        self
    }

    fn add_type(mut self, ty: &str) -> Self {
        let mut types = self.0.types.unwrap_or_default();
        types.push(ty.to_string());
        self.0.types = Some(types);
        self
    }

    pub fn open(self) -> Self {
        self.add_type("opened")
    }

    pub fn synchronize(self) -> Self {
        self.add_type("synchronize")
    }

    pub fn reopen(self) -> Self {
        self.add_type("reopened")
    }
}

impl From<PullRequest> for AnyEvent {
    fn from(value: PullRequest) -> Self {
        AnyEvent::default().pull_request(value)
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<String>>,
}

impl Event<PullRequestTarget> {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        let mut branches = self.0.branches.unwrap_or_default();
        branches.push(branch.to_string());
        self.0.branches = Some(branches);
        self
    }

    fn add_type(mut self, ty: &str) -> Self {
        let mut types = self.0.types.unwrap_or_default();
        types.push(ty.to_string());
        self.0.types = Some(types);
        self
    }

    pub fn open(self) -> Self {
        self.add_type("opened")
    }

    pub fn synchronize(self) -> Self {
        self.add_type("synchronize")
    }

    pub fn reopen(self) -> Self {
        self.add_type("reopened")
    }
}

impl From<PullRequestTarget> for AnyEvent {
    fn from(value: PullRequestTarget) -> Self {
        AnyEvent::default().pull_request_target(value)
    }
}
