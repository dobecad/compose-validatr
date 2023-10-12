use std::collections::HashMap;

use crate::{
    configs::Config,
    errors::{ValidationError, ValidationErrors},
    networks::Network,
    secrets::Secret,
    services::Service,
    volumes::Volume,
};

use super::{configs, networks, secrets, services, volumes};
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Compose {
    pub version: Option<String>,
    pub services: HashMap<String, services::Service>,
    pub networks: Option<HashMap<String, Option<networks::Network>>>,
    pub volumes: Option<HashMap<String, Option<volumes::Volume>>>,
    pub configs: Option<HashMap<String, Option<configs::Config>>>,
    pub secrets: Option<HashMap<String, Option<secrets::Secret>>>,
}

impl Compose {
    pub fn new(contents: &str) -> Result<Self, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let compose: Result<Self, ValidationError> = serde_yaml::from_str(contents)
            .map_err(|e| ValidationError::InvalidCompose(e.to_string()));

        match compose {
            Ok(c) => {
                if let Some(networks) = &c.networks {
                    Self::validate_networks(&c, networks, &mut errors);
                };
                if let Some(volumes) = &c.volumes {
                    Self::validate_volumes(&c, volumes, &mut errors);
                };
                if let Some(configs) = &c.configs {
                    Self::validate_configs(&c, configs, &mut errors);
                };
                if let Some(secrets) = &c.secrets {
                    Self::validate_secrets(&c, secrets, &mut errors);
                };
                Self::validate_services(&c, &c.services, &mut errors);
                if errors.has_errors() {
                    return Err(errors);
                }
                Ok(c)
            }
            Err(err) => {
                errors.add_error(err);
                Err(errors)
            }
        }
    }

    fn validate_networks(
        compose: &Compose,
        networks: &HashMap<String, Option<Network>>,
        errors: &mut ValidationErrors,
    ) {
        for (_, network_attributes) in networks {
            if let Some(network) = network_attributes {
                network.validate(compose, errors);
            }
        }
    }

    fn validate_volumes(
        compose: &Compose,
        volumes: &HashMap<String, Option<Volume>>,
        errors: &mut ValidationErrors,
    ) {
        for (_, volume_attributes) in volumes {
            if let Some(volume) = volume_attributes {
                volume.validate(compose, errors);
            }
        }
    }

    fn validate_configs(
        compose: &Compose,
        configs: &HashMap<String, Option<Config>>,
        errors: &mut ValidationErrors,
    ) {
        for (_, config_attributes) in configs {
            if let Some(config) = config_attributes {
                config.validate(compose, errors);
            }
        }
    }

    fn validate_secrets(
        compose: &Compose,
        secrets: &HashMap<String, Option<Secret>>,
        errors: &mut ValidationErrors,
    ) {
        for (_, secret_attributes) in secrets {
            if let Some(secret) = secret_attributes {
                secret.validate(compose, errors);
            }
        }
    }

    fn validate_services(
        compose: &Compose,
        services: &HashMap<String, Service>,
        errors: &mut ValidationErrors,
    ) {
        for (_, service) in services {
            service.validate(compose, errors);
        }
    }
}

/// This trait needs to be implemented for top level elements
pub(crate) trait Validate {
    /// Validate an attribute is valid within the context of the compose yaml
    ///
    /// Push all validation errors to the ValidationErrors so that users are able to see
    /// all of their errors at once, versus incrementally
    fn validate(&self, ctx: &Compose, errors: &mut ValidationErrors);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_compose() {
        let yaml = r#"
        "#;

        let compose = Compose::new(yaml);
        assert!(compose.is_err());
    }

    #[test]
    fn big_compose() {
        let yaml = r#"
        version: '3.9'

        services:
          gitlab:
            image: gitlab/gitlab-ce:latest
            container_name: gitlab
            hostname: gitlab
            restart: always
            depends_on:
              - postgres
            ports:
              - "8080:80"
              - "8443:443"
              - "8022:22"
            environment:
              GITLAB_ROOT_PASSWORD: eYPkjBbrtzX8eGVc
              DATABASE_URL: "postgres://gitlab:eYPkjBbrtzX8eGVc@postgres:5432/gitlab"
            volumes:
              - ./gitlab/config:/etc/gitlab
              - ./gitlab/logs:/var/log/gitlab
              - ./gitlab/data:/var/opt/gitlab
            shm_size: '256m'
        
          registry:
            image: registry:2
            container_name: registry
            hostname: registry
            ports:
              - "5000:5000"
            volumes:
              - registry:/var/lib/registry
        
          sonarqube:
            build:
              context: ./sonarqube_image
            container_name: sonarqube
            hostname: sonarqube
            restart: always
            ports:
              - "9000:9000"
              - "9092:9092"
            volumes:
              - sonarqube:/opt/sonarqube/data
              - sonarqube:/opt/sonarqube/logs
              - sonarqube:/opt/sonarqube/extensions
        
          jenkins:
            build:
              context: ./jenkins_image
            container_name: jenkins
            hostname: jenkins
            restart: always
            ports:
              - "9080:8080"
              - "50000:50000"
            volumes:
              - jenkins:/var/jenkins_home
              - jenkins-data:/var/jenkins_home
              - jenkins-docker-certs:/certs/client:ro
            environment:
              - JAVA_OPTS=-Djenkins.install.runSetupWizard=false
              - DOCKER_HOST=tcp://docker:2376
              - DOCKER_CERT_PATH=/certs/client
              - DOCKER_TLS_VERIFY=1
        
          jenkins-docker:
            image: docker:dind
            container_name: jenkins-docker
            hostname: docker
            privileged: true
            environment:
              - DOCKER_TLS_CERTDIR=/certs
            volumes:
              - /etc/docker/daemon.json:/etc/docker/daemon.json
              - jenkins-docker-certs:/certs/client
              - jenkins-data:/var/jenkins_home
            ports:
              - '2376:2376'
            command: --storage-driver overlay2
        
          postgres:
            image: postgres:latest
            container_name: postgres
            hostname: postgres
            restart: always
            ports:
              - "5432:5432"
            volumes:
              - postgres:/var/lib/postgresql/data
            environment:
              POSTGRES_DB: gitlab
              POSTGRES_USER: gitlab
              POSTGRES_PASSWORD: eYPkjBbrtzX8eGVc
        
        volumes:
          sonarqube:
          jenkins:
          jenkins-docker-certs:
          jenkins-data:
          postgres:
          registry:
        
        networks:
          default:
            driver: bridge
        "#;
        let compose = Compose::new(yaml);
        assert!(compose.is_ok());
    }
}
