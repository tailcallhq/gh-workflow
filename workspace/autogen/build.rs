use indexmap::IndexMap;
use serde_json::Value;
use gh_workflow::{Expression, Job, OneOrManyOrObject, PermissionLevel, Permissions, Step, WorkflowOn};

fn main() {
    // TODO: replace `with` with RustToolchain struct.

    let mut with = IndexMap::new();
    with.insert("toolchain".to_string(), Value::String("stable".to_string()));
    let steps = vec![
        Step::new("Checkout code").uses("actions/checkout@v4".to_string()),
        Step::new("Setup rust").uses("actions-rust-lang/setup-rust-toolchain@v1".to_string()).with(with),
        Step::new("Run tests").run("cargo test --all-features --workspace".to_string()),
    ];

    let job = Job::new("Run tests")
        .runs_on(OneOrManyOrObject::Single("ubuntu-latest".to_string()))
        .timeout_minutes(10)
        .steps(steps);

    let mut on = IndexMap::new();
    let mut branches = IndexMap::new();
    branches.insert("branches".to_string(), WorkflowOn::Multiple(vec!["main".to_string()]));

    let mut on_pr = IndexMap::new();
    on_pr.insert("types".to_string(), WorkflowOn::Multiple(vec!["opened", "synchronize", "reopened"].into_iter().map(|x| x.to_string()).collect()));
    on_pr.insert("branches".to_string(), WorkflowOn::Multiple(vec!["main"].into_iter().map(|x| x.to_string()).collect()));

    on.insert("push".to_string(), WorkflowOn::Map(branches));
    on.insert("pull_request".to_string(), WorkflowOn::Map(on_pr));

    let workflow = gh_workflow::Workflow::new("CI")
        .permissions(Permissions::default().contents(PermissionLevel::Read))
        .on(WorkflowOn::Map(on))
        .add_job("test", job)
        .unwrap();

    workflow.write(format!("{}/../../.github/workflows/ci.yml",env!("CARGO_MANIFEST_DIR"))).unwrap();
}