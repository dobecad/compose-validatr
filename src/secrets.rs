use std::collections::HashMap;

pub struct Secrets {
    secrets: HashMap<String, SecretAttributes>,
}

pub struct SecretAttributes {
    file: String,
    environment: String,
    external: bool,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
}
