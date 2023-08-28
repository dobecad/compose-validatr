use super::{configs, network, secrets, services, volumes};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Compose {
    pub version: String,
    pub services: services::Services,
    pub networks: network::Networks,
    pub volumes: volumes::Volumes,
    pub configs: configs::Configs,
    pub secrets: secrets::Secrets,
}

#[cfg(test)]
mod tests {
    use super::*;
}
