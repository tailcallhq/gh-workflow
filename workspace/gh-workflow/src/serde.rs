use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use crate::{AnyStep, Run, Step, Use};

impl Serialize for AnyStep {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AnyStep::Run(step) => step.serialize(serializer),
            AnyStep::Use(step) => step.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for AnyStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the input as a generic map first
        let map = serde_yaml::Value::deserialize(deserializer)?;

        // Check if the map contains the `run` or `uses` fields
        if map.get("run").is_some() {
            // If `run` is defined, deserialize it as `Step<Run>`
            let step_run: Step<Run> = serde_yaml::from_value(map).map_err(de::Error::custom)?;
            Ok(AnyStep::Run(step_run))
        } else if map.get("uses").is_some() {
            // If `uses` is defined, deserialize it as `Step<Use>`
            let step_use: Step<Use> = serde_yaml::from_value(map).map_err(de::Error::custom)?;
            Ok(AnyStep::Use(step_use))
        } else {
            Err(de::Error::custom("Expected either `run` or `uses` field"))
        }
    }
}