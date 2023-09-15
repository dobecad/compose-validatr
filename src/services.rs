mod blkio_config;
mod build;
mod deploy;
mod healthcheck;
mod logging;
mod networks;
mod ports;
mod secrets;
mod volumes;

use crate::{compose::Compose, errors::ValidationError};
use regex::Regex;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    pub cap_add: Option<Vec<Capabilities>>,
    pub cap_drop: Option<Vec<Capabilities>>,
    pub cgroup: Option<Cgroup>,
    pub cgroup_parent: Option<String>,
    pub command: Option<Command>,
    pub configs: Option<Vec<Config>>,
    pub container_name: Option<String>,
    pub credential_spec: Option<CredentialSpec>,
    pub depends_on: Option<DependsOn>,
    pub deploy: Option<deploy::Deploy>,
    pub device_cgroup_rules: Option<Vec<String>>,
    pub devices: Option<Vec<String>>,
    pub dns: Option<Labels>, // TODO: maybe validate as ipv4
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
    pub ports: Option<ports::Ports>,
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
#[serde(untagged)]
pub enum Config {
    Short(String),
    Long(ConfigDetails),
}

#[derive(Debug, Deserialize)]
pub struct ConfigDetails {
    pub source: String,
    pub target: String,
    pub uid: String,
    pub gid: String,
    pub mode: String,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Capabilities {
    All,
    AuditControl,
    AuditRead,
    AuditWrite,
    BlockSuspend,
    Bpf,
    CheckpointRestore,
    Chown,
    DacOverride,
    DacReadSearch,
    Fowner,
    Fsetid,
    IpcLock,
    IpcOwner,
    Kill,
    Lease,
    LinuxImmutable,
    MacAdmin,
    MacOverride,
    Mknod,
    NetAdmin,
    NetBindService,
    NetBroadcast,
    NetRaw,
    Perfmon,
    Setgid,
    Setfcap,
    Setpcap,
    Setuid,
    SysAdmin,
    SysBoot,
    SysChroot,
    SysModule,
    SysNice,
    SysPacct,
    SysPtrace,
    SysRawio,
    SysResource,
    SysTime,
    SysTtyConfig,
    Syslog,
    WakeAlarm,
}

impl Service {
    fn validate_blkio_config(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.blkio_config.as_ref().map(|b| b.validate(ctx, errors));
    }

    fn validate_build(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.build.as_ref().map(|b| b.validate(ctx, errors));
    }

    fn validate_deploy(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.deploy.as_ref().map(|d| d.validate(ctx, errors));
    }

    fn validate_healthcheck(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.healthcheck.as_ref().map(|h| h.validate(ctx, errors));
    }

    fn validate_logging(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.logging.as_ref().map(|l| l.validate(ctx, errors));
    }

    fn validate_networks(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.networks.as_ref().map(|n| n.validate(ctx, errors));
    }

    fn validate_ports(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.ports.as_ref().map(|p| p.validate(ctx, errors));
    }

    fn validate_secrets(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.secrets.as_ref().map(|s| s.validate(ctx, errors));
    }

    fn validate_volumes(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.volumes
            .as_ref()
            .map(|v| v.iter().for_each(|volume| volume.validate(ctx, errors)));
    }

    fn validate_configs(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        // configs must exist in top level configs

        self.configs.as_ref().map(|c| {
            c.iter().all(|config| match config {
                Config::Short(c) => ctx
                    .configs
                    .as_ref()
                    .map(|configs| configs.contains_key(c))
                    .is_some(),
                Config::Long(c) => ctx
                    .configs
                    .as_ref()
                    .map(|configs| configs.contains_key(&c.source))
                    .is_some(),
            })
        });
    }

    fn validate_credential_spec(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.credential_spec.as_ref().map(|c| {
            let result = c.config.as_ref().map(|cred| {
                ctx.configs
                    .as_ref()
                    .map(|configs| configs.contains_key(cred))
                    .is_some()
            });
            result.map(|r| {
                if !r {
                    errors.add_error(ValidationError::InvalidValue(
                        "Credential Spec references unknown config".to_string(),
                    ))
                }
            });
        });
    }

