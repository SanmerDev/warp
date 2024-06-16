use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Deserialize, Serialize, Debug)]
pub struct Key {
    pub private_key: String,
    pub public_key: String,
}

impl Key {
    pub fn new() -> Self {
        let private_key = StaticSecret::random();
        let public_key = PublicKey::from(&private_key);

        Self {
            private_key: STANDARD.encode(private_key),
            public_key: STANDARD.encode(public_key),
        }
    }
}
