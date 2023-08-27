use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Configs {
    configs: HashMap<String, Option<ConfigAttributes>>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigAttributes {
    file: Option<String>,
    external: Option<bool>,
    name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_configs() {
        let yaml = r#"
            configs:
              "config1":
                file: "path/to/config1"
              "config2":
                file: "path/to/config2"
              "config3":
                external: true
              "config4":
                name: "named_config"
        "#;

        let configs: Configs = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(configs.configs.len(), 4);
        assert_eq!(
            configs.configs["config1"].as_ref().unwrap().file,
            Some("path/to/config1".to_string())
        );
        assert_eq!(
            configs.configs["config2"].as_ref().unwrap().file,
            Some("path/to/config2".to_string())
        );
        assert_eq!(
            configs.configs["config3"].as_ref().unwrap().external,
            Some(true)
        );
        assert_eq!(
            configs.configs["config4"].as_ref().unwrap().name,
            Some("named_config".to_string())
        );
    }

    #[test]
    fn test_empty_configs() {
        let yaml = r#"
            configs:
                hello:
        "#;

        let configs: Configs = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(configs.configs.len(), 1);

        let yaml = r#"
            configs:
        "#;

        let configs: Configs = serde_yaml::from_str(yaml).unwrap();
        assert!(configs.configs.is_empty());
    }

    #[test]
    fn test_invalid_configs() {
        let yaml = r#"
            configs:
                hello:
                    world
        "#;

        let configs: Result<Configs, _> = serde_yaml::from_str(yaml);
        assert!(configs.is_err());
    }
}
