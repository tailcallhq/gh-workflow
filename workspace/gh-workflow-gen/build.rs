use gh_workflow::*;
use toolchain::Toolchain;

fn main() {
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
        );

    let event = Event::default()
        .push(Push::default().add_branch("main"))
        .pull_request_target(
            PullRequestTarget::default()
                .open()
                .synchronize()
                .reopen()
                .add_branch("main"),
        );

    let flags = RustFlags::deny("warnings");

    Workflow::new("Build and Test")
        .env(flags)
        .permissions(Permissions::read())
        .on(event)
        .add_job("build", job)
        .generate()
        .unwrap();
}
