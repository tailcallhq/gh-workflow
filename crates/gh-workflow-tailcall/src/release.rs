//! Release workflows inspired by the forge project's release pipeline.
//! Uses release-drafter to draft releases from merged PRs and a publish
//! workflow that pushes crates to crates.io when a release is published.

use gh_workflow::error::Result;
use gh_workflow::generate::Generate;
use gh_workflow::{Workflow as GHWorkflow, *};

/// Generates the release-drafter workflow that drafts a release on every
/// push to main and auto-labels pull requests.
///
/// # Errors
/// Returns an error if the workflow file cannot be generated or is outdated
/// on CI.
pub fn generate_release_drafter() -> Result<()> {
    let workflow = GHWorkflow::new("Release Drafter")
        .on(Event {
            push: Some(Push::default().add_branch("main")),
            pull_request_target: Some(
                PullRequestTarget::default()
                    .add_type(PullRequestType::Opened)
                    .add_type(PullRequestType::Reopened)
                    .add_type(PullRequestType::Synchronize)
                    .add_type(PullRequestType::Labeled)
                    .add_type(PullRequestType::Unlabeled)
                    .add_type(PullRequestType::Closed)
                    .add_branch("main"),
            ),
            ..Event::default()
        })
        .permissions(
            Permissions::default()
                .contents(Level::Write)
                .pull_requests(Level::Write),
        )
        .add_job(
            "update_release_draft",
            Job::new("update_release_draft")
                .add_step(
                    Step::new("Auto Labeler")
                        .uses("release-drafter", "release-drafter/autolabeler", "v7")
                        .if_condition(Expression::new(
                            "github.event_name == 'pull_request_target'",
                        ))
                        .add_env(("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}"))
                        .add_with(("config-name", "release-drafter.yml")),
                )
                .add_step(
                    Step::new("Release Drafter")
                        .uses("release-drafter", "release-drafter", "v7")
                        .add_env(("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}"))
                        .add_with(("config-name", "release-drafter.yml")),
                ),
        );

    Generate::new(workflow)
        .name("release-drafter.yml")
        .generate()
}

/// Generates the release publish workflow that publishes all workspace
/// crates to crates.io when a GitHub release is published. The crate
/// versions are derived from the release tag.
///
/// # Errors
/// Returns an error if the workflow file cannot be generated or is outdated
/// on CI.
pub fn generate_release_publish() -> Result<()> {
    let workflow = GHWorkflow::new("Release Publish")
        .on(Event {
            release: Some(Release::default().add_type(ReleaseType::Published)),
            ..Event::default()
        })
        .permissions(Permissions::default().contents(Level::Read))
        .add_job(
            "publish",
            Job::new("publish")
                .add_step(Step::checkout().name("Checkout Code"))
                .add_step(
                    Step::new("Setup Rust Toolchain")
                        .uses("actions-rust-lang", "setup-rust-toolchain", "v1")
                        .add_with(("toolchain", "stable")),
                )
                .add_step(Step::new("Set Version from Tag").run(
                    r#"VERSION="${GITHUB_REF_NAME#v}"
cargo install cargo-edit --locked
cargo set-version --workspace "$VERSION""#,
                ))
                .add_step(
                    Step::new("Publish to Crates.io")
                        .run(
                            r#"cargo publish -p gh-workflow-macros --allow-dirty
cargo publish -p gh-workflow --allow-dirty
cargo publish -p gh-workflow-tailcall --allow-dirty"#,
                        )
                        .add_env((
                            "CARGO_REGISTRY_TOKEN",
                            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
                        )),
                ),
        );

    Generate::new(workflow).name("release.yml").generate()
}
