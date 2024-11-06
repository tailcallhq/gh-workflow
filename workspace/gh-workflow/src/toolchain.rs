use std::time::Duration;

use derive_setters::Setters;

use crate::error::Result;
use crate::workflow::*;

///
/// A type-safe representation of the Rust toolchain.
/// Instead of writing the github action for Rust by hand, we can use this
/// struct to generate the github action.
#[derive(Default)]
pub enum Version {
    #[default]
    Stable,
    Beta,
    Nightly,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Version::Stable => "stable".to_string(),
            Version::Beta => "beta".to_string(),
            Version::Nightly => "nightly".to_string(),
        }
    }
}

#[derive(Setters, Default)]
pub struct RustToolchain {
    version: Version,
    fmt: bool,
    clippy: bool,
    // TODO: add more rust tool chain components
}

impl RustToolchain {
    pub fn to_workflow(&self) -> Result<Workflow> {
        let job = Job::new("Run tests")
            .runs_on("ubuntu-latest")
            .timeout(Duration::from_secs(10 * 60))
            .add_step(Step::uses("actions", "checkout", 4).name("Checkout code"))
            .add_step(
                Step::uses("actions-rust-lang", "setup-rust-toolchain", 1)
                    .name("Setup rust")
                    .with(("toolchain", "stable")),
            )
            .add_step(
                Step::run("RUSTFLAGS=\"-Awarnings\" cargo test --all-features --workspace")
                    .name("Run tests"),
            );

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
            .add_job("test", job)
    }
}
