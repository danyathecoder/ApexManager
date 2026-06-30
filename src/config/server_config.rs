use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct ServerConfig {
    pub udp_port: u16,
    pub tcp_port: u16,
    pub max_connections: u32,
    pub lan_discovery: u8,
    pub register_to_lobby: u8,
    pub public_ip: Option<String>,
    pub config_version: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            udp_port: 9231,
            tcp_port: 9323,
            max_connections: 30,
            lan_discovery: 1,
            register_to_lobby: 1,
            public_ip: None,
            config_version: 1,
        }
    }
}

pub const FILENAME: &str = "configuration.json";
