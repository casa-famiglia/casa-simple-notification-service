use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub network: String,
    pub environment: String,
    pub description: String,
    pub token: String,
    pub is_production: bool,
    pub default: bool,
    pub endpoints: Vec<Endpoint>,
    pub block_explorers: Vec<BlockExplorer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub provider: String,
    pub daemon: String,
    pub indexer: String,
    pub port: u16,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockExplorer {
    pub name: String,
    pub provider: String,
    pub domain: String,
    pub queries: Queries,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Queries {
    pub transaction: String,
    pub asset: String,
    pub account: String,
    pub application: String,
    pub block: String,
}
