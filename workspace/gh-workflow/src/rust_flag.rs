//! A type-safe representation of the Rust toolchain.

use std::fmt;
use std::fmt::{Display, Formatter};

use crate::{Job, SetEnv, Step, Workflow};

#[derive(Clone)]
pub enum RustFlags {
    Lint(String, Lint),
    Combine(Box<RustFlags>, Box<RustFlags>),
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

impl core::ops::Add for RustFlags {
    type Output = RustFlags;

    fn add(self, rhs: Self) -> Self::Output {
        RustFlags::Combine(Box::new(self), Box::new(rhs))
    }
}

impl RustFlags {
    pub fn allow<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Allow)
    }

    pub fn warn<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Warn)
    }

    pub fn deny<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Deny)
    }

    pub fn forbid<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Forbid)
    }

    pub fn codegen<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Codegen)
    }
}

impl Display for RustFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RustFlags::Lint(name, lint) => match lint {
                Lint::Allow => write!(f, "-A {}", name),
                Lint::Warn => write!(f, "-W {}", name),
                Lint::Deny => write!(f, "-D {}", name),
                Lint::Forbid => write!(f, "-F {}", name),
                Lint::Codegen => write!(f, "-C {}", name),
                Lint::Experiment => write!(f, "-Z {}", name),
            },
            RustFlags::Combine(lhs, rhs) => write!(f, "{} {}", lhs, rhs),
        }
    }
}

impl SetEnv<Job> for RustFlags {
    fn apply(self, mut value: Job) -> Job {
        let mut env = value.env.unwrap_or_default();
        env.insert("RUSTFLAGS".to_string(), self.to_string());
        value.env = Some(env);
        value
    }
}

impl SetEnv<Workflow> for RustFlags {
    fn apply(self, mut value: Workflow) -> Workflow {
        let mut env = value.env.unwrap_or_default();
        env.insert("RUSTFLAGS".to_string(), self.to_string());
        value.env = Some(env);
        value
    }
}

impl<T> SetEnv<Step<T>> for RustFlags {
    fn apply(self, mut value: Step<T>) -> Step<T> {
        let mut env = value.env.unwrap_or_default();
        env.insert("RUSTFLAGS".to_string(), self.to_string());
        value.env = Some(env);
        value
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_rust_flags() {
        let rust_flags = RustFlags::allow("Warnings");
        assert_eq!(rust_flags.to_string(), "-AWarnings");

        let rust_flags = RustFlags::warn("Warnings");
        assert_eq!(rust_flags.to_string(), "-WWarnings");

        let rust_flags = RustFlags::deny("Warnings");
        assert_eq!(rust_flags.to_string(), "-DWarnings");

        let rust_flags = RustFlags::forbid("Warnings");
        assert_eq!(rust_flags.to_string(), "-FWarnings");

        let rust_flags = RustFlags::codegen("Warnings");
        assert_eq!(rust_flags.to_string(), "-CWarnings");

        let rust_flags = RustFlags::allow("Warnings") + RustFlags::warn("Warnings");
        assert_eq!(rust_flags.to_string(), "-AWarnings -WWarnings");
    }
}