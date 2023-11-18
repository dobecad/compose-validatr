//! Secret fields and validation

use crate::compose::Compose;
use serde::{Deserialize, Serialize};

use crate::{compose::Validate, errors::ValidationErrors};

/// Represents the top level [Secrets](https://docs.docker.com/compose/compose-file/09-secrets/) element
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Secret {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Validate for Secret {
    fn validate(&self, _: &Compose, _: &mut ValidationErrors) {
        // Nothing to validate
        // Not interested in validating the existence of files on host
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_secrets() {
        let yaml = r#"
        file: "path/to/secret1"
        environment: "ENV_VAR"
        external: true
        name: "named_secret"
        "#;

        let secrets: Secret = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(secrets.file.unwrap(), "path/to/secret1".to_string());
        assert_eq!(secrets.environment.unwrap(), "ENV_VAR".to_string());
        assert_eq!(secrets.external.unwrap(), true);
        assert_eq!(secrets.name.unwrap(), "named_secret".to_string());
    }
}
