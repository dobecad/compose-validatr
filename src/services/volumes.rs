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
#[serde(rename_all = "lowercase")]
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
    /// Note: Currently, this implementation of volumes always assumes references to volumes will
    /// be named volumes from the top level volumes section. However, the official docs say that
    /// the services:volumes should also allow for path references. Since this library does not
    /// care about the host system, maybe there should be additional options to check for host
    /// references.
    fn validate(&self, _ctx: &Compose, _errors: &mut crate::errors::ValidationErrors) {
        // match self {
        //     Volumes::String(s) => {
        //         ctx.volumes.as_ref().map(|available_volumes| {
        //             if !available_volumes.contains_key(s) {
        //                 errors.add_error(crate::errors::ValidationError::InvalidValue(format!(
        //                     "Service volumes reference unknown volume: {s}"
        //                 )));
        //             }
        //         });
        //     }
        //     Volumes::Short(s) => {
        //         ctx.volumes.as_ref().map(|available_volumes| {
        //             if !available_volumes.contains_key(s.volume.as_str()) {
        //                 errors.add_error(crate::errors::ValidationError::InvalidValue(format!(
        //                     "Service volumes reference unknown volume: {}",
        //                     s.volume
        //                 )));
        //             }
        //         });
        //     }
        //     Volumes::Long(s) => {
        //         ctx.volumes.as_ref().map(|available_volumes| {
        //             if !available_volumes.contains_key(s.source.as_str()) {
        //                 errors.add_error(crate::errors::ValidationError::InvalidValue(format!(
        //                     "Service volumes reference unknown volume: {}",
        //                     s.source
        //                 )));
        //             }
        //         });
        //     }
        // }
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_volume_reference() {
        let yaml = r#"
        services:
          backend:
            image: example/backend
            volumes:
              - type: volume
                source: db-data
                target: /data
                volume:
                  nocopy: true
        
        volumes:
          hello:
        "#;
        let compose = Compose::new(yaml);
        assert!(compose.is_ok());

        // See above comment about validation
        // assert!(compose.is_err());
    }

    #[test]
    fn valid_volume_reference() {
        let yaml = r#"
        services:
          backend:
            image: example/backend
            volumes:
              - type: volume
                source: hello
                target: /data
                volume:
                  nocopy: true
        
        volumes:
          hello:
        "#;
        let compose = Compose::new(yaml);
        assert!(compose.is_ok());
    }
}
