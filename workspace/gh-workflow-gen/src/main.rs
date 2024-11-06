use gh_workflow::toolchain::{RustFlag, Version};
use gh_workflow::{Job, Permissions, Step, Workflow};

fn main() {
    let rust_flags = RustFlag::allow("Warnings");

    let stable = Job::new("build")
        .add_step(Step::checkout())
        .add_step(
            // TODO: Rust Tool Chain can be a separate struct
            Step::uses("actions-rust-lang", "setup-rust-toolchain", 1)
                .name("Setup Rust Toolchain")
                .with(("toolchain", Version::Stable)),
        )
        .add_step(
            // TODO: Cargo commands should be more type-safe
            Step::run("cargo test --all-features --workspace").name("Run Test"),
        );

    let nightly = Job::new("Nightly")
        .add_step(Step::uses("actions", "checkout", 4).name("Checkout Code"))
        .add_step(
            // TODO: Rust Tool Chain can be a separate struct
            Step::uses("actions-rust-lang", "setup-rust-toolchain", 1)
                .name("Setup Rust Toolchain")
                .with(("toolchain", Version::Nightly)),
        )
        .add_step(
            // TODO: Cargo fmt command should be more type-safe
            Step::run("cargo +nightly fmt --all-features --workspace -- check")
                .name("Run Fmt"),
        )
        .add_step(
            // TODO: Cargo clippy command should be more type-safe
            Step::run("cargo +nightly clippy --all-features --workspace -- -D warnings ")
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
