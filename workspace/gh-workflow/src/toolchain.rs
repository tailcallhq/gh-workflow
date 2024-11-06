//! The typed version of https://github.com/actions-rust-lang/setup-rust-toolchain

use std::fmt::{Display, Formatter};

use derive_setters::Setters;

use crate::{AddStep, RustFlags};

#[derive(Clone)]
pub enum Version {
    Stable,
    Nightly,
    Custom((u64, u64, u64)),
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Version::Stable => write!(f, "stable"),
            Version::Nightly => write!(f, "nightly"),
            Version::Custom(s) => write!(f, "{}.{}.{}", s.0, s.1, s.2),
        }
    }
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version::Custom((major, minor, patch))
    }
}

#[derive(Clone)]
pub enum Component {
    Clippy,
    Rustfmt,
    Rls,
    RustAnalyzer,
    Cargo,
    Miri,
    LlvmTools,
    Rustc,
    RustDoc,
    RustGdb,
    RustLldb,
    RustAnalysis,
    RustSrc,
    RustStd,
    RustDev,
    RustcDev,
    ClippyDev,
    RlsDev,
    RustAnalyzerDev,
    CargoDev,
    MiriDev,
    LlvmToolsDev,
    RustDocDev,
    RustGdbDev,
    RustLldbDev,
    RustAnalysisDev,
    RustSrcDev,
    RustStdDev,
}

// TODO: Setup correct optionals
/// A Rust representation for the inputs of the setup-rust action.
/// More information can be found [here](https://github.com/actions-rust-lang/setup-rust-toolchain/blob/main/action.yml).
#[derive(Default, Clone, Setters)]
#[setters(strip_option)]
pub struct RustToolchainStep {
    pub toolchain: Vec<Version>,
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

impl RustToolchainStep {
    pub fn add_version(mut self, version: Version) -> Self {
        self.toolchain.push(version);
        self
    }
}

impl AddStep for RustToolchainStep {
    fn apply(self, job: crate::Job) -> crate::Job {
        todo!()
    }
}
