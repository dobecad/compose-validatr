use serde::Deserialize;

pub struct Services {
    attach: bool,
    build: String,
    blkio_config: String,
    cpu_count: u8,
    cpu_percent: f32,
    cpu_shares: u32,
    cpu_period: String,
    cpu_quota: String,
    cpu_rt_runtime: u32,
    cpus: f32, // deprecated
    cpuset: u8,
    cap_add: Vec<String>, // define capabilities enum
    cap_drop: Vec<String>,
    cgroup: String, // define cgroup enum (host, private)
    cgroup_parent: String,
    command: String,      // could also be Vec<String>
    configs: Vec<String>, // needs to exist in config top level
    container_name: String,
    credential_spec: String, // file, registry, config
    depends_on: String,      // existing service
    deploy: String,
    device_cgroup_rules: Vec<String>,
    devices: Vec<String>,
    dns: Vec<String>, // could also just be a string
    dns_opt: Vec<String>,
    dns_search: Vec<String>, // could also be a string
    domainname: String,
    entrypoint: Vec<String>,
    env_file: Vec<String>,
    environment: Vec<String>,
    expose: Vec<String>,
    extends: String, // file, service enum
    annotations: Vec<String>,
    external_links: Vec<String>,
    extra_hosts: Vec<String>,
    group_add: Vec<String>,
    healthcheck: String, // need struct
    hostname: String,
    image: String,
    init: bool,
    ipc: String,
    uts: String,
    isolation: String,
    labels: Vec<String>,
    links: Vec<String>,
    logging: String, // need struct
    network_mode: String,
    networks: Vec<String>,
    mac_address: String,
    mem_limit: String,       // deprecated
    mem_reservation: String, // deprecated
    mem_swappiness: u8,
    memswap_limit: String,
    oom_kill_disable: bool,
    oom_score_adj: i16,
    pid: u32,
    pids_limit: u32,
    platform: String,
    ports: Vec<String>,
    privileged: bool,
    profiles: Vec<String>,
    pull_policy: String, // enum
    read_only: bool,
    restart: String, // enum
    runtime: String,
    scale: u32,      // deprecated
    secrets: String, // valid secrets
    secruity_opt: Vec<String>,
    shm_size: u32,
    stdin_open: String,
    stop_grace_period: String,
    stop_signal: String,
    storage_opt: String, // enum
    sysctls: Vec<String>,
    tmpfs: String,
    tty: String,
    ulimits: String, // enum
    user: String,
    userns_mode: String,
    volumes: Vec<String>, // enum
    volumes_from: Vec<String>,
    working_dir: String,
}

#[cfg(test)]
mod tests {
    use super::*;
}
