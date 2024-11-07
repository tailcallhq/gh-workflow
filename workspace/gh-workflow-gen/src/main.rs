use std::time::Duration;

use gh_workflow::{Job, Permissions, RustFlags, Step, Toolchain, Workflow};

fn main() {
    let rust_flags = RustFlags::deny("Warnings");

    let build = Job::new("build")
        .add_step(Step::checkout())
        .add_step(
            Step::setup_rust()
                .add_toolchain(Toolchain::Stable)
                .add_toolchain(Toolchain::Nightly),
        )
        .add_step(
            Step::cargo("test", vec!["--all-features", "--workspace"])
                .timeout(Duration::from_secs(10)),
        )
        .add_step(Step::cargo_nightly("fmt", vec!["--check"]))
        .add_step(Step::cargo_nightly(
            "clippy",
            vec!["--all-features", "--workspace", "-D", "warnings"],
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
