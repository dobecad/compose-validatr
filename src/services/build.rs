use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    compose::{Compose, Validate},
    errors::ValidationError,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Build {
    String(String),

    // We do not care about the build details
    Map(BuildDetails),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BuildArgs {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SshArgs {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AdditionalContexts {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Labels {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ShmSize {
    String(String),
    Bytes(u64),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BuildSecret {
    Short(String),
    Long(SecretDetails),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
                details.secrets.as_ref().map(|s| {
                    s.iter().for_each(|secret| match secret {
                        BuildSecret::Short(short) => {
                            ctx.secrets.as_ref().map(|x| {
                                if !x.contains_key(short) {
                                    errors.add_error(ValidationError::InvalidValue(format!(
                                        "Secret is not defined: {}",
                                        short
                                    )))
                                }
                            });
                        }
                        BuildSecret::Long(details) => {
                            ctx.secrets.as_ref().map(|x| {
                                if !x.contains_key(&details.source) {
                                    errors.add_error(ValidationError::InvalidValue(format!(
                                        "Secret is not defined: {}",
                                        details.source
                                    )))
                                }
                            });
                        }
                    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_build() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            build:
              context: .
              dockerfile: webapp.Dockerfile
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_ok())
    }

    #[test]
    fn test_invalid_build_inline() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            build:
              context: .
              dockerfile: webapp.Dockerfile
              dockerfile_inline: |
                FROM baseimage
                RUN some command
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_err());
        assert!(compose.is_err_and(|e| e.all_errors().len() == 1));
    }

    #[test]
    fn test_valid_secret_reference() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            build:
              context: .
              dockerfile: webapp.Dockerfile
              secrets:
                - server-certificate
        secrets:
          server-certificate:
            file: ./server.cert
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_ok());
    }

    #[test]
    fn test_invalid_secret_reference() {
        let yaml = r#"
        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            build:
              context: .
              dockerfile: webapp.Dockerfile
              secrets:
                - server-certificate
        secrets:
          hello-world:
            file: ./server.cert
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_err());
    }
}
