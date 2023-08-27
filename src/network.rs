use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Network {
    network: HashMap<String, Option<NetworkAttributes>>, // String, network attributes
}

#[derive(Debug, Deserialize)]
pub struct NetworkAttributes {
    pub ipam: String,   // struct
    pub driver: String, // enum
    pub config: Vec<Config>,
    pub internal: bool,
    pub labels: Vec<String>, // or hashmap string,string
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub subnet: String,
    pub ip_range: String,
    pub gateway: String,
    pub aux_addresses: AuxAddresses,
}

#[derive(Debug, Deserialize)]
pub struct AuxAddresses {
    pub addresses: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
