use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    secrets: HashMap<String, Option<SecretAttributes>>,
}

#[derive(Debug, Deserialize)]
pub struct SecretAttributes {
    file: Option<String>,
    environment: Option<String>,
    external: Option<bool>,
    name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_secrets() {
        let yaml = r#"
            secrets:
              "secret1":
                file: "path/to/secret1"
              "secret2":
                file: "path/to/secret2"
                environment: "ENV_VAR"
              "secret3":
                external: true
              "secret4":
                name: "named_secret"
        "#;

        let secrets: Secrets = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(secrets.secrets.len(), 4);
        assert_eq!(
            secrets.secrets["secret1"].as_ref().unwrap().file,
            Some("path/to/secret1".to_string())
        );
        assert_eq!(
            secrets.secrets["secret2"].as_ref().unwrap().file,
            Some("path/to/secret2".to_string())
        );
        assert_eq!(
            secrets.secrets["secret2"].as_ref().unwrap().environment,
            Some("ENV_VAR".to_string())
        );
        assert_eq!(
            secrets.secrets["secret3"].as_ref().unwrap().external,
            Some(true)
        );
        assert_eq!(
            secrets.secrets["secret4"].as_ref().unwrap().name,
            Some("named_secret".to_string())
        );
    }

    #[test]
    fn test_empty_secrets() {
        let yaml = r#"
            secrets:
                hello:
        "#;

        let secrets: Secrets = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(secrets.secrets.len(), 1);

        let yaml = r#"
            secrets:
        "#;

        let secrets: Secrets = serde_yaml::from_str(yaml).unwrap();
        assert!(secrets.secrets.is_empty());
    }

    #[test]
    fn test_invalid_secrets() {
        let yaml = r#"
            secrets:
                hello:
                    world
        "#;

        let secrets: Result<Secrets, _> = serde_yaml::from_str(yaml);
        assert!(secrets.is_err());
    }
}
