use gh_workflow::*;
use release_plz::Release;
use toolchain::Toolchain;

#[test]
fn generate() {
    let flags = RustFlags::deny("warnings");

    let build = Job::new("Build and Test")
        .add_step(
            Step::checkout()
                .add_with(("token", "{{ secrets.GITHUB_TOKEN }}"))
                .add_with(("fetch-depth", 0)),
        )
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Step::uses("actions-ecosystem", "action-get-labeled", 2)
                .id("check-labels")
                .add_with(("github_token", "${{ secrets.GITHUB_TOKEN }}"))
                .continue_on_error(false)
                .name("Check PR Labels"),
        )
        .add_step(
            Cargo::new("test")
                .args("--all-features --workspace")
                .name("Cargo Test"),
        )
        .add_step(
            Cargo::new("fmt")
                .if_condition("github.event_name == 'pull_request' && !contains(steps.Check_PR_Labels.outputs.labels, 'ci: lintfix')")
                .nightly()
                .args("--check")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("fmt")
                .if_condition("github.event_name == 'pull_request' && contains(steps.Check_PR_Labels.outputs.labels, 'ci: lintfix')")
                .nightly()
                .args("--all")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .if_condition("github.event_name == 'pull_request' && !contains(steps.Check_PR_Labels.outputs.labels, 'ci: lintfix')")
                .args("--all-features --workspace -- -D warnings")
                .name("Cargo Clippy"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .if_condition("github.event_name == 'pull_request' && contains(steps.Check_PR_Labels.outputs.labels, 'ci: lintfix')")
                .args("--fix --allow-dirty")
                .name("Cargo Clippy"),
        )
        .add_step(
            Step::run(
                "git config user.name 'github-actions[bot]' && \
                     git config user.email 'github-actions[bot]@users.noreply.github.com' && \
                     git add . && \
                     git commit -m 'Apply lint fixes (fmt + clippy)' && \
                     git push origin HEAD:${{ github.event.pull_request.head.ref }}",
            )
            .if_condition(Expression::new("github.event_name == 'pull_request' && !contains(steps.Check_PR_Labels.outputs.labels, 'ci: lintfix')"))
            .name("Commit and Push Fixes"),
        );

    let event = Event::default()
        .push(Push::default().add_branch("main"))
        .pull_request(
            PullRequest::default()
                .add_type(PullRequestType::Opened)
                .add_type(PullRequestType::Synchronize)
                .add_type(PullRequestType::Reopened)
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
