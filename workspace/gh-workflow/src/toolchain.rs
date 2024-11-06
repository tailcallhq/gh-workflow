use std::time::Duration;

use derive_setters::Setters;

use crate::error::Result;
use crate::workflow::*;

///
/// A type-safe representation of the Rust toolchain.
/// Instead of writing the github action for Rust by hand, we can use this
/// struct to generate the github action.
#[derive(Default, Clone)]
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

#[derive(Setters, Default, Clone)]
#[setters(strip_option)]
pub struct RustToolchain {
    version: Version,
    fmt: bool,
    clippy: bool,
    timeout: Option<Duration>,
    workspace: bool,
}

impl RustToolchain {
    pub fn to_workflow(&self) -> Result<Workflow> {
        let mut job = Job::new("Rust Job")
            .runs_on("ubuntu-latest")
            .add_step(Step::uses("actions", "checkout", 4).name("Checkout Code"))
            .add_step(
                Step::uses("actions-rust-lang", "setup-rust-toolchain", 1)
                    .name("Setup Rust Toolchain")
                    .with(("toolchain", self.version.clone())),
            );

        if let Some(timeout) = self.timeout {
            job = job.timeout(timeout);
        }

        let mut cargo_test_args = vec!["--all-features"];

        if self.workspace {
            cargo_test_args.push("--workspace");
        }

        job = job.add_step(
            Step::run(format!(
                "RUSTFLAGS=\"-Awarnings\" cargo test {}",
                cargo_test_args.join(" ")
            ))
            .name("Run Cargo Test"),
        );

        if self.fmt {
            job = job.add_step(Step::run("cargo fmt -- --check").name("Check formatting"));
        }

        if self.clippy {
            job = job.add_step(Step::run("cargo clippy -- -D warnings").name("Run clippy"));
        }

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
            .add_job("rust", job)
    }
}
