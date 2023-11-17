use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthCheck {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Test>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_interval: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Test {
    String(String),
    List(Vec<String>),
}

impl Validate for HealthCheck {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        ()
    }
}
