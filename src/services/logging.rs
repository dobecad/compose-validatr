use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub driver: Option<String>,
    pub options: Option<Options>,
}

#[derive(Debug, Deserialize)]
pub struct Options {
    pub syslog_address: String,
}
