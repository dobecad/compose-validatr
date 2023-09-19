use std::collections::HashMap;

use serde::Deserialize;

use crate::compose::{Compose, Validate};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
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

impl Validate for Secrets {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        ()
    }
}
