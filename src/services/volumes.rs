use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Volumes {
    String(String),
    Short(ShortVolumeOptions),
    Long(LongVolumeOptions),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShortVolumeOptions {
    pub volume: String,
    pub container_path: String,
    pub access_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LongVolumeOptions {
    #[serde(rename = "type")]
    pub volume_type: VolumeType,
    pub source: String,
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<Bind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Tmpfs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VolumeType {
    Volume,
    Bind,
    Tmpfs,
    Npipe,
    Cluster,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bind {
    pub propagation: String,
    pub create_host_path: String,
    pub selinux: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VolumeOptions {
    pub nocopy: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tmpfs {
    pub size: String,
    pub mode: String,
}

impl Validate for Volumes {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        ()
    }
}
