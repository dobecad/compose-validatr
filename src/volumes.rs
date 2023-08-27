use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Volumes {
    volumes: HashMap<String, Option<VolumeAttributes>>,
}

#[derive(Debug, Deserialize)]
pub struct VolumeAttributes {
    driver: Option<String>,
    driver_opts: Option<DriverOpts>,
    external: Option<String>,
    labels: Option<Vec<String>>, // could be hashmap string,string
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DriverOpts {
    #[serde(rename = "type")]
    driver_type: Option<String>,
    o: Option<String>,
    device: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_correct_volume() {
        let yaml = r#"
        volumes:
            db-data:
                external: true
        "#;
        let volumes: Volumes = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(volumes.volumes.len(), 1);
        assert!(volumes.volumes.get("db-data").is_some());
    }

    #[test]
    fn test_deserialize_multiple_volumes() {
        let yaml = r#"
            volumes:
              "vol1":
                driver: "driver1"
              "vol2":
                driver: "driver2"
                driver_opts:
                  type: "type1"
                  o: "option1"
              "vol3":
                external: "ext_vol"
                labels:
                  - "label1"
                  - "label2"
              "vol4":
                name: "named_vol"
        "#;

        let volumes: Volumes = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(volumes.volumes.len(), 4);
        assert_eq!(
            volumes.volumes["vol1"].as_ref().unwrap().driver,
            Some("driver1".to_string())
        );
        assert_eq!(
            volumes.volumes["vol2"].as_ref().unwrap().driver,
            Some("driver2".to_string())
        );
        assert_eq!(
            volumes.volumes["vol2"]
                .as_ref()
                .unwrap()
                .driver_opts
                .as_ref()
                .unwrap()
                .driver_type,
            Some("type1".to_string())
        );
        assert_eq!(
            volumes.volumes["vol2"]
                .as_ref()
                .unwrap()
                .driver_opts
                .as_ref()
                .unwrap()
                .o,
            Some("option1".to_string())
        );
        assert_eq!(
            volumes.volumes["vol3"].as_ref().unwrap().external,
            Some("ext_vol".to_string())
        );
        assert_eq!(
            volumes.volumes["vol3"]
                .as_ref()
                .unwrap()
                .labels
                .as_ref()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            volumes.volumes["vol4"].as_ref().unwrap().name,
            Some("named_vol".to_string())
        );
    }

    #[test]
    fn test_empty_volumes() {
        let yaml = r#"
            volumes:
              "vol1":
        "#;

        let volumes: Volumes = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(volumes.volumes.len(), 1);

        let yaml = r#"
            volumes:
        "#;

        let volumes: Volumes = serde_yaml::from_str(yaml).unwrap();
        assert!(volumes.volumes.is_empty());
    }

    #[test]
    fn test_invalid_volume() {
        let yaml = r#"
            volumes:
                hello:
                    world
        "#;

        let volumes: Result<Volumes, _> = serde_yaml::from_str(yaml);
        assert!(volumes.is_err());
    }
}
