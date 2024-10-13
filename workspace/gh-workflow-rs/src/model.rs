use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    /// Concurrency ensures that only a single job or workflow using the same concurrency group
    /// will run at a time. A concurrency group can be any string or expression. The expression
    /// can use any context except for the secrets context.
    /// You can also specify concurrency at the workflow level.
    /// When a concurrent job or workflow is queued, if another job or workflow using the same
    /// concurrency group in the repository is in progress, the queued job or workflow will be
    /// pending. Any previously pending job or workflow in the concurrency group will be
    /// canceled. To also cancel any currently running job or workflow in the same concurrency
    /// group, specify cancel-in-progress: true.
    pub concurrency: Option<ConcurrencyUnion>,

    /// A map of default settings that will apply to all jobs in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,

    /// A map of environment variables that are available to all jobs and steps in the workflow.
    pub env: Option<Env>,

    /// A workflow run is made up of one or more jobs. Jobs run in parallel by default. To run
    /// jobs sequentially, you can define dependencies on other jobs using the
    /// jobs.<job_id>.needs keyword.
    /// Each job runs in a fresh instance of the virtual environment specified by runs-on.
    /// You can run an unlimited number of jobs as long as you are within the workflow usage
    /// limits. For more information, see
    /// https://help.github.com/en/github/automating-your-workflow-with-github-actions/workflow-syntax-for-github-actions#usage-limits.
    pub jobs: Jobs,

    /// The name of your workflow. GitHub displays the names of your workflows on your
    /// repository's actions page. If you omit this field, GitHub sets the name to the workflow's
    /// filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The name of the GitHub event that triggers the workflow. You can provide a single event
    /// string, array of events, array of event types, or an event configuration map that
    /// schedules a workflow or restricts the execution of a workflow to specific files, tags, or
    /// branch changes. For a list of available events, see
    /// https://help.github.com/en/github/automating-your-workflow-with-github-actions/events-that-trigger-workflows.
    pub on: OnUnion,

    pub permissions: Option<Permissions>,

    /// The name for workflow runs generated from the workflow. GitHub displays the workflow run
    /// name in the list of workflow runs on your repository's 'Actions' tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
}

/// Concurrency ensures that only a single job or workflow using the same concurrency group
/// will run at a time. A concurrency group can be any string or expression. The expression
/// can use any context except for the secrets context.
/// You can also specify concurrency at the workflow level.
/// When a concurrent job or workflow is queued, if another job or workflow using the same
/// concurrency group in the repository is in progress, the queued job or workflow will be
/// pending. Any previously pending job or workflow in the concurrency group will be
/// canceled. To also cancel any currently running job or workflow in the same concurrency
/// group, specify cancel-in-progress: true.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConcurrencyUnion {
    Concurrency(Concurrency),

    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Concurrency {
    /// To cancel any currently running job or workflow in the same concurrency group, specify
    /// cancel-in-progress: true.
    pub cancel_in_progress: Option<CancelInProgress>,

    /// When a concurrent job or workflow is queued, if another job or workflow using the same
    /// concurrency group in the repository is in progress, the queued job or workflow will be
    /// pending. Any previously pending job or workflow in the concurrency group will be canceled.
    pub group: String,
}

/// To cancel any currently running job or workflow in the same concurrency group, specify
/// cancel-in-progress: true.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelInProgress {
    Bool(bool),

    String(String),
}

