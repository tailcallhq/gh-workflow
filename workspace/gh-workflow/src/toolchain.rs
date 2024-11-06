//! The typed version of https://github.com/actions-rust-lang/setup-rust-toolchain

use std::fmt::{Display, Formatter};

use derive_setters::Setters;

use crate::{AddStep, RustFlags};

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
        todo!()
    }
}
