use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
pub struct BuildSecret {
    pub source: String,
    pub target: String,
    pub uid: String,
    pub gid: String,
    pub mode: String,
}
