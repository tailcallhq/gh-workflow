use std::fmt::{Display, Formatter};

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
