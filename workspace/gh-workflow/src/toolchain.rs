//! The typed version of https://github.com/actions-rust-lang/setup-rust-toolchain

use std::fmt::{Display, Formatter};
use std::time::Duration;
use derive_setters::Setters;
use indexmap::IndexMap;
use crate::{AddStep, AnyStep, Expression, RustFlags, Step};

#[derive(Clone)]
pub enum Toolchain {
    Stable,
    Nightly,
    Custom((u64, u64, u64)),
}

impl Display for Toolchain {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Toolchain::Stable => write!(f, "stable"),
            Toolchain::Nightly => write!(f, "nightly"),
            Toolchain::Custom(s) => write!(f, "{}.{}.{}", s.0, s.1, s.2),
        }
    }
}

impl Toolchain {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Toolchain::Custom((major, minor, patch))
    }
}

#[derive(Clone)]
pub enum Component {
    Clippy,
    Rustfmt,
    RustDoc,
}

// TODO: Setup correct optionals
/// A Rust representation for the inputs of the setup-rust action.
/// More information can be found [here](https://github.com/actions-rust-lang/setup-rust-toolchain/blob/main/action.yml).
/// NOTE: The public API should be close to the original action as much as possible.
#[derive(Default, Clone, Setters)]
#[setters(strip_option)]
pub struct ToolchainStep {
    pub toolchain: Vec<Toolchain>,
    pub target: Option<String>,
    pub components: Vec<Component>,
    pub cache: bool,
    pub cache_directories: Vec<String>,
    pub cache_workspaces: Vec<String>,
    pub cache_on_failure: bool,
    pub cache_key: Option<String>,
    pub matcher: bool,
    pub rust_flags: Option<RustFlags>,
    pub override_setup: bool,
}

impl ToolchainStep {
    pub fn add_toolchain(mut self, version: Toolchain) -> Self {
        self.toolchain.push(version);
        self
    }
}

impl AddStep for ToolchainStep {
    fn apply(self, job: crate::Job) -> crate::Job {
        let mut steps = vec![];
        steps.push(AnyStep::Use(Step::checkout()));

        if self.override_setup {
            let mut setup = Step::uses("actions-rust-lang", "setup-rust-toolchain", 1);
            setup = setup.with(IndexMap::new());
            steps.push(AnyStep::Use(setup));
        }

        if self.matcher {
            let matcher = Step::run(
                // TODO: needs check
                r#"echo "::add-matcher::${{ github.workspace }}/rust-error-matcher.json""#
            )
                .name("Add Rust Error Matcher");
            steps.push(AnyStep::Run(matcher));
        }

        let mut rust = Step::uses("actions-rust-lang", "setup-rust-toolchain", 1);

        // TODO: impl for vec![(.., ..)]
        let mut map = IndexMap::new();
        map.insert(
            "toolchain".to_string(),
            serde_json::Value::String(self.toolchain.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")),
        );
        rust = rust.with(map);

        steps.push(AnyStep::Use(rust));

        let mut command = vec![];
        if let Some(rust_flags) = self.rust_flags {
            command.push("RUSTFLAGS=".to_string() + &rust_flags.to_string());
        }

        command.extend(vec!["cargo", "test", "--all-features", "--workspace"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>());

        if let Some(target) = self.target {
            command.push("--target".to_string());
            command.push(target);
        }

        let test = Step::run(command.join(" "))
            .name("Run Tests")
            .timeout(Duration::from_secs(10)); // TODO: Add timeout to ToolchainStep
        steps.push(AnyStep::Run(test));

        if self.matcher {
            let mut matcher = Step::run(
                r#"echo "::remove-matcher owner=rust-error""#
            ).name("Remove Rust Error Matcher");
            matcher.if_condition = Some(Expression::new("always()"));
            steps.push(AnyStep::Run(matcher));
        }

        job
    }
}
