use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Network {
    network: HashMap<String, Option<NetworkAttributes>>, // String, network attributes
}

#[derive(Debug, Deserialize)]
pub struct NetworkAttributes {
    ipam: String,   // struct
    driver: String, // enum
    config: Vec<Config>,
    internal: bool,
    labels: Vec<String>, // or hashmap string,string
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    subnet: String,
    ip_range: String,
    gateway: String,
    aux_addresses: AuxAddresses,
}

#[derive(Debug, Deserialize)]
pub struct AuxAddresses {
    addresses: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
