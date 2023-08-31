use std::collections::HashMap;

use super::{configs, networks, secrets, services, volumes};
use serde::Deserialize;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Compose {
    pub version: Option<String>,
    pub services: HashMap<String, services::Service>,
    pub networks: Option<HashMap<String, Option<networks::Network>>>,
    pub volumes: Option<HashMap<String, Option<volumes::Volume>>>,
    pub configs: Option<HashMap<String, Option<configs::Config>>>,
    pub secrets: Option<HashMap<String, Option<secrets::Secret>>>,
}

impl Compose {
    pub fn validate(contents: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let compose: Self = serde_yaml::from_str(contents)?;
        Ok(compose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_compose() {
        let yaml = r#"
        "#;
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
        let compose = Compose::validate(yaml);
        assert!(compose.is_ok());
    }
}
