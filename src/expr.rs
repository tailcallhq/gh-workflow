use std::fmt::Display;
use std::marker::PhantomData;
use std::rc::Rc;

use gh_workflow_macros::Expr;

pub struct Expr<A> {
    marker: PhantomData<A>,
    step: Step,
}

#[derive(Default, Clone)]
enum Step {
    #[default]
    Root,
    Select {
        name: Rc<String>,
        object: Box<Step>,
    },
}

impl Step {
    fn select(name: impl Into<String>) -> Step {
        Step::Select { name: Rc::new(name.into()), object: Box::new(Step::Root) }
    }
}

impl<A> Expr<A> {
    fn new() -> Self {
        Expr { marker: PhantomData, step: Step::Root }
    }

    fn select<B>(&self, path: impl Into<String>) -> Expr<B> {
        Expr {
            marker: PhantomData,
            step: Step::Select {
                name: Rc::new(path.into()),
                object: Box::new(self.step.clone()),
            },
        }
    }
}

impl<A> Display for Expr<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack: Vec<Step> = vec![self.step.clone()];

        write!(f, "{{{{ ")?;

        loop {
            match stack.pop() {
                None => break,
                Some(step) => match step {
                    Step::Root => break,
                    Step::Select { name, object } => {
                        if matches!(*object, Step::Root) {
                            write!(f, "{}", name.replace('"', ""))?;
                        } else {
                            stack.push(Step::select(name.as_str()));
                            // TODO: this is a hack to insert a `.` between the two steps
                            stack.push(Step::select("."));
                            stack.push(*object);
                        }
                    }
                },
            }
        }

        write!(f, " }}}}")
    }
}

#[derive(Expr)]
pub struct Github {
    /// The name of the action currently running, or the id of a step.
    action: String,
    /// The path where an action is located. This property is only supported in
    /// composite actions.
    action_path: String,
    /// For a step executing an action, this is the ref of the action being
    /// executed.
    action_ref: String,
    /// For a step executing an action, this is the owner and repository name of
    /// the action.
    action_repository: String,
    /// For a composite action, the current result of the composite action.
    action_status: String,
    /// The username of the user that triggered the initial workflow run.
    actor: String,
    /// The account ID of the person or app that triggered the initial workflow
    /// run.
    actor_id: String,
    /// The URL of the GitHub REST API.
    api_url: String,
    /// The base_ref or target branch of the pull request in a workflow run.
    base_ref: String,
    /// Path on the runner to the file that sets environment variables from
    /// workflow commands.
    env: String,
    /// The full event webhook payload.
    event: serde_json::Value,
    /// The name of the event that triggered the workflow run.
    event_name: String,
    /// The path to the file on the runner that contains the full event webhook
    /// payload.
    event_path: String,
    /// The URL of the GitHub GraphQL API.
    graphql_url: String,
    /// The head_ref or source branch of the pull request in a workflow run.
    head_ref: String,
    /// The job id of the current job.
    job: String,
    /// The path of the repository.
    path: String,
    /// The short ref name of the branch or tag that triggered the workflow run.
    ref_name: String,
    /// true if branch protections are configured for the ref that triggered the
    /// workflow run.
    ref_protected: bool,
    /// The type of ref that triggered the workflow run. Valid values are branch
    /// or tag.
    ref_type: String,
    /// The owner and repository name.
    repository: String,
    /// The ID of the repository.
    repository_id: String,
    /// The repository owner's username.
    repository_owner: String,
    /// The repository owner's account ID.
    repository_owner_id: String,
    /// The Git URL to the repository.
    repository_url: String,
    /// The number of days that workflow run logs and artifacts are kept.
    retention_days: String,
    /// A unique number for each workflow run within a repository.
    run_id: String,
    /// A unique number for each run of a particular workflow in a repository.
    run_number: String,
    /// A unique number for each attempt of a particular workflow run in a
    /// repository.
    run_attempt: String,
    /// The source of a secret used in a workflow.
    secret_source: String,
    /// The URL of the GitHub server.
    server_url: String,
    /// The commit SHA that triggered the workflow.
    sha: String,
    /// A token to authenticate on behalf of the GitHub App installed on your
    /// repository.
    token: String,
    /// The username of the user that initiated the workflow run.
    triggering_actor: String,
    /// The name of the workflow.
    workflow: String,
    /// The ref path to the workflow.
    workflow_ref: String,
    /// The commit SHA for the workflow file.
    workflow_sha: String,
    /// The default working directory on the runner for steps.
    workspace: String,
}

impl Expr<Github> {
    pub fn ref_(&self) -> Expr<String> {
        self.select("ref")
    }
}

#[derive(Expr)]

/// The job context contains information about the currently running job.
pub struct Job {
    /// A unique number for each container in a job. This property is only
    /// available if the job uses a container.
    container: Container,

    /// The services configured for a job. This property is only available if
    /// the job uses service containers.
    services: Services,

    /// The status of the current job.
    status: JobStatus,
}

/// The status of a job execution
#[derive(Clone)]
pub enum JobStatus {
    /// The job completed successfully
    Success,
    /// The job failed
    Failure,
    /// The job was cancelled
    Cancelled,
}

#[derive(Expr)]

/// Container information for a job. This is only available if the job runs in a
/// container.
pub struct Container {
    /// The ID of the container
    id: String,
    /// The container network
    network: String,
}

#[derive(Expr)]

/// Services configured for a job. This is only available if the job uses
/// service containers.
pub struct Services {}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_expr() {
        let github = Expr::github(); // Expr<Github>

        assert_eq!(github.to_string(), "{{ github }}");

        let action = github.action(); // Expr<String>
        assert_eq!(action.to_string(), "{{ github.action }}");

        let action_path = github.action_path(); // Expr<String>
        assert_eq!(action_path.to_string(), "{{ github.action_path }}");
    }
}
