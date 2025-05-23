//! StandardWorkflow is designed to be used for most Rust projects that are
//! built at Tailcall. Though gh-workflow makes it much easier to write
//! workflows you still need to constantly keep referring to the Github
//! documentation to write your own workflows. This module saves all that time
//! by using feature flags to enable or disable features that you want in your
//! workflow. Based on the features enabled or disabled a workflow is generated.

use ctx::Context;
use derive_setters::Setters;
use generate::Generate;
use gh_workflow::error::Result;
use gh_workflow::{Workflow as GHWorkflow, *};
use heck::ToTitleCase;
use release_plz::{Command, Release};
use toolchain::Toolchain;

/// Defines the test runner to use for running tests
#[derive(Debug, Clone, Default)]
pub enum TestRunner {
    /// Uses the default cargo test runner
    Cargo,

    /// Uses cargo-nextest for running tests
    #[default]
    Nextest,
}

#[derive(Debug, Clone, Setters)]
pub struct StandardWorkflow {
    /// When enabled, a release job is added to the workflow.
    /// *IMPORTANT:* Ensure `secrets.CARGO_REGISTRY_TOKEN` is set for your
    /// github action.
    pub auto_release: bool,

    /// Name of the workflow.
    pub name: String,

    /// When enabled, a benchmark job is added to the workflow.
    pub benchmarks: bool,

    /// Auto-fixes the code after
    pub auto_fix: bool,

    /// Steps to be executed before the checkout step
    pub setup: Vec<Step<Run>>,

    /// The test runner to use for running tests
    pub test_runner: TestRunner,
}

impl Default for StandardWorkflow {
    fn default() -> Self {
        Self {
            auto_release: false,
            name: "ci".into(),
            benchmarks: false,
            auto_fix: false,
            setup: Vec::new(),
            test_runner: TestRunner::default(),
        }
    }
}

impl StandardWorkflow {
    /// Initialize a job with common configuration including:
    /// - Permissions
    /// - Setup steps
    /// - Checkout step
    ///
    /// This reduces duplication across different job types.
    fn init_job(&self, name: impl ToString) -> Job {
        let mut job = Job::new(name).permissions(Permissions::default().contents(Level::Read));

        // Add setup steps in reverse order to maintain the correct sequence
        for step in self.setup.iter().rev() {
            job = job.add_step(step.clone());
        }

        job.add_step(Step::checkout())
    }

    /// Add a setup step to be executed before the checkout step.
    ///
    /// # Example
    /// ```ignore
    /// use gh_workflow_tailcall::*;
    /// let workflow = StandardWorkflow::default()
    ///     .add_setup(Step::run("git config --global core.autocrlf false")
    ///         .name("Configure git"));
    /// ```
    pub fn add_setup<S: Into<Step<Run>>>(mut self, step: S) -> Self {
        self.setup.push(step.into());
        self
    }
}

impl StandardWorkflow {
    /// Generates and tests the workflow file.
    pub fn generate(self) -> Result<()> {
        self.to_ci_workflow().generate()?;
        Generate::new(self.to_autofix_workflow())
            .name("autofix.yml")
            .generate()?;
        Ok(())
    }

    /// Converts the workflow into a Github workflow.
    fn to_autofix_workflow(&self) -> GHWorkflow {
        // NOTE: The workflow name needs to by `autofix.ci`
        GHWorkflow::new("autofix.ci")
            .add_env(self.workflow_flags())
            .on(self.workflow_event())
            .add_job("lint", self.lint_job(true))
    }

    /// Converts the workflow into a Github workflow.
    pub fn to_ci_workflow(&self) -> GHWorkflow {
        GHWorkflow::new(self.name.clone())
            .add_env(self.workflow_flags())
            .on(self.workflow_event())
            .add_job("build", self.test_job())
            .add_job("lint", self.lint_job(false))
            .add_job_when(
                self.auto_release,
                "release",
                self.release_job(Command::Release),
            )
            .add_job_when(
                self.auto_release,
                "release-pr",
                self.release_job(Command::ReleasePR),
            )
    }

