//! Volume fields and validation

use serde::{Deserialize, Serialize};

use crate::compose::Compose;
use crate::compose::Validate;
use crate::errors::ValidationErrors;
use crate::services::Labels;

/// Represents the top level [Volume](https://docs.docker.com/compose/compose-file/07-volumes/) element
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<DriverOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriverOpts {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub driver_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

impl Validate for Volume {
    fn validate(&self, _: &Compose, _: &mut ValidationErrors) {
        // Nothing to really validate here
        // Not too interested in validating the existence of drivers on the host
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_correct_volume() {
        let yaml = r#"
            external: true
        "#;
        let volumes: Volume = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(volumes.external.unwrap(), true);
    }

    #[test]
    fn test_deserialize_big_volumes() {
        let yaml = r#"
            driver: "driver2"
            driver_opts:
                type: "type1"
                o: "option1"
            external: true
            labels:
                - "label1"
                - "label2"
        "#;

        let volumes: Volume = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(volumes.driver.unwrap(), "driver2".to_string());
        assert_eq!(
            volumes.driver_opts.as_ref().unwrap().driver_type,
            Some("type1".to_string())
        );
        assert_eq!(
            volumes.driver_opts.as_ref().unwrap().o,
            Some("option1".to_string())
        );
        assert_eq!(volumes.external.unwrap(), true);
        if let Some(labels) = &volumes.labels {
            match labels {
                Labels::List(list) => {
                    assert_eq!(list, &vec!["label1".to_string(), "label2".to_string()]);
                    assert_eq!(list.len(), 2);
                }
                _ => panic!("Unexpected labels format"),
            }
        } else {
            panic!("No labels found");
        }
    }
}
