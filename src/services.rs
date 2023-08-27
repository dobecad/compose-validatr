use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Services {
    pub attach: bool,
    pub build: String,
    pub blkio_config: String,
    pub cpu_count: u8,
    pub cpu_percent: f32,
    pub cpu_shares: u32,
    pub cpu_period: String,
    pub cpu_quota: String,
    pub cpu_rt_runtime: u32,
    pub cpus: f32, // deprecated
    pub cpuset: u8,
    pub cap_add: Vec<String>, // define capabilities enum
    pub cap_drop: Vec<String>,
    pub cgroup: String, // define cgroup enum (host, private)
    pub cgroup_parent: String,
    pub command: String,      // could also be Vec<String>
    pub configs: Vec<String>, // needs to exist in config top level
    pub container_name: String,
    pub credential_spec: String, // file, registry, config
    pub depends_on: String,      // existing service
    pub deploy: String,
    pub device_cgroup_rules: Vec<String>,
    pub devices: Vec<String>,
    pub dns: Vec<String>, // could also just be a string
    pub dns_opt: Vec<String>,
    pub dns_search: Vec<String>, // could also be a string
    pub domainname: String,
    pub entrypoint: Vec<String>,
    pub env_file: Vec<String>,
    pub environment: Vec<String>,
    pub expose: Vec<String>,
    pub extends: String, // file, service enum
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

#[cfg(test)]
mod tests {
    use super::*;
}
