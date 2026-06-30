use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Bop {
    pub entries: Vec<BopEntry>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BopEntry {
    pub track: String,
    pub car_model: u32,
    pub ballast_kg: u32,
    pub restrictor: u32,
}

pub const FILENAME: &str = "bop.json";
