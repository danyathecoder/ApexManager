use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct EntryList {
    pub entries: Vec<Entry>,
    pub force_entry_list: u8,
    pub config_version: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub drivers: Vec<Driver>,
    pub race_number: i32,
    pub forced_car_model: i32,
    pub override_driver_info: u8,
    pub custom_car: String,
    pub override_car_model_for_custom_car: u8,
    pub is_server_admin: u8,
    pub default_grid_position: i32,
    pub ballast_kg: u32,
    pub restrictor: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub player_id: String,
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
    pub driver_category: u8,
}

pub const FILENAME: &str = "entrylist.json";
