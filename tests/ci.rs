use gh_workflow::*;
use release_plz::Release;
use toolchain::Toolchain;

#[test]
fn generate() {
    let lint_mode_condition =
        "contains(github.event.pull_request.labels.*.name, 'ci: lintfix') && 'fix' || 'check'";

    let flags = RustFlags::deny("warnings");

    let build = Job::new("Build and Test")
        .add_env(("LINT_MODE", format!("${{{{{}}}}}", lint_mode_condition)))
        .add_step(Step::checkout())
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--check")
                .if_condition("env.LINT_MODE == 'check'")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("test")
                .args("--all --all-targets --all-features")
                .if_condition("env.LINT_MODE == 'fix'")
                .name("Cargo Test"),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--all")
                .if_condition("env.LINT_MODE == 'fix'")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .args("--all-features --workspace --fix --allow-staged --allow-dirty")
                .name("Cargo Clippy"),
        )
        .add_step(
            Cargo::new("test")
                .args("--all-features --workspace")
                .if_condition("env.LINT_MODE == 'check'")
                .name("Cargo Test"),
        )
        .add_step(
            Step::uses(
                "autofix-ci",
                "action",
                "ff86a557419858bb967097bfc916833f5647fa8c",
            )
            .if_condition(Expression::new("env.LINT_MODE == 'fix'"))
            .name("Commit and push if changed"),
        );

    let event = Event::default()
        .push(Push::default().add_branch("main"))
        .pull_request(
            PullRequest::default()
                .add_type(PullRequestType::Opened)
                .add_type(PullRequestType::Synchronize)
                .add_type(PullRequestType::Reopened)
                .add_type(PullRequestType::Labeled)
                .add_branch("main"),
        );

    let permissions = Permissions::default()
        .pull_requests(Level::Write)
        .packages(Level::Write)
        .contents(Level::Write);

    let release = Job::new("Release")
        .needs("build")
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions)
        .add_step(Step::checkout())
        .add_step(Release::default());

    Workflow::new("Build and Test")
        .add_env(flags)
        .on(event)
        .add_job("build", build)
        .add_job("release", release)
        .generate()
        .unwrap();
}
