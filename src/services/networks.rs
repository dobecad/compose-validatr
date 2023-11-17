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
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        ()
    }
}
