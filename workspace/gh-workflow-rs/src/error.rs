#[derive(Debug, derive_more::From)]
pub enum Error {
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    GitHubWorkflowMismatch,
    JobIdAlreadyExists(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
