use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Volumes {
    Short(ShortVolumeOptions),
    Long(LongVolumeOptions),
}

#[derive(Debug, Deserialize)]
pub struct ShortVolumeOptions {
    pub volume: String,
    pub container_path: String,
    pub access_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct LongVolumeOptions {
    #[serde(rename = "type")]
    pub volume_type: VolumeType,
    pub source: String,
    pub target: String,
    pub read_only: Option<bool>,
    pub bind: Option<Bind>,
    pub volume: Option<VolumeOptions>,
    pub tmpfs: Option<Tmpfs>,
    pub consistency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum VolumeType {
    Volume,
    Bind,
    Tmpfs,
    Npipe,
    Cluster,
}

#[derive(Debug, Deserialize)]
pub struct Bind {
    pub propagation: String,
    pub create_host_path: String,
    pub selinux: String,
}

#[derive(Debug, Deserialize)]
pub struct VolumeOptions {
    pub nocopy: bool,
}

#[derive(Debug, Deserialize)]
pub struct Tmpfs {
    pub size: String,
    pub mode: String,
}
