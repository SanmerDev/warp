use crate::wireguard::WireguardKey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterRequest {
    pub key: String,
    pub locale: String,
    pub name: String,
    pub os_version: String,
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
}

impl RegisterRequest {
    pub fn new(key: &WireguardKey) -> Self {
        Self {
            key: key.public_key.to_owned(),
            locale: "en-US".to_owned(),
            name: "Nothing phone (2)".to_owned(),
            os_version: "14.0.0".to_owned(),
            manufacturer: "Nothing".to_owned(),
            model: "A065".to_owned(),
            serial_number: "proxy".to_owned(),
        }
    }
}
