mod blkio_config;
mod build;
mod deploy;
mod healthcheck;
mod logging;
mod networks;
mod ports;
mod secrets;
mod volumes;

use std::collections::HashMap;

use serde::Deserialize;

use crate::{compose::Validate, errors::ValidationErrors};

#[derive(Debug, Deserialize)]
pub struct Service {
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
    pub cap_drop: Option<Vec<String>>,
    pub cgroup: Option<Cgroup>, // define cgroup enum (host, private)
    pub cgroup_parent: Option<String>,
    pub command: Option<Command>,
    pub configs: Option<Vec<String>>, // needs to exist in config top level
    pub container_name: Option<String>,
    pub credential_spec: Option<CredentialSpec>, // file, registry, config
    pub depends_on: Option<DependsOn>,           // existing service
    pub deploy: Option<deploy::Deploy>,
    pub device_cgroup_rules: Option<Vec<String>>,
    pub devices: Option<Vec<String>>,
    pub dns: Option<Labels>,
    pub dns_opt: Option<Vec<String>>,
    pub dns_search: Option<Labels>,
    pub domainname: Option<String>,
    pub entrypoint: Option<Labels>,
    pub env_file: Option<Labels>,
    pub environment: Option<Labels>,
    pub expose: Option<Vec<String>>,
    pub extends: Option<Extends>,
    pub annotations: Option<Labels>,
    pub external_links: Option<Vec<String>>,
    pub extra_hosts: Option<Labels>,
    pub group_add: Option<Vec<String>>,
    pub healthcheck: Option<healthcheck::HealthCheck>,
    pub hostname: Option<String>,
    pub image: Option<String>,
    pub init: Option<bool>,
    pub ipc: Option<String>,
    pub uts: Option<String>,
    pub isolation: Option<String>, // TODO: Verify this
    pub labels: Option<Labels>,
    pub links: Option<Vec<String>>,
    pub logging: Option<logging::Logging>,
    pub network_mode: Option<String>, // TODO: Maybe make enum for this?
    pub networks: Option<networks::Networks>,
    pub mac_address: Option<String>,
    pub mem_limit: Option<String>,       // deprecated
    pub mem_reservation: Option<String>, // deprecated
    pub mem_swappiness: Option<u8>,
    pub memswap_limit: Option<String>,
    pub oom_kill_disable: Option<bool>,
    pub oom_score_adj: Option<i16>,
    pub pid: Option<u32>,
    pub pids_limit: Option<u32>,
    pub platform: Option<String>,
    pub ports: Option<Vec<String>>,
    pub privileged: Option<bool>,
    pub profiles: Option<Vec<String>>,
    pub pull_policy: Option<PullPolicy>,
    pub read_only: Option<bool>,
    pub restart: Option<Restart>,
    pub runtime: Option<String>,
    pub scale: Option<u32>, // deprecated
    pub secrets: Option<secrets::Secrets>,
    pub secruity_opt: Option<Vec<String>>,
    pub shm_size: Option<String>,
    pub stdin_open: Option<String>,
    pub stop_grace_period: Option<String>,
    pub stop_signal: Option<String>,
    pub storage_opt: Option<String>,
    pub sysctls: Option<Labels>,
    pub tmpfs: Option<Tmpfs>,
    pub tty: Option<String>,
    pub ulimits: Option<Ulimits>,
    pub user: Option<String>,
    pub userns_mode: Option<String>,
    pub volumes: Option<Vec<volumes::Volumes>>, // enum
    pub volumes_from: Option<Vec<String>>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Labels {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Tmpfs {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cgroup {
    Host,
    Private,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Command {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct CredentialSpec {
    pub file: Option<String>,
    pub registry: Option<String>,
    pub config: Option<String>, // must be valid config
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
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

#[derive(Debug, Deserialize)]
pub struct Extends {
    // https://docs.docker.com/compose/compose-file/05-services/#extends
    pub file: String,
    pub service: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PullPolicy {
    Always,
    Never,
    Missing,
    Build,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Restart {
    No,
    Always,
    OnFailure,
    UnlessStopped,
}

#[derive(Debug, Deserialize)]
pub struct Ulimits {
    pub nproc: u16,
    pub nofile: Nofile,
}

#[derive(Debug, Deserialize)]
pub struct Nofile {
    pub soft: u16,
    pub hard: u16,
}

impl Validate for Service {
    fn validate(&self, errors: &mut ValidationErrors) {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
