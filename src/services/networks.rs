use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Networks {
    List(Vec<String>),
    Map(HashMap<String, NetworkOptions>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_ips: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,
}

impl Validate for Networks {
    fn validate(&self, ctx: &Compose, errors: &mut crate::errors::ValidationErrors) {
        match self {
            Networks::List(n) => {
                ctx.networks.as_ref().map(|available_networks| {
                    n.iter().for_each(|network| {
                        if !available_networks.contains_key(network) {
                            errors.add_error(crate::errors::ValidationError::InvalidValue(
                                format!("Service networks reference an unknown network: {network}"),
                            ));
                        }
                    });
                });
            }
            Networks::Map(n) => {
                ctx.networks.as_ref().map(|available_networks| {
                    n.keys().for_each(|network| {
                        if !available_networks.contains_key(network) {
                            errors.add_error(crate::errors::ValidationError::InvalidValue(
                                format!("Service networks reference an unknown network: {network}"),
                            ));
                        }
                    });
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_network() {
        let yaml = r#"
        services:
          some-service:
            networks:
              - hello
              - world
        networks:
            some-network:
            other-network:
        "#;
        let compose = Compose::new(yaml);
        assert!(compose.is_err());
    }

    #[test]
    fn valid_network() {
        let yaml = r#"
        services:
          some-service:
            networks:
              - some-network
              - other-network
        networks:
            some-network:
            other-network:
        "#;
        let compose = Compose::new(yaml);
        assert!(compose.is_ok());
    }
}
