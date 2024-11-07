use std::path::Path;
use crate::error::Error;

pub fn validate<P: AsRef<Path>>(cur: &str, prev: P) -> Result<(), Error> {
    if let Ok(prev) = std::fs::read_to_string(prev) {
        if cur != prev && built::util::detect_ci().is_some() {
            return Err(Error::GitHubWorkflowMismatch);
        }
    }

    Ok(())
}