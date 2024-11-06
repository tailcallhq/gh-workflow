use std::time::Duration;

use gh_workflow::{Job, Permissions, Step};

fn main() {
    // TODO: replace `with` with RustToolchain struct.

    let job = Job::new("Run tests")
        .runs_on("ubuntu-latest")
        .timeout(Duration::from_secs(10 * 60))
        .add_step(Step::uses("actions", "checkout", 4).name("Checkout code"))
        .add_step(
            Step::uses("actions-rust-lang", "setup-rust-toolchain", 1).name("Setup rust").with(
                (
                    "toolchain",
                    "stable",
                ),
            ),
            Step::new("Setup rust")
                .uses("actions-rust-lang/setup-rust-toolchain@v1")
                .with(("toolchain", "stable")),
        )
        .add_step(
            Step::new("Run tests")
                .run("RUSTFLAGS=\"-Awarnings\" cargo test --all-features --workspace"),
        );

    let workflow = gh_workflow::Workflow::new("CI")
        .permissions(Permissions::read())
        .on(vec![
            ("push", vec![("branches", vec!["main"])]),
            (
                "pull_request",
                vec![
                    ("types", vec!["opened", "synchronize", "reopened"]),
                    ("branches", vec!["main"]),
                ],
            ),
        ])
        .add_job("test", job)
        .unwrap();

    workflow
        .write(format!(
            "{}/../../.github/workflows/ci.yml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
}
