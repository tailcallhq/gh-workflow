use gh_workflow::*;

fn main() {
    let rust_flags = RustFlags::deny("warnings");

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
        .add_job(
            "build",
            Job::new("Build and Test")
                .add_step(Step::checkout())
                .add_step(
                    Step::setup_rust()
                        .add_toolchain(Toolchain::Stable)
                        .add_toolchain(Toolchain::Nightly)
                        .components(vec![Component::Clippy, Component::Rustfmt]),
                )
                .add_step(Step::cargo(Cargo::Test, vec!["--all-features", "--workspace"]))
                .add_step(Step::cargo_nightly(Cargo::Fmt, vec!["--check"]))
                .add_step(Step::cargo_nightly(
                    Cargo::Clippy,
                    vec!["--all-features", "--workspace", "--", "-D warnings"],
                )),
        )
        .unwrap()
        .generate()
        .unwrap();
}
