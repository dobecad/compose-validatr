use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Network {
    network: Option<HashMap<String, Option<NetworkAttributes>>>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkAttributes {
    pub attachable: Option<bool>,
    pub ipam: Option<Ipam>,
    pub driver: Option<Driver>,
    pub driver_opts: Option<HashMap<String, String>>,
    pub enable_ipv6: Option<bool>,
    pub external: Option<bool>,
    pub config: Option<Vec<Config>>,
    pub internal: Option<bool>,
    pub labels: Option<Labels>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Ipam {
    pub driver: Option<Driver>,
    pub config: Option<Config>,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Driver {
    None,
    Host,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Labels {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub subnet: Option<String>,
    pub ip_range: Option<String>,
    pub gateway: Option<String>,
    pub aux_addresses: Option<AuxAddresses>,
}

#[derive(Debug, Deserialize)]
pub struct AuxAddresses {
    pub addresses: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
