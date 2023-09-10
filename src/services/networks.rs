use std::collections::HashMap;

use serde::Deserialize;

use crate::compose::{Compose, Validate};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Networks {
    List(Vec<String>),
    Map(HashMap<String, NetworkOptions>),
}

#[derive(Debug, Deserialize)]
pub struct NetworkOptions {
    pub aliases: Option<Vec<String>>,
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub link_local_ips: Option<Vec<String>>,
    pub priority: Option<u16>,
}

impl Validate for Networks {
    fn validate(&self, ctx: &Compose, errors: &mut crate::errors::ValidationErrors) {
        todo!()
    }
}
