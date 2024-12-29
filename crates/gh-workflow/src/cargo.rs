use derive_setters::Setters;

use crate::toolchain::Version;
use crate::{Env, Run, Step};

#[derive(Clone, Setters)]
#[setters(strip_option, into)]
pub struct Cargo {
    /// The command to be executed for eg: fmt, clippy, build, test, etc.
    pub command: String,

    /// The unique identifier of the Step.
    pub id: Option<String>,

    /// Name of the Step
    pub name: Option<String>,

    /// Toolchain to be used for example `+nightly`.
    pub toolchain: Option<Version>,

    /// Arguments to be passed to the cargo command.
    #[setters(skip)]
    pub args: Vec<String>,

    pub env: Option<Env>,
}

impl Cargo {
    /// Creates a new `Cargo` instance with the specified command.
    pub fn new<T: ToString>(cmd: T) -> Cargo {
        Cargo {
            command: cmd.to_string(),
            id: Default::default(),
            name: Default::default(),
            toolchain: Default::default(),
            args: Default::default(),
            env: Default::default(),
        }
    }

    pub fn add_env<T: Into<Env>>(mut self, new_env: T) -> Self {
        let mut env = self.env.unwrap_or_default();
        env.0.extend(new_env.into().0);

        self.env = Some(env);
        self
    }

    /// Sets the toolchain to nightly.
    pub fn nightly(mut self) -> Self {
        self.toolchain = Some(Version::Nightly);
        self
    }

    /// Sets the arguments for the cargo command. If arguments are already set,
    /// it will be overwritten.
    pub fn args<T: ToString>(mut self, args: T) -> Self {
        self.args = vec![args.to_string()];
        self
    }

    /// Adds additional arguments to the cargo command.
    pub fn add_args<T: ToString>(mut self, args: T) -> Self {
        self.args.extend(
            args.to_string()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Adds the arguments to the cargo command when a condition is met.
    pub fn add_args_when<T: ToString>(self, when: bool, args: T) -> Self {
        if when {
            self.add_args(args)
        } else {
            self
        }
    }
}

impl From<Cargo> for Step<Run> {
    fn from(value: Cargo) -> Self {
        let mut command = vec!["cargo".to_string()];

        if let Some(toolchain) = value.toolchain {
            command.push(format!("+{}", toolchain));
        }

        command.push(value.command);

        // Extend the command with non-empty arguments
        command.extend(
            value
                .args
                .into_iter()
                .map(|arg| arg.trim().to_string())
                .filter(|arg| !arg.is_empty()),
        );

        let mut step = Step::run(command.join(" "));

        if let Some(id) = value.id {
            step = step.id(id);
        }

        if let Some(name) = value.name {
            step = step.name(name);
        }

        if let Some(env) = value.env {
            step = step.env(env);
        }

        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo() {
        let cargo = Cargo::new("fmt")
            .add_args("--all")
            .add_args("--check")
            .nightly()
            .add_env(Env::github());

        let step: Step<Run> = cargo.into();

        insta::assert_snapshot!(serde_yaml::to_string(&step).unwrap());
    }
}
