use super::{configs, network, secrets, services, volumes};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Compose {
    pub version: Option<String>,
    pub services: services::Services,
    pub networks: Option<network::Networks>,
    pub volumes: Option<volumes::Volumes>,
    pub configs: Option<configs::Configs>,
    pub secrets: Option<secrets::Secrets>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_compose() {
        todo!()
    }
}
