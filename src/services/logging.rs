use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Deserialize, Serialize)]
pub struct Logging {
    pub driver: Option<String>,
    pub options: Option<Options>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub syslog_address: String,
}

impl Validate for Logging {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        ()
    }
}
