//! A type-safe representation of the Rust toolchain.
use std::time::Duration;

use derive_setters::Setters;

use crate::workflow::*;

pub trait Version: ToString {}

#[derive(Default, Clone)]
pub struct Stable;

impl Version for Stable {}
impl ToString for Stable {
    fn to_string(&self) -> String {
        "stable".to_string()
    }
}

#[derive(Default, Clone)]
pub struct Nightly;

impl Version for Nightly {}
impl ToString for Nightly {
    fn to_string(&self) -> String {
        "nightly".to_string()
    }
}

#[derive(Setters, Clone)]
pub struct Toolchain<Version> {
    version: Version,
    fmt: bool,
    clippy: bool,
    timeout: Option<Duration>,
    workspace: bool,
    test: bool,
}

impl<V: Version + Default> Toolchain<V> {
    pub fn new(version: V) -> Self {
        Toolchain {
            version,
            fmt: false,
            clippy: false,
            timeout: None,
            workspace: false,
            test: false,
        }
    }
}

impl Toolchain<Stable> {
    pub fn stable() -> Self {
        Toolchain::new(Stable)
    }
}

impl Toolchain<Nightly> {
    pub fn nightly() -> Self {
        Toolchain::new(Nightly)
    }
}

impl<V: Version> Into<Job> for Toolchain<V> {
    fn into(self) -> Job {
        let mut job = Job::new("Rust Job")
            .runs_on("ubuntu-latest")
            .add_step(Step::uses("actions", "checkout", 4).name("Checkout Code"))
            .add_step(
                Step::uses("actions-rust-lang", "setup-rust-toolchain", 1)
                    .name("Setup Rust Toolchain")
                    .with(("toolchain", self.version)),
            );

        if let Some(timeout) = self.timeout {
            job = job.timeout(timeout);
        }

        let mut cargo_test_args = vec!["--all-features"];

        if self.workspace {
            cargo_test_args.push("--workspace");
        }

        if self.test {
            job = job.add_step(
                Step::run(format!(
                    "RUSTFLAGS=\"-Awarnings\" cargo test {}",
                    cargo_test_args.join(" ")
                ))
                .name("Run Cargo Test"),
            );
        }

        if self.fmt {
            job = job.add_step(Step::run("cargo fmt -- --check").name("Check formatting"));
        }

        if self.clippy {
            job = job.add_step(Step::run("cargo clippy -- -D warnings").name("Run clippy"));
        }

        job
    }
}
