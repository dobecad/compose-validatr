use crate::compose::{Compose, Validate};

use super::Labels;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deploy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enpoint_mode: Option<EndpointMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<RestartPolicy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_config: Option<RollbackConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<UpdateConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Placement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Labels>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EndpointMode {
    Vip,
    Dnsrr,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Global,
    Replicated,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limits>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Reservations>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DriverCapabilities>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<DriverCapabilities>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DriverCapabilities {
    Gpu,
    Tpu,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestartPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<RestartCondition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartCondition {
    None,
    OnFailure,
    Any,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reservations {
    pub cpus: String,
    pub memory: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollbackConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<FailureAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_ration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<FailureAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_ration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Order {
    StopFirst,
    StartFirst,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FailureAction {
    Continue,
    Pause,
}

impl Validate for Deploy {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        // Nothing to really validate here
        ()
    }
}
