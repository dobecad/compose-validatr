use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlkioConfig {
    pub weight: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_device: Option<Vec<WeightDevice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_read_bps: Option<Vec<DeviceReadBps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_read_iops: Option<Vec<DeviceReadIops>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_write_bps: Option<Vec<DeviceWriteBps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_write_iops: Option<Vec<DeviceWriteIops>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeightDevice {
    pub path: String,
    pub weight: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceReadBps {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceWriteBps {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceReadIops {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceWriteIops {
    pub path: String,
    pub rate: Rate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Rate {
    String(String),
    Bytes(u64),
}

impl Validate for BlkioConfig {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        // Not interested in verifying host devices
        ()
    }
}
