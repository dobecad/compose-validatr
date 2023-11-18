//! Network fields and validation

use crate::compose::Compose;
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    compose::Validate,
    errors::{ValidationError, ValidationErrors},
    services::Labels,
};

/// Represents the top level [Network](https://docs.docker.com/compose/compose-file/06-networks/) element
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Network {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Ipam>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<Driver>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<Config>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ipam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<Driver>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Driver {
    None,
    Default,
    Host,
    Bridge,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aux_addresses: Option<HashMap<String, String>>,
}

impl Validate for Network {
    fn validate(&self, _: &Compose, errors: &mut ValidationErrors) {
        if let Some(config) = &self.config {
            for c in config {
                if let Some(subnet) = &c.subnet {
                    match subnet.parse::<IpNetwork>() {
                        Err(e) => errors.add_error(ValidationError::InvalidValue(format!(
                            "Invalid subnet address: {}",
                            e
                        ))),
                        _ => (),
                    }
                }
                if let Some(ip_range) = &c.ip_range {
                    match ip_range.parse::<IpNetwork>() {
                        Err(e) => errors.add_error(ValidationError::InvalidValue(format!(
                            "Invalid ip_range address: {}",
                            e
                        ))),
                        _ => (),
                    }
                }
                if let Some(gateway) = &c.gateway {
                    match gateway.parse::<IpNetwork>() {
                        Err(e) => errors.add_error(ValidationError::InvalidValue(format!(
                            "Invalid gateway address: {}",
                            e
                        ))),
                        _ => (),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_network() {
        let yaml = r#"
        ipam:
        driver: default
        config:
        - subnet: 172.28.0.0/16
          ip_range: 172.28.5.0/24
          gateway: 172.28.5.254
          aux_addresses:
            host1: 172.28.1.5
            host2: 172.28.1.6
            host3: 172.28.1.7
        options:
          foo: bar
          baz: "0"
        "#;

        let networks: Network = serde_yaml::from_str(yaml).unwrap();
        assert!(networks.ipam.is_none());
        assert_eq!(networks.driver.as_ref().unwrap(), &Driver::Default);
        assert_eq!(networks.config.as_ref().unwrap().len(), 1);
        assert_eq!(
            networks.config.as_ref().unwrap()[0]
                .aux_addresses
                .as_ref()
                .unwrap()
                .len(),
            3
        );
        assert_eq!(networks.options.unwrap().len(), 2);
    }

    #[test]
    fn test_invalid_ip() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
        networks:
          hello:
            ipam:
            driver: default
            config:
            - subnet: Invalid Ip
              ip_range: 172.28.5.0/24
              gateway: 172.28.5.254
              aux_addresses:
                host1: 172.28.1.5
                host2: 172.28.1.6
                host3: 172.28.1.7
            options:
              foo: bar
              baz: "0"
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_err());
        assert!(compose.is_err_and(|e| e.all_errors().len() == 1))
    }
}
