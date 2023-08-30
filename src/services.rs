mod build;
mod blkio_config;
mod deploy;

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Services {
    pub attach: Option<bool>,
    pub build: Option<build::Build>,
    pub blkio_config: Option<blkio_config::BlkioConfig>,
    pub cpu_count: Option<u8>,
    pub cpu_percent: Option<f32>,
    pub cpu_shares: Option<u32>,
    pub cpu_period: Option<String>,
    pub cpu_quota: Option<String>,
    pub cpu_rt_runtime: Option<String>,
    pub cpu_rt_period: Option<String>,
    pub cpus: Option<f32>, // deprecated
    pub cpuset: Option<u8>,
    pub cap_add: Option<Vec<String>>, // TODO: define capabilities enum
    pub cap_drop: Vec<String>,
    pub cgroup: Option<Cgroup>, // define cgroup enum (host, private)
    pub cgroup_parent: Option<String>,
    pub command: Option<Command>,
    pub configs: Option<Vec<String>>, // needs to exist in config top level
    pub container_name: Option<String>,
    pub credential_spec: Option<CredentialSpec>, // file, registry, config
    pub depends_on: Option<DependsOn>,      // existing service
    pub deploy: Option<deploy::Deploy>,
    pub device_cgroup_rules: Option<Vec<String>>,
    pub devices: Option<Vec<String>>,
    pub dns: Option<Vec<Labels>>,
    pub dns_opt: Option<Vec<String>>,
    pub dns_search: Option<Vec<Labels>>,
    pub domainname: Option<String>,
    pub entrypoint: Option<Vec<Labels>>,
    pub env_file: Option<Vec<Labels>>,
    pub environment: Option<Vec<Labels>>,
    pub expose: Option<Vec<String>>,
    pub extends: String, // file, service struct
    pub annotations: Vec<String>,
    pub external_links: Vec<String>,
    pub extra_hosts: Vec<String>,
    pub group_add: Vec<String>,
    pub healthcheck: String, // need struct
    pub hostname: String,
    pub image: String,
    pub init: bool,
    pub ipc: String,
    pub uts: String,
    pub isolation: String,
    pub labels: Vec<String>,
    pub links: Vec<String>,
    pub logging: String, // need struct
    pub network_mode: String,
    pub networks: Vec<String>,
    pub mac_address: String,
    pub mem_limit: String,       // deprecated
    pub mem_reservation: String, // deprecated
    pub mem_swappiness: u8,
    pub memswap_limit: String,
    pub oom_kill_disable: bool,
    pub oom_score_adj: i16,
    pub pid: u32,
    pub pids_limit: u32,
    pub platform: String,
    pub ports: Vec<String>,
    pub privileged: bool,
    pub profiles: Vec<String>,
    pub pull_policy: String, // enum
    pub read_only: bool,
    pub restart: String, // enum
    pub runtime: String,
    pub scale: u32,      // deprecated
    pub secrets: String, // valid secrets
    pub secruity_opt: Vec<String>,
    pub shm_size: u32,
    pub stdin_open: String,
    pub stop_grace_period: String,
    pub stop_signal: String,
    pub storage_opt: String, // enum
    pub sysctls: Vec<String>,
    pub tmpfs: String,
    pub tty: String,
    pub ulimits: String, // enum
    pub user: String,
    pub userns_mode: String,
    pub volumes: Vec<String>, // enum
    pub volumes_from: Vec<String>,
    pub working_dir: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Labels {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cgroup {
    Host,
    Private,
}

#[derive(Debug, Deserialize)]
pub enum Command {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct CredentialSpec {
    pub file: Option<String>,
    pub registry: Option<String>,
    pub config: Option<String>,  // must be valid config
}

#[derive(Debug, Deserialize)]
pub enum DependsOn {
    List(Vec<String>), // must be valid services
    Map(HashMap<String, DependsOnDetail>),
}

#[derive(Debug, Deserialize)]
pub struct DependsOnDetail {
    pub restart: Option<bool>,
    pub condition: Option<DependsOnCondition>,
    pub required: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependsOnCondition {
    ServiceStarted,
    ServiceHealthy,
    ServiceCompletedSuccessfully,
}

#[cfg(test)]
mod tests {
    use super::*;
}
