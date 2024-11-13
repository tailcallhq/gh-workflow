use std::collections::HashMap;

use derive_setters::Setters;
use merge::Merge;
use serde::{Deserialize, Serialize};

use crate::is_default;

#[derive(Default, Debug, Clone, Deserialize, Serialize, Merge, Setters)]
#[setters(strip_option, into)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_protection_rule: Option<BranchProtectionRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_run: Option<CheckRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_suite: Option<CheckSuite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<Create>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Delete>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<DeploymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion: Option<Discussion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_comment: Option<DiscussionComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gollum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_comment: Option<IssueComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Issues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_group: Option<MergeGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_review: Option<PullRequestReview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_review_comment: Option<PullRequestReviewComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_target: Option<PullRequestTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_package: Option<RegistryPackage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_dispatch: Option<RepositoryDispatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<Watch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_call: Option<WorkflowCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_dispatch: Option<WorkflowDispatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run: Option<WorkflowRun>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionRuleType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct BranchProtectionRule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<BranchProtectionRuleType>,
}

impl BranchProtectionRule {
    pub fn add_type(mut self, type_: BranchProtectionRuleType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunType {
    Created,
    Rerequested,
    Completed,
    RequestedAction,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct CheckRun {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<CheckRunType>,
}

impl CheckRun {
    pub fn add_type(mut self, type_: CheckRunType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteType {
    Completed,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct CheckSuite {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<CheckSuiteType>,
}

impl CheckSuite {
    pub fn add_type(mut self, type_: CheckSuiteType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Create {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Create {
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    pub fn add_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Delete {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Delete {
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    pub fn add_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Deployment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl Deployment {
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct DeploymentStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<String>,
}

impl DeploymentStatus {
    pub fn add_state<S: Into<String>>(mut self, state: S) -> Self {
        self.states.push(state.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionType {
    Created,
    Edited,
    Deleted,
    Transferred,
    Pinned,
    Unpinned,
    Labeled,
    Unlabeled,
    Locked,
    Unlocked,
    CategoryChanged,
    Answered,
    Unanswered,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Discussion {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<DiscussionType>,
}

impl Discussion {
    pub fn add_type(mut self, type_: DiscussionType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionCommentType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct DiscussionComment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<DiscussionCommentType>,
}

impl DiscussionComment {
    pub fn add_type(mut self, type_: DiscussionCommentType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct IssueComment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<IssueCommentType>,
}

impl IssueComment {
    pub fn add_type(mut self, type_: IssueCommentType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuesType {
    Opened,
    Edited,
    Deleted,
    Transferred,
    Pinned,
    Unpinned,
    Closed,
    Reopened,
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Locked,
    Unlocked,
    Milestoned,
    Demilestoned,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Issues {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<IssuesType>,
}

impl Issues {
    pub fn add_type(mut self, type_: IssuesType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Label {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<LabelType>,
}

impl Label {
    pub fn add_type(mut self, type_: LabelType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeGroupType {
    ChecksRequested,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct MergeGroup {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<MergeGroupType>,
}

impl MergeGroup {
    pub fn add_type(mut self, type_: MergeGroupType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneType {
    Created,
    Closed,
    Opened,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Milestone {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<MilestoneType>,
}

impl Milestone {
    pub fn add_type(mut self, type_: MilestoneType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestType {
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Opened,
    Edited,
    Closed,
    Reopened,
    Synchronize,
    ReadyForReview,
    Locked,
    Unlocked,
    ReviewRequested,
    ReviewRequestRemoved,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct PullRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}

impl PullRequest {
    pub fn add_type(mut self, type_: PullRequestType) -> Self {
        self.types.push(type_);
        self
    }

    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    pub fn add_path<S: Into<String>>(mut self, path: S) -> Self {
        self.paths.push(path.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewType {
    Submitted,
    Edited,
    Dismissed,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct PullRequestReview {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestReviewType>,
}

impl PullRequestReview {
    pub fn add_type(mut self, type_: PullRequestReviewType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewCommentType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct PullRequestReviewComment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestReviewCommentType>,
}

impl PullRequestReviewComment {
    pub fn add_type(mut self, type_: PullRequestReviewCommentType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct PullRequestTarget {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl PullRequestTarget {
    pub fn add_type(mut self, type_: PullRequestType) -> Self {
        self.types.push(type_);
        self
    }

    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Push {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}

impl Push {
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    pub fn add_path<S: Into<String>>(mut self, path: S) -> Self {
        self.paths.push(path.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryPackageType {
    Published,
    Updated,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct RegistryPackage {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<RegistryPackageType>,
}

impl RegistryPackage {
    pub fn add_type(mut self, type_: RegistryPackageType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseType {
    Published,
    Unpublished,
    Created,
    Edited,
    Deleted,
    Prereleased,
    Released,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Release {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<ReleaseType>,
}

impl Release {
    pub fn add_type(mut self, type_: ReleaseType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct RepositoryDispatch {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
}

impl RepositoryDispatch {
    pub fn add_type<S: Into<String>>(mut self, type_: S) -> Self {
        self.types.push(type_.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Schedule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cron: Vec<String>,
}

impl Schedule {
    pub fn add_cron<S: Into<String>>(mut self, cron: S) -> Self {
        self.cron.push(cron.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct Watch {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
}

impl Watch {
    pub fn add_type<S: Into<String>>(mut self, type_: S) -> Self {
        self.types.push(type_.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowCall {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub inputs: HashMap<String, WorkflowCallInput>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub outputs: HashMap<String, WorkflowCallOutput>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub secrets: HashMap<String, WorkflowCallSecret>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowCallInput {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "is_default")]
    pub required: bool,
    #[serde(rename = "type")]
    pub input_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowCallOutput {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowCallSecret {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "is_default")]
    pub required: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowDispatch {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub inputs: HashMap<String, WorkflowDispatchInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowDispatchInput {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "is_default")]
    pub required: bool,
    #[serde(rename = "type")]
    pub input_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunType {
    Completed,
    Requested,
    InProgress,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowRun {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<WorkflowRunType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflows: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl WorkflowRun {
    pub fn add_type(mut self, type_: WorkflowRunType) -> Self {
        self.types.push(type_);
        self
    }

    pub fn add_workflow<S: Into<String>>(mut self, workflow: S) -> Self {
        self.workflows.push(workflow.into());
        self
    }

    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }
}
