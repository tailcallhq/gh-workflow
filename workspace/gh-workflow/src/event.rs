use std::fmt::{Display, Formatter};
use derive_setters::Setters;
use indexmap::IndexMap;
use serde::Serialize;
use crate::is_default;
use crate::SetEvent;

#[derive(Serialize, Setters, Clone)]
#[serde(rename_all = "snake_case")]
#[setters(strip_option)]
pub struct Event {
    #[serde(skip_serializing_if = "is_default")]
    pub push: Option<Push>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request: Option<PullRequest>,
    #[serde(skip_serializing_if = "is_default")]
    pub branch_protection_rule: Option<BranchProtectionRule>,
    #[serde(skip_serializing_if = "is_default")]
    pub check_run: Option<CheckRun>,
    #[serde(skip_serializing_if = "is_default")]
    pub check_suite: Option<CheckSuite>,
    #[serde(skip_serializing_if = "is_default")]
    pub create: Option<Create>,
    #[serde(skip_serializing_if = "is_default")]
    pub delete: Option<Delete>,
    #[serde(skip_serializing_if = "is_default")]
    pub deployment: Option<Deployment>,
    #[serde(skip_serializing_if = "is_default")]
    pub deployment_status: Option<DeploymentStatus>,
    #[serde(skip_serializing_if = "is_default")]
    pub discussion: Option<Discussion>,
    #[serde(skip_serializing_if = "is_default")]
    pub discussion_comment: Option<DiscussionComment>,
    #[serde(skip_serializing_if = "is_default")]
    pub fork: Option<Fork>,
    #[serde(skip_serializing_if = "is_default")]
    pub gollum: Option<Gollum>,
    #[serde(skip_serializing_if = "is_default")]
    pub issue_comment: Option<IssueComment>,
    #[serde(skip_serializing_if = "is_default")]
    pub issues: Option<Issues>,
    #[serde(skip_serializing_if = "is_default")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "is_default")]
    pub merge_group: Option<MergeGroup>,
    #[serde(skip_serializing_if = "is_default")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "is_default")]
    pub page_build: Option<PageBuild>,
    #[serde(skip_serializing_if = "is_default")]
    pub project: Option<Project>,
    #[serde(skip_serializing_if = "is_default")]
    pub project_card: Option<ProjectCard>,
    #[serde(skip_serializing_if = "is_default")]
    pub project_column: Option<ProjectColumn>,
    #[serde(skip_serializing_if = "is_default")]
    pub public: Option<Public>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request_review: Option<PullRequestReview>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request_review_comment: Option<PullRequestReviewComment>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request_target: Option<PullRequestTarget>,
    #[serde(skip_serializing_if = "is_default")]
    pub registry_package: Option<RegistryPackage>,
    #[serde(skip_serializing_if = "is_default")]
    pub release: Option<Release>,
    #[serde(skip_serializing_if = "is_default")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "is_default")]
    pub watch: Option<Watch>,
    #[serde(skip_serializing_if = "is_default")]
    pub workflow_call: Option<WorkflowCall>,
    #[serde(skip_serializing_if = "is_default")]
    pub workflow_dispatch: Option<WorkflowDispatch>,
    #[serde(skip_serializing_if = "is_default")]
    pub workflow_run: Option<WorkflowRun>,
    #[serde(skip_serializing_if = "is_default")]
    pub repository_dispatch: Option<RepositoryDispatch>,
}

// TODO: rename event
#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BranchEvent {
    Created,
    #[default]
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunEvent {
    #[default]
    Created,
    Rerequested,
    Completed,
    RequestedAction,
}

impl Display for CheckRunEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            CheckRunEvent::Created => "created",
            CheckRunEvent::Rerequested => "rerequested",
            CheckRunEvent::Completed => "completed",
            CheckRunEvent::RequestedAction => "requested_action",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteEvent {
    #[default]
    Completed,
    Requested,
    Rerequested,
}

impl Display for CheckSuiteEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            CheckSuiteEvent::Completed => "completed",
            CheckSuiteEvent::Requested => "requested",
            CheckSuiteEvent::Rerequested => "rerequested",
        };
        write!(f, "{}", val)
    }
}

