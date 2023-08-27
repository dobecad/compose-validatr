/// Top level elements:
/// services
/// network
/// volumes
/// configs
/// secrets

use serde::Deserialize;
use super::services;

pub struct Compose {
    version: String,
    services: services::Services,
    network: Network,
    volumes: Volumes,
    configs: Configs,
    secrets: Secrets,
}

pub struct Network {

}

pub struct Volumes {

}

pub struct Configs {

}

pub struct Secrets {

}

