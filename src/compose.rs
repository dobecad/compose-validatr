use super::{configs, network, secrets, services, volumes};
use serde::Deserialize;

pub struct Compose {
    version: String,
    services: services::Services,
    network: network::Network,
    volumes: volumes::Volumes,
    configs: configs::Configs,
    secrets: secrets::Secrets,
}