impl Display for BranchEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            BranchEvent::Created => "created",
            BranchEvent::Edited => "edited",
            BranchEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestEvent {
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    #[default]
    Opened,
    Edited,
    Closed,
    Reopened,
    Synchronize,
    ConvertedToDraft,
    ReadyForReview,
    Locked,
    Unlocked,
    Milestoned,
    Demilestoned,
    ReviewRequested,
    ReviewRequestRemoved,
    AutoMergeEnabled,
    AutoMergeDisabled,
    Enqueued,
    Dequeued,
}

impl Display for PullRequestEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            PullRequestEvent::Assigned => "assigned",
            PullRequestEvent::Unassigned => "unassigned",
            PullRequestEvent::Labeled => "labeled",
            PullRequestEvent::Unlabeled => "unlabeled",
            PullRequestEvent::Opened => "opened",
            PullRequestEvent::Edited => "edited",
            PullRequestEvent::Closed => "closed",
            PullRequestEvent::Reopened => "reopened",
            PullRequestEvent::Synchronize => "synchronize",
            PullRequestEvent::ConvertedToDraft => "converted_to_draft",
            PullRequestEvent::ReadyForReview => "ready_for_review",
            PullRequestEvent::Locked => "locked",
            PullRequestEvent::Unlocked => "unlocked",
            PullRequestEvent::Milestoned => "milestoned",
            PullRequestEvent::Demilestoned => "demilestoned",
            PullRequestEvent::ReviewRequested => "review_requested",
            PullRequestEvent::ReviewRequestRemoved => "review_request_removed",
            PullRequestEvent::AutoMergeEnabled => "auto_merge_enabled",
            PullRequestEvent::AutoMergeDisabled => "auto_merge_disabled",
            PullRequestEvent::Enqueued => "enqueued",
            PullRequestEvent::Dequeued => "dequeued",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionEvent {
    #[default]
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

impl Display for DiscussionEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            DiscussionEvent::Created => "created",
            DiscussionEvent::Edited => "edited",
            DiscussionEvent::Deleted => "deleted",
            DiscussionEvent::Transferred => "transferred",
            DiscussionEvent::Pinned => "pinned",
            DiscussionEvent::Unpinned => "unpinned",
            DiscussionEvent::Labeled => "labeled",
            DiscussionEvent::Unlabeled => "unlabeled",
            DiscussionEvent::Locked => "locked",
            DiscussionEvent::Unlocked => "unlocked",
            DiscussionEvent::CategoryChanged => "category_changed",
            DiscussionEvent::Answered => "answered",
            DiscussionEvent::Unanswered => "unanswered",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionCommentEvent {
    #[default]
    Created,
    Edited,
    Deleted,
}

impl Display for DiscussionCommentEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            DiscussionCommentEvent::Created => "created",
            DiscussionCommentEvent::Edited => "edited",
            DiscussionCommentEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentEvent {
    #[default]
    Created,
    Edited,
    Deleted,
}

impl Display for IssueCommentEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            IssueCommentEvent::Created => "created",
            IssueCommentEvent::Edited => "edited",
            IssueCommentEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuesEvent {
    #[default]
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

impl Display for IssuesEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            IssuesEvent::Opened => "opened",
            IssuesEvent::Edited => "edited",
            IssuesEvent::Deleted => "deleted",
            IssuesEvent::Transferred => "transferred",
            IssuesEvent::Pinned => "pinned",
            IssuesEvent::Unpinned => "unpinned",
            IssuesEvent::Closed => "closed",
            IssuesEvent::Reopened => "reopened",
            IssuesEvent::Assigned => "assigned",
            IssuesEvent::Unassigned => "unassigned",
            IssuesEvent::Labeled => "labeled",
            IssuesEvent::Unlabeled => "unlabeled",
            IssuesEvent::Locked => "locked",
            IssuesEvent::Unlocked => "unlocked",
            IssuesEvent::Milestoned => "milestoned",
            IssuesEvent::Demilestoned => "demilestoned",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LabelEvent {
    #[default]
    Created,
    Edited,
    Deleted,
}

impl Display for LabelEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            LabelEvent::Created => "created",
            LabelEvent::Edited => "edited",
            LabelEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MergeGroupEvent {
    #[default]
    ChecksRequested,
}

impl Display for MergeGroupEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            MergeGroupEvent::ChecksRequested => "checks_requested",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneEvent {
    #[default]
    Created,
    Closed,
    Opened,
    Edited,
    Deleted,
}

impl Display for MilestoneEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            MilestoneEvent::Created => "created",
            MilestoneEvent::Closed => "closed",
            MilestoneEvent::Opened => "opened",
            MilestoneEvent::Edited => "edited",
            MilestoneEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectEvent {
    #[default]
    Created,
    Updated,
    Closed,
    Reopened,
    Edited,
    Deleted,
}

impl Display for ProjectEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            ProjectEvent::Created => "created",
            ProjectEvent::Updated => "updated",
            ProjectEvent::Closed => "closed",
            ProjectEvent::Reopened => "reopened",
            ProjectEvent::Edited => "edited",
            ProjectEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCardEvent {
    #[default]
    Created,
    Moved,
    Converted,
    Edited,
    Deleted,
}

impl Display for ProjectCardEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            ProjectCardEvent::Created => "created",
            ProjectCardEvent::Moved => "moved",
            ProjectCardEvent::Converted => "converted",
            ProjectCardEvent::Edited => "edited",
            ProjectCardEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectColumnEvent {
    #[default]
    Created,
    Updated,
    Moved,
    Deleted,
}

impl Display for ProjectColumnEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            ProjectColumnEvent::Created => "created",
            ProjectColumnEvent::Updated => "updated",
            ProjectColumnEvent::Moved => "moved",
            ProjectColumnEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewEvent {
    #[default]
    Submitted,
    Edited,
    Dismissed,
}

impl Display for PullRequestReviewEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            PullRequestReviewEvent::Submitted => "submitted",
            PullRequestReviewEvent::Edited => "edited",
            PullRequestReviewEvent::Dismissed => "dismissed",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewCommentEvent {
    #[default]
    Created,
    Edited,
    Deleted,
}

impl Display for PullRequestReviewCommentEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            PullRequestReviewCommentEvent::Created => "created",
            PullRequestReviewCommentEvent::Edited => "edited",
            PullRequestReviewCommentEvent::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestTargetEvent {
    #[default]
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Opened,
    Edited,
    Closed,
    Reopened,
    Synchronize,
    ConvertedToDraft,
    ReadyForReview,
    Locked,
    Unlocked,
    Milestoned,
    Demilestoned,
    ReviewRequested,
    ReviewRequestRemoved,
    AutoMergeEnabled,
    AutoMergeDisabled,
}

impl Display for PullRequestTargetEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            PullRequestTargetEvent::Assigned => "assigned",
            PullRequestTargetEvent::Unassigned => "unassigned",
            PullRequestTargetEvent::Labeled => "labeled",
            PullRequestTargetEvent::Unlabeled => "unlabeled",
            PullRequestTargetEvent::Opened => "opened",
            PullRequestTargetEvent::Edited => "edited",
            PullRequestTargetEvent::Closed => "closed",
            PullRequestTargetEvent::Reopened => "reopened",
            PullRequestTargetEvent::Synchronize => "synchronize",
            PullRequestTargetEvent::ConvertedToDraft => "converted_to_draft",
            PullRequestTargetEvent::ReadyForReview => "ready_for_review",
            PullRequestTargetEvent::Locked => "locked",
            PullRequestTargetEvent::Unlocked => "unlocked",
            PullRequestTargetEvent::Milestoned => "milestoned",
            PullRequestTargetEvent::Demilestoned => "demilestoned",
            PullRequestTargetEvent::ReviewRequested => "review_requested",
            PullRequestTargetEvent::ReviewRequestRemoved => "review_request_removed",
            PullRequestTargetEvent::AutoMergeEnabled => "auto_merge_enabled",
            PullRequestTargetEvent::AutoMergeDisabled => "auto_merge_disabled",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RegistryPackageEvent {
    #[default]
    Published,
    Updated,
}

impl Display for RegistryPackageEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            RegistryPackageEvent::Published => "published",
            RegistryPackageEvent::Updated => "updated",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseEvent {
    #[default]
    Published,
    Unpublished,
    Created,
    Edited,
    Deleted,
    Prereleased,
    Released,
}

impl Display for ReleaseEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            ReleaseEvent::Published => "published",
            ReleaseEvent::Unpublished => "unpublished",
            ReleaseEvent::Created => "created",
            ReleaseEvent::Edited => "edited",
            ReleaseEvent::Deleted => "deleted",
            ReleaseEvent::Prereleased => "prereleased",
            ReleaseEvent::Released => "released",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunEvent {
    #[default]
    Requested,
    Completed,
    InProgress,
}

impl Display for WorkflowRunEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            WorkflowRunEvent::Requested => "requested",
            WorkflowRunEvent::Completed => "completed",
            WorkflowRunEvent::InProgress => "in_progress",
        };
        write!(f, "{}", val)
    }
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Push {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Public {
    // TODO: needs review
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequest {
    types: Vec<PullRequestEvent>,
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct BranchProtectionRule {
    types: Vec<BranchEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CheckRun {
    types: Vec<CheckRunEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CheckSuite {
    types: Vec<CheckSuiteEvent>,
}


#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Create {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Delete {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Deployment {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentStatus {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Discussion {
    types: Vec<DiscussionEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DiscussionComment {
    types: Vec<DiscussionCommentEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Fork {
    // TODO: needs check
    types: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Gollum {
    // TODO: needs check
    types: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct IssueComment {
    types: Vec<IssueCommentEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Issues {
    types: Vec<IssuesEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Label {
    types: Vec<LabelEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct MergeGroup {
    types: Vec<MergeGroupEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Milestone {
    types: Vec<MilestoneEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PageBuild {
    // TODO: needs review
    types: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    types: Vec<ProjectEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectCard {
    types: Vec<ProjectCardEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectColumn {
    types: Vec<ProjectColumnEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestReview {
    types: Vec<PullRequestReviewEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestReviewComment {
    types: Vec<PullRequestReviewCommentEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestTarget {
    types: Vec<PullRequestTargetEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RegistryPackage {
    types: Vec<RegistryPackageEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    types: Vec<ReleaseEvent>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Status {
    // TODO: needs review
    types: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Watch {
    // TODO: needs review
    types: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowCall {
    pub inputs: IndexMap<String, WorkflowInput>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowInput {
    // TODO: needs review
    pub description: Option<String>,
    pub deprecation_message: Option<String>,
    pub required: Option<bool>,
    pub input_type: String, // "boolean", "number", or "string"
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowDispatch {
    // TODO: needs review (this is probably incorrect)
    pub inputs: IndexMap<String, WorkflowInput>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowRun {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    types: Vec<WorkflowRunEvent>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    workflows: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RepositoryDispatch {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    types: Vec<String>,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            push: Some(Push::default()),
            pull_request: None,
            branch_protection_rule: None,
            check_run: None,
            check_suite: None,
            create: None,
            delete: None,
            deployment: None,
            deployment_status: None,
            discussion: None,
            discussion_comment: None,
            fork: None,
            gollum: None,
            issue_comment: None,
            issues: None,
            label: None,
            merge_group: None,
            milestone: None,
            page_build: None,
            project: None,
            project_card: None,
            project_column: None,
            public: None,
            pull_request_review: None,
            pull_request_review_comment: None,
            pull_request_target: None,
            registry_package: None,
            release: None,
            status: None,
            watch: None,
            workflow_call: None,
            workflow_dispatch: None,
            workflow_run: None,
            repository_dispatch: None,
        }
    }
}

impl SetEvent for Event {
    fn apply(self, mut workflow: crate::Workflow) -> crate::Workflow {
        workflow.on = serde_json::to_value(self).ok();
        workflow
    }
}

impl Push {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        self.branches.push(branch.to_string());
        self
    }
}

impl PullRequest {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        self.branches.push(branch.to_string());
        self
    }

    pub fn open(mut self) -> Self {
        self.types.push(PullRequestEvent::Opened);
        self
    }

    pub fn synchronize(mut self) -> Self {
        self.types.push(PullRequestEvent::Synchronize);
        self
    }

    pub fn reopen(mut self) -> Self {
        self.types.push(PullRequestEvent::Reopened);
        self
    }
}
