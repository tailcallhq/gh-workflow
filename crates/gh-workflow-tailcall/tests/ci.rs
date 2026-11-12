//! Generates this repository's own workflows.
//!
//! Every job runs on the self-hosted `sccache` pool. The label is written alone
//! (no `self-hosted`): the pool's scaler uses `noDefaultLabels`, so a job that
//! asks for any other label would never get an instance started.

use gh_workflow_tailcall::generate::Generate;
use gh_workflow_tailcall::{
    release_drafter_workflow, release_publish_workflow, StandardWorkflow, Workflow,
};

const RUNNER_LABEL: &str = "sccache";

/// Points every job of `workflow` at the sccache runner pool.
fn on_sccache(workflow: Workflow) -> Workflow {
    let mut value = serde_json::to_value(&workflow).unwrap();
    for job in value["jobs"].as_object_mut().unwrap().values_mut() {
        job["runs-on"] = serde_json::json!([RUNNER_LABEL]);
    }
    serde_json::from_value(value).unwrap()
}

#[test]
fn generate() {
    let standard = StandardWorkflow::default().auto_fix(true).to_owned();
    Generate::new(on_sccache(standard.to_ci_workflow()))
        .name("ci.yml")
        .generate()
        .unwrap();
    Generate::new(on_sccache(standard.to_autofix_workflow()))
        .name("autofix.yml")
        .generate()
        .unwrap();
}

#[test]
fn generate_release_workflows() {
    Generate::new(on_sccache(release_drafter_workflow()))
        .name("release-drafter.yml")
        .generate()
        .unwrap();
    Generate::new(on_sccache(release_publish_workflow()))
        .name("release.yml")
        .generate()
        .unwrap();
}
