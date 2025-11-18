use gh_workflow_tailcall::StandardWorkflow;

#[test]
fn generate() {
    StandardWorkflow::default()
        .auto_fix(true)
        .to_owned()
        .generate()
        .unwrap();
}
