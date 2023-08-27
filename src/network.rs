use std::collections::HashMap;

pub struct Network {
    network: HashMap<String, NetworkAttributes>, // String, network attributes
}

pub struct NetworkAttributes {
    ipam: String,   // struct
    driver: String, // enum
    config: Vec<Config>,
    internal: bool,
    labels: Vec<String>, // or hashmap string,string
    name: String,
}

pub struct Config {
    subnet: String,
    ip_range: String,
    gateway: String,
    aux_addresses: AuxAddresses,
}

pub struct AuxAddresses {
    addresses: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
