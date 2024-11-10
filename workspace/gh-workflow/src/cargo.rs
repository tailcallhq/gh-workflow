use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
pub enum Command {
    Add,
    Bench,
    Build,
    Check,
    Chef,
    Clean,
    Clippy,
    Component,
    Config,
    Criterion,
    Doc,
    Expand,
    Fetch,
    Fix,
    Flamegraph,
    Fmt,
    Generate,
    GenerateLockfile,
    GitCheckout,
    Help,
    Info,
    Init,
    Insta,
    Install,
    LocateProject,
    Login,
    Logout,
    Make,
    Metadata,
    Miri,
    New,
    Owner,
    Package,
    ReadManifest,
    Remove,
    Report,
    Run,
    Rustc,
    Rustdoc,
    Search,
    Test,
    Tree,
    Udeps,
    Uninstall,
    Update,
    Vendor,
    VerifyProject,
    Version,
    Watch,
    Yank,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Test;

#[derive(Debug, PartialEq, Eq)]
pub struct Clippy;

#[derive(Debug, PartialEq, Eq)]
pub struct Fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Check;

#[derive(Debug, PartialEq, Eq)]
pub struct Clean;

#[derive(Debug, PartialEq, Eq)]
pub struct Cargo<T> {
    pub command: Command,
    pub args: Vec<String>,
    marker: PhantomData<T>,
}

impl<T> Cargo<T> {
    // TODO: add function to handle command
    // specific sub command
    pub fn add_arg<S: ToString>(mut self, arg: S) -> Self {
        self.args.push(arg.to_string());

        self
    }
}

impl Cargo<Clean> {
    pub fn clean() -> Cargo<Clean> {
        Cargo {
            command: Command::Clean,
            args: vec![],
            marker: Default::default(),
        }
    }
}

impl Cargo<Check> {
    pub fn check() -> Cargo<Check> {
        Cargo {
            command: Command::Check,
            args: vec![],
            marker: Default::default(),
        }
    }
}

impl Cargo<Test> {
    pub fn test() -> Cargo<Test> {
        Cargo {
            command: Command::Test,
            args: vec![],
            marker: Default::default(),
        }
    }
    pub fn all_features(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--all-features") {
            self.args.push("--all-features".to_string());
        }

        self
    }
    pub fn workspace(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--workspace") {
            self.args.push("--workspace".to_string());
        }
        self
    }
}

impl Cargo<Clippy> {
    pub fn clippy() -> Cargo<Clippy> {
        Cargo {
            command: Command::Clippy,
            args: vec![],
            marker: Default::default(),
        }
    }
    pub fn all_features(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--all-features") {
            self.args.push("--all-features".to_string());
        }
        self
    }
    pub fn workspace(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--workspace") {
            self.args.push("--workspace".to_string());
        }
        self
    }
}

impl Cargo<Fmt> {
    pub fn fmt() -> Cargo<Fmt> {
        Cargo {
            command: Command::Fmt,
            args: vec![],
            marker: Default::default(),
        }
    }
    pub fn all(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--all") {
            self.args.push("--all".to_string());
        }

        self
    }
    pub fn check(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--check") {
            self.args.push("--check".to_string());
        }
        self
    }
}
