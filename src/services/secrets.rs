use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Secret {
    Short(String),
    Long(SecretOptions),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecretOptions {
    pub source: String,
    pub target: String,
    pub uid: String,
    pub gid: String,
    pub mode: String,
}

impl Validate for Secret {
    fn validate(&self, ctx: &Compose, errors: &mut crate::errors::ValidationErrors) {
        match self {
            Secret::Short(s) => {
                ctx.secrets.as_ref().map(|available_secrets| {
                    if !available_secrets.contains_key(s) {
                        errors.add_error(crate::errors::ValidationError::InvalidValue(format!(
                            "Service secrets reference an unknown secret: {s}"
                        )));
                    }
                });
            }
            Secret::Long(s) => {
                ctx.secrets.as_ref().map(|available_secrets| {
                    if !available_secrets.contains_key(s.source.as_str()) {
                        errors.add_error(crate::errors::ValidationError::InvalidValue(format!(
                            "Service secrets reference an unknown secret: {}",
                            s.source
                        )));
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_secret() {
        let yaml = r#"
        services:
          frontend:
            image: example/webapp
            secrets:
              - server-certificate
        secrets:
          helloworld:
            file: ./server.cert
          worldhello:
            file: ./server.cert
        "#;
        let compose = Compose::new(yaml);
        assert!(compose.is_err());
    }

    #[test]
    fn valid_secret() {
        let yaml = r#"
        services:
          frontend:
            image: example/webapp
            secrets:
              - helloworld
        secrets:
          helloworld:
            file: ./server.cert
        "#;
        let compose = Compose::new(yaml);
        assert!(compose.is_ok());
    }
}
