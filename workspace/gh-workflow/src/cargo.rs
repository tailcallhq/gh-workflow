use derive_setters::Setters;

use crate::toolchain::Version;
use crate::{Run, Step};

#[derive(Setters)]
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
    pub args: Option<String>,
}

impl Cargo {
    pub fn new<T: ToString>(cmd: T) -> Cargo {
        Cargo {
            command: cmd.to_string(),
            id: Default::default(),
            name: Default::default(),
            toolchain: Default::default(),
            args: Default::default(),
        }
    }

    pub fn nightly(mut self) -> Self {
        self.toolchain = Some(Version::Nightly);
        self
    }
}

impl From<Cargo> for Step<Run> {
    fn from(value: Cargo) -> Self {
        let mut command = vec!["cargo".to_string()];

        if let Some(toolchain) = value.toolchain {
            command.push(format!("+{}", toolchain));
        }

        command.push(value.command);

        if let Some(args) = value.args {
            if !args.is_empty() {
                command.push(args);
            }
        }

        let mut step = Step::run(command.join(" "));

        if let Some(id) = value.id {
            step = step.id(id);
        }

        if let Some(name) = value.name {
            step = step.name(name);
        }

        step
    }
}
