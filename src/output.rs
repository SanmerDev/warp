use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Output {
    pub account: Account,
    pub config: Config,
    pub created: String,
    pub id: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    pub account_type: String,
    pub id: String,
    pub managed: String,
    pub organization: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub client_id: String,
    pub peers: Vec<Peer>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Peer {
    pub public_key: String,
    pub endpoint: Endpoint,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Endpoint {
    pub host: String,
    pub ports: Vec<u64>,
    pub v4: String,
    pub v6: String,
}
