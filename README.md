[![Unit Tests and Formatting](https://github.com/dobecad/compose-validatr/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/dobecad/compose-validatr/actions/workflows/tests.yml)

# compose-validatr

Rust library for validating and inspecting docker compose configs

## Goals

The main reason this library exists is for another web app project I am working on for visualizing docker-compose configs.

This library is primarily meant for parsing the metadata from a compose yaml and performing surface level validation.

This library performs all validation except host device and local files validation, since that is out of scope of what I need this library for.

## Example

```rust
// Simple docker-compose.yaml &str
let yaml = r#"
services:
  gitlab:
  image: gitlab/gitlab-ce:latest
  container_name: gitlab
  hostname: gitlab
  restart: always
  build:
    context: .
    dockerfile: webapp.Dockerfile
"#;

let compose = Compose::new(yaml).unwrap();
compose.services.keys().for_each(|service_name| println!("Service: {}", service_name));
```