/// A map of default settings that will apply to all jobs in the workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Run>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Run {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// A map of environment variables that are available to all jobs and steps in the workflow.
///
/// To set custom environment variables, you need to specify the variables in the workflow
/// file. You can define environment variables for a step, job, or entire workflow using the
/// jobs.<job_id>.steps[*].env, jobs.<job_id>.env, and env keywords. For more information,
/// see
/// https://docs.github.com/en/actions/learn-github-actions/workflow-syntax-for-github-actions#jobsjob_idstepsenv
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Env {
    String(String),

    UnionMap(HashMap<String, EnvValue>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvValue {
    Bool(bool),

    Double(f64),

    String(String),
}

/// A workflow run is made up of one or more jobs. Jobs run in parallel by default. To run
/// jobs sequentially, you can define dependencies on other jobs using the
/// jobs.<job_id>.needs keyword.
/// Each job runs in a fresh instance of the virtual environment specified by runs-on.
/// You can run an unlimited number of jobs as long as you are within the workflow usage
/// limits. For more information, see
/// https://help.github.com/en/github/automating-your-workflow-with-github-actions/workflow-syntax-for-github-actions#usage-limits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jobs {
}

/// The name of the GitHub event that triggers the workflow. You can provide a single event
/// string, array of events, array of event types, or an event configuration map that
/// schedules a workflow or restricts the execution of a workflow to specific files, tags, or
/// branch changes. For a list of available events, see
/// https://help.github.com/en/github/automating-your-workflow-with-github-actions/events-that-trigger-workflows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnUnion {
    Enum(Event),

    EnumArray(Vec<Event>),

    OnClass(OnClass),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    #[serde(rename = "branch_protection_rule")]
    BranchProtectionRule,

    #[serde(rename = "check_run")]
    CheckRun,

    #[serde(rename = "check_suite")]
    CheckSuite,

    Create,

    Delete,

    Deployment,

    #[serde(rename = "deployment_status")]
    DeploymentStatus,

    Discussion,

    #[serde(rename = "discussion_comment")]
    DiscussionComment,

    Fork,

    Gollum,

    #[serde(rename = "issue_comment")]
    IssueComment,

    Issues,

    Label,

    #[serde(rename = "merge_group")]
    MergeGroup,

    Milestone,

    #[serde(rename = "page_build")]
    PageBuild,

    Project,

    #[serde(rename = "project_card")]
    ProjectCard,

    #[serde(rename = "project_column")]
    ProjectColumn,

    Public,

    #[serde(rename = "pull_request")]
    PullRequest,

    #[serde(rename = "pull_request_review")]
    PullRequestReview,

    #[serde(rename = "pull_request_review_comment")]
    PullRequestReviewComment,

    #[serde(rename = "pull_request_target")]
    PullRequestTarget,

    Push,

    #[serde(rename = "registry_package")]
    RegistryPackage,

    Release,

    #[serde(rename = "repository_dispatch")]
    RepositoryDispatch,

    Status,

    Watch,

    #[serde(rename = "workflow_call")]
    WorkflowCall,

    #[serde(rename = "workflow_dispatch")]
    WorkflowDispatch,

    #[serde(rename = "workflow_run")]
    WorkflowRun,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnClass {
    /// Runs your workflow anytime the branch_protection_rule event occurs. More than one
    /// activity type triggers this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_protection_rule: Option<PurpleEventObject>,

    /// Runs your workflow anytime the check_run event occurs. More than one activity type
    /// triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/checks/runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_run: Option<FluffyEventObject>,

    /// Runs your workflow anytime the check_suite event occurs. More than one activity type
    /// triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/checks/suites/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_suite: Option<TentacledEventObject>,

    /// Runs your workflow anytime someone creates a branch or tag, which triggers the create
    /// event. For information about the REST API, see
    /// https://developer.github.com/v3/git/refs/#create-a-reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime someone deletes a branch or tag, which triggers the delete
    /// event. For information about the REST API, see
    /// https://developer.github.com/v3/git/refs/#delete-a-reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime someone creates a deployment, which triggers the deployment
    /// event. Deployments created with a commit SHA may not have a Git ref. For information
    /// about the REST API, see https://developer.github.com/v3/repos/deployments/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime a third party provides a deployment status, which triggers the
    /// deployment_status event. Deployments created with a commit SHA may not have a Git ref.
    /// For information about the REST API, see
    /// https://developer.github.com/v3/repos/deployments/#create-a-deployment-status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime the discussion event occurs. More than one activity type
    /// triggers this event. For information about the GraphQL API, see
    /// https://docs.github.com/en/graphql/guides/using-the-graphql-api-for-discussions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion: Option<StickyEventObject>,

    /// Runs your workflow anytime the discussion_comment event occurs. More than one activity
    /// type triggers this event. For information about the GraphQL API, see
    /// https://docs.github.com/en/graphql/guides/using-the-graphql-api-for-discussions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_comment: Option<IndigoEventObject>,

    /// Runs your workflow anytime when someone forks a repository, which triggers the fork
    /// event. For information about the REST API, see
    /// https://developer.github.com/v3/repos/forks/#create-a-fork.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow when someone creates or updates a Wiki page, which triggers the gollum
    /// event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gollum: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime the issue_comment event occurs. More than one activity type
    /// triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/issues/comments/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_comment: Option<IndecentEventObject>,

    /// Runs your workflow anytime the issues event occurs. More than one activity type triggers
    /// this event. For information about the REST API, see
    /// https://developer.github.com/v3/issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<HilariousEventObject>,

    /// Runs your workflow anytime the label event occurs. More than one activity type triggers
    /// this event. For information about the REST API, see
    /// https://developer.github.com/v3/issues/labels/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<AmbitiousEventObject>,

    /// Runs your workflow when a pull request is added to a merge queue, which adds the pull
    /// request to a merge group. For information about the merge queue, see
    /// https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/incorporating-changes-from-a-pull-request/merging-a-pull-request-with-a-merge-queue
    /// .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_group: Option<CunningEventObject>,

    /// Runs your workflow anytime the milestone event occurs. More than one activity type
    /// triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/issues/milestones/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<MagentaEventObject>,

    /// Runs your workflow anytime someone pushes to a GitHub Pages-enabled branch, which
    /// triggers the page_build event. For information about the REST API, see
    /// https://developer.github.com/v3/repos/pages/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_build: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime the project event occurs. More than one activity type triggers
    /// this event. For information about the REST API, see
    /// https://developer.github.com/v3/projects/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<FriskyEventObject>,

    /// Runs your workflow anytime the project_card event occurs. More than one activity type
    /// triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/projects/cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_card: Option<MischievousEventObject>,

    /// Runs your workflow anytime the project_column event occurs. More than one activity type
    /// triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/projects/columns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_column: Option<BraggadociousEventObject>,

    /// Runs your workflow anytime someone makes a private repository public, which triggers the
    /// public event. For information about the REST API, see
    /// https://developer.github.com/v3/repos/#edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime the pull_request event occurs. More than one activity type
    /// triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/pulls.
    /// Note: Workflows do not run on private base repositories when you open a pull request from
    /// a forked repository.
    /// When you create a pull request from a forked repository to the base repository, GitHub
    /// sends the pull_request event to the base repository and no pull request events occur on
    /// the forked repository.
    /// Workflows don't run on forked repositories by default. You must enable GitHub Actions in
    /// the Actions tab of the forked repository.
    /// The permissions for the GITHUB_TOKEN in forked repositories is read-only. For more
    /// information about the GITHUB_TOKEN, see
    /// https://help.github.com/en/articles/virtual-environments-for-github-actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PurpleRef>,

    /// Runs your workflow anytime the pull_request_review event occurs. More than one activity
    /// type triggers this event. For information about the REST API, see
    /// https://developer.github.com/v3/pulls/reviews.
    /// Note: Workflows do not run on private base repositories when you open a pull request from
    /// a forked repository.
    /// When you create a pull request from a forked repository to the base repository, GitHub
    /// sends the pull_request event to the base repository and no pull request events occur on
    /// the forked repository.
    /// Workflows don't run on forked repositories by default. You must enable GitHub Actions in
    /// the Actions tab of the forked repository.
    /// The permissions for the GITHUB_TOKEN in forked repositories is read-only. For more
    /// information about the GITHUB_TOKEN, see
    /// https://help.github.com/en/articles/virtual-environments-for-github-actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_review: Option<EventObject1>,

    /// Runs your workflow anytime a comment on a pull request's unified diff is modified, which
    /// triggers the pull_request_review_comment event. More than one activity type triggers this
    /// event. For information about the REST API, see
    /// https://developer.github.com/v3/pulls/comments.
    /// Note: Workflows do not run on private base repositories when you open a pull request from
    /// a forked repository.
    /// When you create a pull request from a forked repository to the base repository, GitHub
    /// sends the pull_request event to the base repository and no pull request events occur on
    /// the forked repository.
    /// Workflows don't run on forked repositories by default. You must enable GitHub Actions in
    /// the Actions tab of the forked repository.
    /// The permissions for the GITHUB_TOKEN in forked repositories is read-only. For more
    /// information about the GITHUB_TOKEN, see
    /// https://help.github.com/en/articles/virtual-environments-for-github-actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_review_comment: Option<EventObject2>,

    /// This event is similar to pull_request, except that it runs in the context of the base
    /// repository of the pull request, rather than in the merge commit. This means that you can
    /// more safely make your secrets available to the workflows triggered by the pull request,
    /// because only workflows defined in the commit on the base repository are run. For example,
    /// this event allows you to create workflows that label and comment on pull requests, based
    /// on the contents of the event payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_target: Option<FluffyRef>,

    /// Runs your workflow when someone pushes to a repository branch, which triggers the push
    /// event.
    /// Note: The webhook payload available to GitHub Actions does not include the added,
    /// removed, and modified attributes in the commit object. You can retrieve the full commit
    /// object using the REST API. For more information, see
    /// https://developer.github.com/v3/repos/commits/#get-a-single-commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<TentacledRef>,

    /// Runs your workflow anytime a package is published or updated. For more information, see
    /// https://help.github.com/en/github/managing-packages-with-github-packages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_package: Option<EventObject3>,

    /// Runs your workflow anytime the release event occurs. More than one activity type triggers
    /// this event. For information about the REST API, see
    /// https://developer.github.com/v3/repos/releases/ in the GitHub Developer documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<EventObject4>,

    /// You can use the GitHub API to trigger a webhook event called repository_dispatch when you
    /// want to trigger a workflow for activity that happens outside of GitHub. For more
    /// information, see
    /// https://developer.github.com/v3/repos/#create-a-repository-dispatch-event.
    /// To trigger the custom repository_dispatch webhook event, you must send a POST request to
    /// a GitHub API endpoint and provide an event_type name to describe the activity type. To
    /// trigger a workflow run, you must also configure your workflow to use the
    /// repository_dispatch event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_dispatch: Option<HashMap<String, Option<serde_json::Value>>>,

    /// You can schedule a workflow to run at specific UTC times using POSIX cron syntax
    /// (https://pubs.opengroup.org/onlinepubs/9699919799/utilities/crontab.html#tag_20_25_07).
    /// Scheduled workflows run on the latest commit on the default or base branch. The shortest
    /// interval you can run scheduled workflows is once every 5 minutes.
    /// Note: GitHub Actions does not support the non-standard syntax @yearly, @monthly, @weekly,
    /// @daily, @hourly, and @reboot.
    /// You can use crontab guru (https://crontab.guru/). to help generate your cron syntax and
    /// confirm what time it will run. To help you get started, there is also a list of crontab
    /// guru examples (https://crontab.guru/examples.html).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Vec<Option<ScheduleElement>>>,

    /// Runs your workflow anytime the status of a Git commit changes, which triggers the status
    /// event. For information about the REST API, see
    /// https://developer.github.com/v3/repos/statuses/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Runs your workflow anytime the watch event occurs. More than one activity type triggers
    /// this event. For information about the REST API, see
    /// https://developer.github.com/v3/activity/starring/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Allows workflows to be reused by other workflows.
    pub workflow_call: Option<WorkflowCallUnion>,

    /// You can now create workflows that are manually triggered with the new workflow_dispatch
    /// event. You will then see a 'Run workflow' button on the Actions tab, enabling you to
    /// easily trigger a run.
    pub workflow_dispatch: Option<WorkflowDispatchUnion>,

    /// This event occurs when a workflow run is requested or completed, and allows you to
    /// execute a workflow based on the finished result of another workflow. For example, if your
    /// pull_request workflow generates build artifacts, you can create a new workflow that uses
    /// workflow_run to analyze the results and add a comment to the original pull request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run: Option<EventObject5>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagentaEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriskyEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischievousEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BraggadociousEventObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventObject1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventObject2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledRef {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventObject3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventObject4 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScheduleElement {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    ScheduleClass(ScheduleClass),

    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
}

/// Allows workflows to be reused by other workflows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowCallUnion {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    String(String),

    WorkflowCallClass(WorkflowCallClass),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCallClass {
    /// When using the workflow_call keyword, you can optionally specify inputs that are passed
    /// to the called workflow from the caller workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<WorkflowCallInputs>,

    /// A map of the secrets that can be used in the called workflow. Within the called workflow,
    /// you can use the secrets context to refer to a secret.
    pub secrets: Option<SecretsUnion>,
}

/// When using the workflow_call keyword, you can optionally specify inputs that are passed
/// to the called workflow from the caller workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCallInputs {
}

