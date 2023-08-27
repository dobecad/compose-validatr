use std::collections::HashMap;

pub struct Volumes {
    volumes: HashMap<String, VolumeAttributes>,
}

pub struct VolumeAttributes {
    driver: String, // enum
    driver_opts: DriverOpts,
    external: String,
    labels: Vec<String>, // could be hashmap string,string
    name: String,
}

pub struct DriverOpts {
    driver_type: String, // rename to type
    o: String,
    device: String,
}
