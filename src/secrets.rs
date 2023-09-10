use serde::Deserialize;

use crate::{compose::Validate, errors::ValidationErrors};

#[derive(Debug, Deserialize)]
pub struct Secret {
    pub file: Option<String>,
    pub environment: Option<String>,
    pub external: Option<bool>,
    pub name: Option<String>,
}

impl Validate for Secret {
    fn validate(&self, _: &mut ValidationErrors) {
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