/// A map of the secrets that can be used in the called workflow. Within the called workflow,
/// you can use the secrets context to refer to a secret.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretsUnion {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    SecretsClass(SecretsClass),

    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretsClass {
}

/// You can now create workflows that are manually triggered with the new workflow_dispatch
/// event. You will then see a 'Run workflow' button on the Actions tab, enabling you to
/// easily trigger a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowDispatchUnion {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    String(String),

    WorkflowDispatchClass(WorkflowDispatchClass),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDispatchClass {
    /// Input parameters allow you to specify data that the action expects to use during runtime.
    /// GitHub stores input parameters as environment variables. Input ids with uppercase letters
    /// are converted to lowercase during runtime. We recommended using lowercase input ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<WorkflowDispatchInputs>,
}

/// Input parameters allow you to specify data that the action expects to use during runtime.
/// GitHub stores input parameters as environment variables. Input ids with uppercase letters
/// are converted to lowercase during runtime. We recommended using lowercase input ids.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDispatchInputs {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventObject5 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Option<serde_json::Value>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<String>>,
}

/// You can modify the default permissions granted to the GITHUB_TOKEN, adding or removing
/// access as required, so that you only allow the minimum required access.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Permissions {
    Enum(PermissionsEnum),

    PermissionsEvent(PermissionsEvent),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PermissionsEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestations: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussions: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_events: Option<PermissionsLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<PermissionsLevel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionsLevel {
    None,

    Read,

    Write,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionsEnum {
    #[serde(rename = "read-all")]
    ReadAll,

    #[serde(rename = "write-all")]
    WriteAll,
}
