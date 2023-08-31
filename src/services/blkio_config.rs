use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlkioConfig {
    pub weight: u16,
    pub weight_device: Option<Vec<WeightDevice>>,
    pub device_read_bps: Option<Vec<DeviceReadBps>>,
    pub device_read_iops: Option<Vec<DeviceReadIops>>,
    pub device_write_bps: Option<Vec<DeviceWriteBps>>,
    pub device_write_iops: Option<Vec<DeviceWriteIops>>,
}

#[derive(Debug, Deserialize)]
pub struct WeightDevice {
    pub path: String,
    pub weight: u16,
}

#[derive(Debug, Deserialize)]
pub struct DeviceReadBps {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Deserialize)]
pub struct DeviceWriteBps {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Deserialize)]
pub struct DeviceReadIops {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Deserialize)]
pub struct DeviceWriteIops {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Deserialize)]
pub enum Rate {
    String(String),
    Bytes(u64),
}
