use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HealthCheck {
    pub test: Option<Test>,
    pub interval: Option<String>,
    pub timeout: Option<String>,
    pub retries: Option<u8>,
    pub start_period: Option<String>,
    pub start_interval: Option<String>,
    pub disable: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum Test {
    String(String),
    List(Vec<String>),
}
