use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    compose::{Compose, Validate},
    errors::ValidationError,
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Build {
    String(String),

    // We do not care about the build details
    Map(BuildDetails),
}

#[derive(Debug, Deserialize)]
pub struct BuildDetails {
    pub context: Option<String>,
    pub dockerfile: Option<String>,
    pub dockerfile_inline: Option<String>, // dockerfile must NOT be defined
    pub args: Option<BuildArgs>,
    pub ssh: Option<SshArgs>,
    pub cache_from: Option<Vec<String>>,
    pub cache_to: Option<Vec<String>>,
    pub additional_contexts: Option<AdditionalContexts>,
    pub extra_hosts: Option<Vec<String>>,
    pub isolation: Option<String>,
    pub privileged: Option<bool>,
    pub labels: Option<Labels>,
    pub no_cache: Option<bool>,
    pub pull: Option<bool>,
    pub shm_size: Option<ShmSize>,
    pub target: Option<String>,
    pub secrets: Option<Vec<BuildSecret>>, // must be defined secrets
    pub tags: Option<Vec<String>>,
    pub platforms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub enum BuildArgs {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub enum SshArgs {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub enum AdditionalContexts {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub enum Labels {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub enum ShmSize {
    String(String),
    Bytes(u64),
}

#[derive(Debug, Deserialize)]
pub enum BuildSecret {
    Short(String),
    Long(SecretDetails),
}

#[derive(Debug, Deserialize)]
pub struct SecretDetails {
    pub source: String,
    pub target: String,
    pub uid: String,
    pub gid: String,
    pub mode: String,
}

impl Validate for Build {
    fn validate(&self, ctx: &Compose, errors: &mut crate::errors::ValidationErrors) {
        // Check that specified secrets exist
        match self {
            Build::Map(details) => {
                let results = details.secrets.as_ref().map(|s| {
                    s.iter().all(|secret| match secret {
                        BuildSecret::Short(short) => ctx
                            .secrets
                            .as_ref()
                            .map(|x| x.contains_key(short))
                            .is_some(),
                        BuildSecret::Long(details) => ctx
                            .secrets
                            .as_ref()
                            .map(|x| x.contains_key(&details.source))
                            .is_some(),
                    })
                });
                results.as_ref().map(|r| {
                    if !r {
                        errors.add_error(ValidationError::InvalidValue(
                            "Build definition secret is not defined".to_string(),
                        ));
                    }
                });
            }
            _ => (),
        }

        match self {
            Build::Map(details) => {
                if details.dockerfile_inline.is_some() && details.dockerfile.is_some() {
                    errors.add_error(ValidationError::InvalidValue(
                        "Cannot specify a Dockerfile and an inline Dockerfile".to_string(),
                    ))
                }
            }
            _ => (),
        }
        ()
    }
}
