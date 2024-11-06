use gh_workflow::toolchain::Toolchain;
use gh_workflow::{Permissions, Workflow};

fn main() {
    let stable = Toolchain::stable().workspace(true).test(true);
    let nightly = Toolchain::nightly().workspace(true).fmt(true).clippy(true);

    Workflow::new("CI")
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
        .add_job("stable", stable)
        .unwrap()
        .add_job("nightly", nightly)
        .unwrap()
        .generate(format!(
            "{}/../../.github/workflows/ci.yml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
}
