use serde::Deserialize;

use crate::compose::{Compose, Validate};

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub driver: Option<String>,
    pub options: Option<Options>,
}

#[derive(Debug, Deserialize)]
pub struct Options {
    pub syslog_address: String,
}

impl Validate for Logging {
    fn validate(&self, ctx: &Compose, errors: &mut crate::errors::ValidationErrors) {
        todo!()
    }
}
