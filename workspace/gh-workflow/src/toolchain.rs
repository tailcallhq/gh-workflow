use derive_setters::Setters;

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
impl Version {
    pub fn to_string(&self) -> String {
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
    pub fn to_job(&self) -> Job {
        todo!();
    }
}
