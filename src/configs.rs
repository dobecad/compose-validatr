use std::collections::HashMap;

pub struct Configs {
    configs: HashMap<String, ConfigAttributes>,
}

pub struct ConfigAttributes {
    file: String,
    external: bool,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
}
