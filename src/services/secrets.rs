use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Secrets {
    Short(Vec<String>),
    Long(HashMap<String, SecretOptions>),
}

#[derive(Debug, Deserialize)]
pub struct SecretOptions {
    pub source: String,
    pub target: String,
    pub uid: String,
    pub gid: String,
    pub mode: String,
}
