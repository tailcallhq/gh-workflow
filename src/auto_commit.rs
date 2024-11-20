use derive_setters::Setters;

use crate::{Run, Step};

#[derive(Setters)]
#[setters(strip_option, into)]
pub struct AutoCommit {
    /// The commit message to be used.
    pub message: String,

    /// The unique identifier of the Step.
    pub id: Option<String>,

    /// Name of the Step.
    pub name: Option<String>,

    /// Git user name for the commit. Defaults to 'github-actions' if not set.
    pub user_name: Option<String>,

    /// Git user email for the commit. Defaults to 'github-actions@github.com' if not set.
    pub user_email: Option<String>,

    /// Files to include in the commit. If None, all changes are committed.
    pub files: Option<Vec<String>>,

    /// Whether to push the changes after committing. Defaults to false if not set.
    pub push: Option<bool>,
}

impl AutoCommit {
    /// Creates a new `AutoCommit` with the specified commit message.
    pub fn new<T: ToString>(msg: T) -> AutoCommit {
        AutoCommit {
            message: msg.to_string(),
            id: Default::default(),
            name: Default::default(),
            user_name: Default::default(),
            user_email: Default::default(),
            files: Default::default(),
            push: Default::default(),
        }
    }
}

impl From<AutoCommit> for Step<Run> {
    fn from(value: AutoCommit) -> Self {
        let mut commands = Vec::new();

        // Configure Git user name
        let user_name = value.user_name.unwrap_or_else(|| "github-actions".to_string());
        commands.push(format!("git config --global user.name '{}'", user_name));

        // Configure Git user email
        let user_email = value.user_email.unwrap_or_else(|| "github-actions@github.com".to_string());
        commands.push(format!("git config --global user.email '{}'", user_email));

        // Add specific files if provided
        if let Some(files) = &value.files {
            if !files.is_empty() {
                let files_str = files.join(" ");
                commands.push(format!("git add {}", files_str));
            } else {
                commands.push("git add .".to_string());
            }
        } else {
            // If no specific files are provided, stage all changes
            commands.push("git add .".to_string());
        }

        // Add the commit command
        commands.push(format!("git commit -m \"{}\" || echo 'No changes to commit'", value.message));

        // Add push command if enabled
        if value.push.unwrap_or(false) {
            commands.push("git push".to_string());
        }

        // Combine all commands into a single shell script
        let full_command = commands.join(" && ");

        let mut step = Step::run(full_command);

        if let Some(id) = value.id {
            step = step.id(id);
        }

        if let Some(name) = value.name {
            step = step.name(name);
        }

        step
    }
}