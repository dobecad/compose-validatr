use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Ports {
    Short(Vec<String>),
    Long(Vec<PortOptions>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PortOptions {
    pub target: u16,
    pub host_ip: String,
    pub published: String,
    pub protocol: Protocol,
    pub mode: Mode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Protocol {
    Udp,
    Tcp,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Mode {
    Host,
    Ingress,
}

impl Validate for Ports {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        ()
    }
}
