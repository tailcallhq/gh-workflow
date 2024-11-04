use indexmap::IndexMap;
use serde_json::Value;
use gh_workflow::{one_or_many_or_kv, workflow_on, Expression, Job, OneOrManyOrObject, PermissionLevel, Permissions, Step, WorkflowOn};

fn main() {
    // TODO: replace `with` with RustToolchain struct.

    let job = Job::new("Run tests")
        .runs_on(one_or_many_or_kv!("ubuntu-latest"))
        .timeout_minutes(10)
        .add_step(Step::new("Checkout code").uses("actions/checkout@v4"))
        .add_step(
            Step::new("Setup rust")
                .uses("actions-rust-lang/setup-rust-toolchain@v1")
                .with(("toolchain", "stable"))
        )
        .add_step(
            Step::new("Run tests").run(
                "RUSTFLAGS=\"-Awarnings\" cargo test --all-features --workspace"
            )
        );

    let workflow = gh_workflow::Workflow
    ::new("CI")
        .permissions(Permissions::default().contents(PermissionLevel::Read))
        .on(workflow_on!([
            ("push", workflow_on!([
                ("branches", workflow_on!([
                    "main"
                ]))
            ])),
            ("pull_request", workflow_on!([("types", workflow_on!(["opened", "synchronize", "reopened"])), ("branches", workflow_on!(["main"]))]))
        ]))
        .add_job("test", job)
        .unwrap();

    workflow
        .write(format!("{}/../../.github/workflows/ci.yml", env!("CARGO_MANIFEST_DIR")))
        .unwrap();
}
