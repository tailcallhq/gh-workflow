use gh_workflow::*;
use gh_workflow_release_plz::{Command, ReleasePlz};
use toolchain::Toolchain;

fn main() {
    let flags = RustFlags::deny("warnings");

    let job = Job::new("Build and Test")
        .add_step(Step::checkout())
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Cargo::new("test")
                .args("--all-features --workspace")
                .name("Cargo Test"),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--check")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .args("--all-features --workspace -- -D warnings")
                .name("Cargo Clippy"),
        )
        .add_step(Step::run("find . -name Cargo.toml"))
        .add_step(ReleasePlz::default().command(Command::ReleasePR))
        .add_github_token();

    let event = Event::default()
        .push(Push::default().add_branch("main"))
        .pull_request_target(
            PullRequestTarget::default()
                .open()
                .synchronize()
                .reopen()
                .add_branch("main"),
        );

    Workflow::new("Build and Test")
        .add_env(flags)
        .permissions(Permissions::read())
        .on(event)
        .add_job("build", job)
        .generate()
        .unwrap();
}
