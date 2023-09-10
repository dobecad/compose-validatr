use serde::Deserialize;

use crate::compose::{Compose, Validate};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Ports {
    Short(Vec<String>),
    Long(Vec<PortOptions>),
}

#[derive(Debug, Deserialize)]
pub struct PortOptions {
    pub target: u16,
    pub host_ip: String,
    pub published: String,
    pub protocol: Protocol,
    pub mode: Mode,
}

#[derive(Debug, Deserialize)]
pub enum Protocol {
    Udp,
    Tcp,
}

#[derive(Debug, Deserialize)]
pub enum Mode {
    Host,
    Ingress,
}

impl Validate for Ports {
    fn validate(&self, ctx: &Compose, errors: &mut crate::errors::ValidationErrors) {
        todo!()
    }
}
