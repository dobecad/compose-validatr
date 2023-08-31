use serde::Deserialize;

use crate::services::Labels;

#[derive(Debug, Deserialize)]
pub struct Volumes {
    pub driver: Option<String>,
    pub driver_opts: Option<DriverOpts>,
    pub external: Option<bool>,
    pub labels: Option<Labels>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DriverOpts {
    #[serde(rename = "type")]
    pub driver_type: Option<String>,
    pub o: Option<String>,
    pub device: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_correct_volume() {
        let yaml = r#"
        db-data:
            external: true
        "#;
        let volumes: Volumes = serde_yaml::from_str(yaml).unwrap();
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

        let volumes: Volumes = serde_yaml::from_str(yaml).unwrap();

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
