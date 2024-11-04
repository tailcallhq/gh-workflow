use indexmap::IndexMap;
use serde_json::Value;
use gh_workflow::{Expression, Job, OneOrManyOrObject, Step, WorkflowOn};

fn main() {
    // TODO: replace `with` with RustToolchain struct.

    let mut with = IndexMap::new();
    with.insert("toolchain".to_string(), Value::String("stable".to_string()));
    let steps = vec![
        Step::new("Checkout code").uses("actions/checkout@v2".to_string()),
        Step::new("Setup rust").uses("actions-rs/toolchain@v1".to_string()).with(with),
        Step::new("Run tests").run("cargo test --all-features --workspace".to_string()),
    ];

    let job = Job::new("Run tests")
        .runs_on(OneOrManyOrObject::Single("ubuntu-latest".to_string()))
        .if_condition(Expression::new("github.event_name == 'push'"))
        .timeout_minutes(10)
        .steps(steps);

    let mut on = IndexMap::new();
    let mut branches = IndexMap::new();
    branches.insert("branches".to_string(), WorkflowOn::Single("main".to_string()));

    let mut on_pr = IndexMap::new();
    on_pr.insert("types".to_string(), WorkflowOn::Multiple(vec!["opened", "synchronize", "reopened"].into_iter().map(|x| x.to_string()).collect()));

    on.insert("push".to_string(), WorkflowOn::Map(branches));
    on.insert("pull_request".to_string(), WorkflowOn::Map(on_pr));

    let workflow = gh_workflow::Workflow::new("CI")
        .on(WorkflowOn::Map(on))
        .add_job("test", job)
        .unwrap();

    workflow.write(format!("{}/../../.github/workflow/ci.yml",env!("CARGO_MANIFEST_DIR"))).unwrap();
}