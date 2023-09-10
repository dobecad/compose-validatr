use crate::compose::{Compose, Validate};

use super::Labels;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Deploy {
    pub replicas: Option<u16>,
    pub enpoint_mode: Option<EndpointMode>,
    pub labels: Option<Labels>,
    pub mode: Option<Mode>,
    pub placement: Option<Placement>,
    pub resources: Option<Resources>,
    pub restart_policy: Option<RestartPolicy>,
    pub rollback_config: Option<RollbackConfig>,
    pub update_config: Option<UpdateConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Placement {
    pub constraints: Option<Labels>,
    pub preferences: Option<Labels>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EndpointMode {
    Vip,
    Dnsrr,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Global,
    Replicated,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    pub limits: Option<Limits>,
    pub reservations: Option<Reservations>,
}

#[derive(Debug, Deserialize)]
pub struct Limits {
    pub cpus: Option<String>,
    pub memory: Option<String>,
    pub pids: Option<u16>,
    pub devices: Option<Vec<DriverCapabilities>>,
    pub capabilities: Option<Vec<DriverCapabilities>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DriverCapabilities {
    Gpu,
    Tpu,
}

#[derive(Debug, Deserialize)]
pub struct RestartPolicy {
    pub condition: Option<RestartCondition>,
    pub delay: Option<String>,
    pub max_attempts: Option<u16>,
    pub window: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartCondition {
    None,
    OnFailure,
    Any,
}

#[derive(Debug, Deserialize)]
pub struct Reservations {
    pub cpus: String,
    pub memory: String,
}

#[derive(Debug, Deserialize)]
pub struct RollbackConfig {
    pub parallelism: Option<u8>,
    pub delay: Option<String>,
    pub failure_action: Option<FailureAction>,
    pub monitor: Option<String>,
    pub max_failure_ration: Option<String>,
    pub order: Option<String>, // this should be an enum
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfig {
    pub parallelism: Option<u8>,
    pub delay: Option<String>,
    pub failure_action: Option<FailureAction>,
    pub monitor: Option<String>,
    pub max_failure_ration: Option<String>,
    pub order: Option<String>, // this should be an enum
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FailureAction {
    Continue,
    Pause,
}

impl Validate for Deploy {
    fn validate(&self, ctx: &Compose, errors: &mut crate::errors::ValidationErrors) {
        todo!()
    }
}
