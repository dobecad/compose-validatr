//! compose-validatr is a library for validating and building valid Docker Compose manifests that are not dependent on the host system.
//!
//! # Table of Contents
//!
//! - [High-level features](#high-level-features)
//! - [Purpose](#purpose)
//! - [Examples](#examples)
//!
//!
//! # High-level Features
//!
//! - Create and validate a Compose structure from &str
//! - View multiple validation errors at once
//!  
//! # Purpose
//!
//! The main purpose of this library is for building and validating Docker Compose manifests
//! in contexts that are agnostic to a host machine. This is useful for web applications
//! that want to visualize existing Compose manifests or create valid manifests without
//! caring if host files or devices are present.
//!
//! # Examples
//!
//! ```rust
//! use compose_validatr::Compose;
//!
//! let yaml = r#"
//! version: '3.9'
//! services:
//!   gitlab:
//!     image: gitlab/gitlab-ce:latest
//!     container_name: gitlab
//!     hostname: gitlab
//!     restart: always
//!     depends_on:
//!       - postgres
//!     ports:
//!       - "8080:80"
//!       - "8443:443"
//!       - "8022:22"
//!     environment:
//!       GITLAB_ROOT_PASSWORD: eYPkjBbrtzX8eGVc
//!       DATABASE_URL: "postgres://gitlab:eYPkjBbrtzX8eGVc@postgres:5432/gitlab"
//!     volumes:
//!       - ./gitlab/config:/etc/gitlab
//!       - ./gitlab/logs:/var/log/gitlab
//!       - ./gitlab/data:/var/opt/gitlab
//!     shm_size: '256m'
//!
//!   registry:
//!     image: registry:2
//!     container_name: registry
//!     hostname: registry
//!     ports:
//!       - "5000:5000"
//!     volumes:
//!       - registry:/var/lib/registry
//!
//!   sonarqube:
//!     build:
//!       context: ./sonarqube_image
//!     container_name: sonarqube
//!     hostname: sonarqube
//!     restart: always
//!     ports:
//!       - "9000:9000"
//!       - "9092:9092"
//!     volumes:
//!       - sonarqube:/opt/sonarqube/data
//!       - sonarqube:/opt/sonarqube/logs
//!       - sonarqube:/opt/sonarqube/extensions
//!
//!   jenkins:
//!     build:
//!       context: ./jenkins_image
//!     container_name: jenkins
//!     hostname: jenkins
//!     restart: always
//!     ports:
//!       - "9080:8080"
//!       - "50000:50000"
//!     volumes:
//!       - jenkins:/var/jenkins_home
//!       - jenkins-data:/var/jenkins_home
//!       - jenkins-docker-certs:/certs/client:ro
//!     environment:
//!       - JAVA_OPTS=-Djenkins.install.runSetupWizard=false
//!       - DOCKER_HOST=tcp://docker:2376
//!       - DOCKER_CERT_PATH=/certs/client
//!       - DOCKER_TLS_VERIFY=1
//!
//!   jenkins-docker:
//!     image: docker:dind
//!     container_name: jenkins-docker
//!     hostname: docker
//!     privileged: true
//!     environment:
//!       - DOCKER_TLS_CERTDIR=/certs
//!     volumes:
//!       - /etc/docker/daemon.json:/etc/docker/daemon.json
//!       - jenkins-docker-certs:/certs/client
//!       - jenkins-data:/var/jenkins_home
//!     ports:
//!       - '2376:2376'
//!     command: --storage-driver overlay2
//!
//!   postgres:
//!     image: postgres:latest
//!     container_name: postgres
//!     hostname: postgres
//!     restart: always
//!     ports:
//!       - "5432:5432"
//!     volumes:
//!       - postgres:/var/lib/postgresql/data
//!     environment:
//!       POSTGRES_DB: gitlab
//!       POSTGRES_USER: gitlab
//!       POSTGRES_PASSWORD: eYPkjBbrtzX8eGVc
//!
//! volumes:
//!   sonarqube:
//!   jenkins:
//!   jenkins-docker-certs:
//!   jenkins-data:
//!   postgres:
//!   registry:
//!
//! networks:
//!   default:
//!     driver: bridge
//! "#;
//!
//! // Create a new `Compose` from the &str. This will return a Result<Compose, ValidationErrors>
//! let compose = Compose::new(yaml);
//! match &compose {
//!   Ok(c) => {
//!     // Compose is valid. A lot of Compose fields are optional, so maps are very useful here
//!     c.version.as_ref().map(|v| println!("Version: {v}"));
//!     ()
//!   }
//!   Err(errors) => {
//!     // Compose had one or many errors
//!     ()
//!   }
//! }
//! ```

pub mod compose;
pub mod configs;
pub mod errors;
pub mod networks;
pub mod secrets;
pub mod services;
pub mod volumes;

pub use crate::compose::Compose;
