use gh_workflow::{Component, Job, Permissions, RustFlags, Step, Toolchain, Workflow};

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
        .add_step(Step::cargo("test", vec!["--all-features", "--workspace"]))
        .add_step(Step::cargo_nightly("fmt", vec!["--check"]))
        .add_step(Step::cargo_nightly(
            "clippy",
            vec!["--all-features", "--workspace"],
        ));

    Workflow::new("CI")
        .env(rust_flags)
        .permissions(Permissions::read())
        .on(vec![
            // TODO: enums
            ("push", vec![("branches", vec!["main"])]),
            (
                "pull_request",
                vec![
                    ("types", vec!["opened", "synchronize", "reopened"]),
                    ("branches", vec!["main"]),
                ],
            ),
        ])
        .add_job("build", build)
        .unwrap()
        .generate(format!(
            "{}/../../.github/workflows/ci.yml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
}
