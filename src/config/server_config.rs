use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub udp_port: u16,
    pub tcp_port: u16,
    pub max_connections: u32,
    pub lan_discovery: u8,
    pub register_to_lobby: u8,
    pub public_ip: Option<String>,
    pub config_version: u32,
}

pub const FILENAME: &str = "configuration.json";
