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
    pub aliases: Option<Vec<String>>,
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub link_local_ips: Option<Vec<String>>,
    pub priority: Option<u16>,
}

impl Validate for Networks {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        todo!()
    }
}
