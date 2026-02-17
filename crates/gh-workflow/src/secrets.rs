//!
//! Secrets for GitHub workflow secrets and security.

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents secrets required for the workflow or job or step.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Secrets {
    #[serde(
        deserialize_with = "deserialize_inherit",
        serialize_with = "serialize_inherit"
    )]
    Inherit,
    Values(IndexMap<String, String>),
}

fn deserialize_inherit<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s == "inherit" {
        Ok(())
    } else {
        Err(serde::de::Error::custom("expected string 'inherit'"))
    }
}

fn serialize_inherit<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("inherit")
}
