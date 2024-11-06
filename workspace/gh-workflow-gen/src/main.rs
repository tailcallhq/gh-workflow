use gh_workflow::{Job, Permissions, RustFlags, Step, Version, Workflow};

fn main() {
    let rust_flags = RustFlags::allow("Warnings");

    let build = Job::new("build")
        .add_step(Step::checkout())
        .add_step(
            Step::rust_toolchain()
                .add_version(Version::Stable)
                .add_version(Version::Nightly),
        )
        .add_step(
            // TODO: Cargo commands should be more type-safe
            Step::run("cargo test --all-features --workspace").name("Run Test"),
        )
        .add_step(
            // TODO: Cargo fmt command should be more type-safe
            Step::run("cargo +nightly fmt --check").name("Run Fmt"),
        )
        .add_step(
            // TODO: Cargo clippy command should be more type-safe
            Step::run("cargo +nightly clippy --all-features --workspace -- -D warnings")
                .name("Run Clippy"),
        );

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
