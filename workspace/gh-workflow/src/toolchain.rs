//! A type-safe representation of the Rust toolchain.

use std::fmt::{Display, Formatter};

use crate::{AsEnv, Job, Step, Workflow};

pub enum Version {
    Stable,
    Nightly,
    Custom(String),
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Version::Stable => write!(f, "stable"),
            Version::Nightly => write!(f, "nightly"),
            Version::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl Version {
    pub fn new<S: ToString>(s: S) -> Self {
        Version::Custom(s.to_string())
    }
}

#[derive(Clone)]
pub enum RustFlag {
    Lint(String, Lint),
    Combine(Box<RustFlag>, Box<RustFlag>),
}

impl AsEnv<Job> for RustFlag {
    fn apply(self, mut value: Job) -> Job {
        let mut env = value.env.unwrap_or_default();
        env.insert("RUSTFLAGS".to_string(), self.to_string());
        value.env = Some(env);
        value
    }
}

impl AsEnv<Workflow> for RustFlag {
    fn apply(self, mut value: Workflow) -> Workflow {
        let mut env = value.env.unwrap_or_default();
        env.insert("RUSTFLAGS".to_string(), self.to_string());
        value.env = Some(env);
        value
    }
}

impl<T> AsEnv<Step<T>> for RustFlag {
    fn apply(self, mut value: Step<T>) -> Step<T> {
        let mut env = value.env.unwrap_or_default();
        env.insert("RUSTFLAGS".to_string(), self.to_string());
        value.env = Some(env);
        value
    }
}

#[derive(Clone)]
pub enum Lint {
    Allow,
    Warn,
    Deny,
    Forbid,
    Codegen,
    Experiment,
}

impl core::ops::Add for RustFlag {
    type Output = RustFlag;

    fn add(self, rhs: Self) -> Self::Output {
        RustFlag::Combine(Box::new(self), Box::new(rhs))
    }
}

impl RustFlag {
    pub fn allow<S: ToString>(name: S) -> Self {
        RustFlag::Lint(name.to_string(), Lint::Allow)
    }

    pub fn warn<S: ToString>(name: S) -> Self {
        RustFlag::Lint(name.to_string(), Lint::Warn)
    }

    pub fn deny<S: ToString>(name: S) -> Self {
        RustFlag::Lint(name.to_string(), Lint::Deny)
    }

    pub fn forbid<S: ToString>(name: S) -> Self {
        RustFlag::Lint(name.to_string(), Lint::Forbid)
    }

    pub fn codegen<S: ToString>(name: S) -> Self {
        RustFlag::Lint(name.to_string(), Lint::Codegen)
    }
}

impl Display for RustFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustFlag::Lint(name, lint) => match lint {
                Lint::Allow => write!(f, "-A{}", name),
                Lint::Warn => write!(f, "-W{}", name),
                Lint::Deny => write!(f, "-D{}", name),
                Lint::Forbid => write!(f, "-F{}", name),
                Lint::Codegen => write!(f, "-C{}", name),
                Lint::Experiment => write!(f, "-Z{}", name),
            },
            RustFlag::Combine(lhs, rhs) => write!(f, "{} {}", lhs, rhs),
        }
    }
}