    fn release_job(&self, cmd: Command) -> Job {
        self.init_job(cmd.to_string().to_title_case())
            .concurrency(
                Concurrency::new(Expression::new("release-${{github.ref}}"))
                    .cancel_in_progress(false),
            )
            .cond(self.workflow_cond())
            .add_needs(self.test_job())
            .add_needs(self.lint_job(false))
            .add_env(Env::github())
            .add_env(Env::new(
                "CARGO_REGISTRY_TOKEN",
                "${{ secrets.CARGO_REGISTRY_TOKEN }}",
            ))
            .permissions(self.write_permissions())
            .add_step(Release::default().command(cmd))
    }

    fn lint_job(&self, auto_fix: bool) -> Job {
        let job = self.init_job(if auto_fix { "Lint Fix" } else { "Lint" });

        let job = if auto_fix {
            job.concurrency(
                Concurrency::new(Expression::new("autofix-${{github.ref}}"))
                    .cancel_in_progress(false),
            )
        } else {
            job
        };

        job.add_step(
            Toolchain::default()
                .add_nightly()
                .add_clippy()
                .add_fmt()
                .cache(true)
                .cache_directories(vec![
                    "~/.cargo/registry".into(),
                    "~/.cargo/git".into(),
                    "target".into(),
                ]),
        )
        .add_step(
            Cargo::new("fmt")
                .name("Cargo Fmt")
                .nightly()
                .add_args("--all")
                .add_args_when(!auto_fix, "--check"),
        )
        .add_step(
            Cargo::new("clippy")
                .name("Cargo Clippy")
                .nightly()
                .add_args_when(auto_fix, "--fix")
                .add_args_when(auto_fix, "--allow-dirty")
                .add_args("--all-features --workspace -- -D warnings"),
        )
        .add_step_when(
            auto_fix,
            Step::uses(
                "autofix-ci",
                "action",
                "551dded8c6cc8a1054039c8bc0b8b48c51dfc6ef",
            ),
        )
    }

    /// Creates the "Build and Test" job for the workflow.
    fn test_job(&self) -> Job {
        self.init_job("Build and Test")
            .add_step(Toolchain::default().add_stable())
            .add_step_when(
                matches!(self.test_runner, TestRunner::Nextest),
                Cargo::new("install")
                    .args("cargo-nextest --locked")
                    .name("Install nextest"),
            )
            .add_step(match self.test_runner {
                TestRunner::Cargo => Cargo::new("test")
                    .args("--all-features --workspace")
                    .name("Cargo Test"),
                TestRunner::Nextest => Cargo::new("nextest")
                    .args("run --all-features --workspace")
                    .name("Cargo Nextest"),
            })
            .add_step_when(
                self.benchmarks,
                Cargo::new("bench").args("--workspace").name("Cargo Bench"),
            )
    }

    fn write_permissions(&self) -> Permissions {
        Permissions::default()
            .pull_requests(Level::Write)
            .packages(Level::Write)
            .contents(Level::Write)
    }

    fn workflow_cond(&self) -> Context<bool> {
        let is_main = Context::github().ref_().eq("refs/heads/main".into());
        let is_push = Context::github().event_name().eq("push".into());

        is_main.and(is_push)
    }

    fn workflow_event(&self) -> Event {
        Event::default()
            .push(Push::default().add_branch("main").add_tag("v*"))
            .pull_request(
                PullRequest::default()
                    .add_type(PullRequestType::Opened)
                    .add_type(PullRequestType::Synchronize)
                    .add_type(PullRequestType::Reopened)
                    .add_branch("main"),
            )
    }

    fn workflow_flags(&self) -> RustFlags {
        RustFlags::deny("warnings")
    }
}
