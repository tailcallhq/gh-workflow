use gh_workflow::{CargoCommand, Component, Event, Job, Permissions, PullRequest, Push, RustFlags, Step, Toolchain, Workflow};

fn main() {
    let rust_flags = RustFlags::deny("warnings");

    let build = Job::new("Build and Test")
        .add_step(Step::checkout())
        .add_step(
            Step::setup_rust()
                .add_toolchain(Toolchain::Stable)
                .add_toolchain(Toolchain::Nightly)
                .components(vec![Component::Clippy, Component::Rustfmt]),
        )
        .add_step(Step::cargo(CargoCommand::Test, vec!["--all-features", "--workspace"]))
        .add_step(Step::cargo_nightly(CargoCommand::Fmt, vec!["--check"]))
        .add_step(Step::cargo_nightly(
            CargoCommand::Clippy,
            vec!["--all-features", "--workspace"],
        ));

    Workflow::new("CI")
        .env(rust_flags)
        .permissions(Permissions::read())
        .on(Event::default()
            .push(Push::default().branch("main"))
            .pull_request(
                PullRequest::default()
                    .open()
                    .synchronize()
                    .reopen()
                    .branch("main"),
            ))
        .add_job("build", build)
        .unwrap()
        .generate(format!(
            "{}/../../.github/workflows/ci.yml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
}
