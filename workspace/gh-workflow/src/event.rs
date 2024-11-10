use derive_setters::Setters;
use merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(Default, Setters, Debug, Serialize, Deserialize, Clone, Merge, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[setters(strip_option, into)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_target: Option<PullRequestTarget>,
    // TODO: add all more events
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Push {
    branches: Vec<String>,
}

impl Push {
    pub fn add_branch<S: ToString>(mut self, branch: S) -> Self {
        self.branches.push(branch.to_string());
        self
    }
}

impl From<Push> for Event {
    fn from(value: Push) -> Self {
        Event::default().push(value)
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

impl PullRequest {
    pub fn add_branch<S: ToString>(mut self, branch: S) -> Self {
        let mut branches = self.branches.unwrap_or_default();
        branches.push(branch.to_string());
        self.branches = Some(branches);
        self
    }

    fn add_type(mut self, ty: &str) -> Self {
        let mut types = self.types.unwrap_or_default();
        types.push(ty.to_string());
        self.types = Some(types);
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

impl From<PullRequest> for Event {
    fn from(value: PullRequest) -> Self {
        Event::default().pull_request(value)
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

impl PullRequestTarget {
    pub fn add_branch<S: ToString>(mut self, branch: S) -> Self {
        let mut branches = self.branches.unwrap_or_default();
        branches.push(branch.to_string());
        self.branches = Some(branches);
        self
    }

    fn add_type(mut self, ty: &str) -> Self {
        let mut types = self.types.unwrap_or_default();
        types.push(ty.to_string());
        self.types = Some(types);
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

impl From<PullRequestTarget> for Event {
    fn from(value: PullRequestTarget) -> Self {
        Event::default().pull_request_target(value)
    }
}
