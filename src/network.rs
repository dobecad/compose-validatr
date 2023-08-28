use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Networks {
    networks: HashMap<String, Option<NetworkAttributes>>,
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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Driver {
    None,
    Default,
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
    pub aux_addresses: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn test_big_network() {
        let yaml = r#"
        networks:
          mynet1:
            ipam:
            driver: default
            config:
            - subnet: 172.28.0.0/16
              ip_range: 172.28.5.0/24
              gateway: 172.28.5.254
              aux_addresses:
                host1: 172.28.1.5
                host2: 172.28.1.6
                host3: 172.28.1.7
            options:
              foo: bar
              baz: "0"
        "#;

        let networks: Networks = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(networks.networks.len(), 1);
        assert_eq!(
            networks.networks["mynet1"].as_ref().unwrap().driver.as_ref().unwrap(),
            &Driver::Default
        );
        assert_eq!(networks.networks["mynet1"].as_ref().unwrap().config.as_ref().unwrap().len(), 1);
    }
}
