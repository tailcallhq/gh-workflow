#[derive(Debug, derive_more::From)]
pub enum Error {
    IO(std::io::Error),
    Yaml(serde_yaml::Error),
    GitHubWorkflowMismatch,
    JobIdAlreadyExists(String),
    UTF8(std::string::FromUtf8Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