    fn validate_container_name(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_.-]+$").unwrap();
        self.container_name.as_ref().map(|c| {
            if !re.is_match(c) {
                errors.add_error(ValidationError::InvalidValue(
                    "Invalid container name".to_string(),
                ));
            }
        });
    }

    fn validate_depends_on(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.depends_on.as_ref().map(|d| match d {
            DependsOn::List(service) => {
                let result = service.iter().any(|s| ctx.services.contains_key(s));
                if !result {
                    errors.add_error(ValidationError::InvalidValue(
                        "Invalid service for depends_on".to_string(),
                    ));
                }
            }
            DependsOn::Map(service) => {
                let result = service.iter().any(|s| ctx.services.contains_key(s.0));
                if !result {
                    errors.add_error(ValidationError::InvalidValue(
                        "Invalid service for depnds_on".to_string(),
                    ));
                }
            }
        });
    }

    fn validate_expose(&self, _: &Compose, errors: &mut ValidationErrors) {
        self.expose.as_ref().map(|e| {
            e.iter().all(|port| match port.parse::<u16>() {
                Err(_) => {
                    errors.add_error(ValidationError::InvalidValue("Invalid port".to_string()));
                    false
                }
                _ => true,
            })
        });
    }

    fn validate_extends(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.extends.as_ref().map(|e| {
            let result = ctx.services.contains_key(&e.service);
            if result {
                // Services that have dependencies on other services cannot be used as a base.
                // Therefore, any key that introduces a dependency on another service is incompatible
                // with extends. The non-exhaustive list of such keys is: links, volumes_from, container
                // mode (in ipc, pid, network_mode and net), service mode (in ipc, pid and network_mode), depends_on.
                let service = ctx.services.get(&e.service).unwrap();
                service.depends_on.as_ref().map(|d| {
                    errors.add_error(ValidationError::InvalidValue(
                        "Extends cannot extend another service that has a depends_on".to_string(),
                    ))
                });
                service.links.as_ref().map(|l| {
                    if l.len() > 0 {
                        errors.add_error(ValidationError::InvalidValue(
                            "Extends cannot have any links".to_string(),
                        ))
                    }
                });
                service.volumes_from.as_ref().map(|v| {
                    if v.len() > 0 {
                        errors.add_error(ValidationError::InvalidValue(
                            "Extends canot have any volumes_from".to_string(),
                        ))
                    }
                });
                service.ipc.as_ref().map(|i| {
                    errors.add_error(ValidationError::InvalidValue(
                        "Extends cannot have an IPC mode".to_string(),
                    ))
                });
                service.network_mode.as_ref().map(|n| {
                    if n.starts_with("service:") {
                        errors.add_error(ValidationError::InvalidValue(
                            "Extends cannot extend a service that has a network dependency"
                                .to_string(),
                        ))
                    }
                });
            } else {
                errors.add_error(ValidationError::InvalidValue(
                    "Extends references invalid service".to_string(),
                ));
            }
        });
    }
}

impl Validate for Service {
    fn validate(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.validate_blkio_config(ctx, errors);
        self.validate_build(ctx, errors);
        self.validate_deploy(ctx, errors);
        self.validate_healthcheck(ctx, errors);
        self.validate_logging(ctx, errors);
        self.validate_networks(ctx, errors);
        self.validate_ports(ctx, errors);
        self.validate_secrets(ctx, errors);
        self.validate_volumes(ctx, errors);
        self.validate_configs(ctx, errors);
        self.validate_container_name(ctx, errors);
        self.validate_credential_spec(ctx, errors);
        self.validate_depends_on(ctx, errors);
        self.validate_expose(ctx, errors);
        self.validate_extends(ctx, errors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
