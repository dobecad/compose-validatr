use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub file: Option<String>,
    pub external: Option<bool>,
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_configs() {
        let yaml = r#"
          file: "path/to/config1"
          external: true
          name: "named_config"
        "#;

        let configs: Configs = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(configs.file.unwrap(), "path/to/config1".to_string());
        assert_eq!(configs.external.unwrap(), true);
        assert_eq!(configs.name.unwrap(), "named_config".to_string());
    }
}
