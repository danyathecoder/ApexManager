use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub server_name: String,
    pub admin_password: String,
    pub car_group: String,
    pub track_medals_requirement: i32,
    pub safety_rating_requirement: i32,
    pub racecraft_rating_requirement: i32,
    pub password: String,
    pub spectator_password: String,
    pub max_car_slots: u32,
    pub dump_leaderboards: u8,
    pub is_race_locked: u8,
    pub is_prep_phase_locked: u8,
    pub randomize_track_when_empty: u8,
    pub central_entry_list_path: String,
    pub allow_auto_dq: u8,
    pub short_formation_lap: u8,
    pub dump_entry_list: u8,
    pub formation_lap_type: u8,
    pub ignore_premature_disconnects: u8,
    pub do_driver_swap_broadcast: u8,
    pub latency_strategy: u8,
    pub config_version: u32,
}

pub const FILENAME: &str = "settings.json";
