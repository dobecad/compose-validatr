//! Service fields and validation

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

/// Represents the top level [Service](https://docs.docker.com/compose/compose-file/05-services/) element
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<build::Build>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_config: Option<blkio_config::BlkioConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_rt_runtime: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_rt_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<f32>, // deprecated

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<Capabilities>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<Capabilities>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<Cgroup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<Config>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_spec: Option<CredentialSpec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<DependsOn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<deploy::Deploy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Labels>, // TODO: maybe validate as ipv4

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_opt: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_file: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<Extends>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_add: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<healthcheck::HealthCheck>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<String>, // TODO: Verify this

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<logging::Logging>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>, // TODO: Maybe make enum for this?

    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<networks::Networks>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_limit: Option<String>, // deprecated

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_reservation: Option<String>, // deprecated

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_swappiness: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memswap_limit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<ports::Ports>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<PullPolicy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<Restart>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>, // deprecated

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<secrets::Secret>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secruity_opt: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_open: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_opt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Tmpfs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Ulimits>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<volumes::Volumes>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Labels {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Tmpfs {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Config {
    Short(String),
    Long(ConfigDetails),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigDetails {
    pub source: String,
    pub target: String,
    pub uid: String,
    pub gid: String,
    pub mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Cgroup {
    Host,
    Private,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Command {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>, // must be valid config
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DependsOn {
    List(Vec<String>), // must be valid services
    Map(HashMap<String, DependsOnDetail>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DependsOnDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<DependsOnCondition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependsOnCondition {
    ServiceStarted,
    ServiceHealthy,
    ServiceCompletedSuccessfully,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Extends {
    // https://docs.docker.com/compose/compose-file/05-services/#extends
    pub file: String,
    pub service: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PullPolicy {
    Always,
    Never,
    Missing,
    Build,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Restart {
    No,
    Always,
    OnFailure,
    UnlessStopped,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ulimits {
    pub nproc: u16,
    pub nofile: Nofile,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Nofile {
    pub soft: u16,
    pub hard: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
        self.secrets
            .as_ref()
            .map(|s| s.iter().for_each(|s| s.validate(ctx, errors)));
    }

    fn validate_volumes(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        self.volumes
            .as_ref()
            .map(|v| v.iter().for_each(|volume| volume.validate(ctx, errors)));
    }

    fn validate_configs(&self, ctx: &Compose, errors: &mut ValidationErrors) {
        // configs must exist in top level configs

        self.configs.as_ref().map(|c| {
            c.iter().for_each(|config| match config {
                Config::Short(c) => {
                    ctx.configs.as_ref().map(|configs| {
                        if !configs.contains_key(c) {
                            errors.add_error(ValidationError::InvalidValue(format!(
                                "Config is not defined: {}",
                                c
                            )))
                        }
                    });
                }
                Config::Long(c) => {
                    ctx.configs.as_ref().map(|configs| {
                        if !configs.contains_key(&c.source) {
                            errors.add_error(ValidationError::InvalidValue(format!(
                                "Config is not defined: {}",
                                &c.source
                            )))
                        }
                    });
                }
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

    fn validate_container_name(&self, _: &Compose, errors: &mut ValidationErrors) {
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
                        "Invalid service for depends_on".to_string(),
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
                service.depends_on.as_ref().map(|_| {
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
                            "Extends cannot have any volumes_from".to_string(),
                        ))
                    }
                });
                service.ipc.as_ref().map(|_| {
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

    #[test]
    fn depends_on_missing_service() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            depends_on:
              - hello_world
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_err());
        assert!(compose.is_err_and(|e| e.all_errors().len() == 1));
    }

    #[test]
    fn depends_on_valid_service() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            depends_on:
              - hello_world
          hello_world:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_ok());
    }

    #[test]
    fn valid_config() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            configs:
              - my_config
              - my_other_config
        configs:
          my_config:
            file: ./my_config.txt  
          my_other_config:
            external: true
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_ok());
    }

    #[test]
    fn invalid_config() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            configs:
              - hello
              - world
        configs:
          my_config:
            file: ./my_config.txt  
          my_other_config:
            external: true
        "#;

        let compose = Compose::new(yaml);
        dbg!(&compose);
        assert!(compose.is_err());
    }
}
