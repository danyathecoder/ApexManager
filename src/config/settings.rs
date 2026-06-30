use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_name: "ACC Dedicated Server".to_string(),
            admin_password: String::new(),
            car_group: "FreeForAll".to_string(),
            track_medals_requirement: 0,
            safety_rating_requirement: -1,
            racecraft_rating_requirement: -1,
            password: String::new(),
            spectator_password: String::new(),
            max_car_slots: 30,
            dump_leaderboards: 1,
            is_race_locked: 1,
            is_prep_phase_locked: 0,
            randomize_track_when_empty: 0,
            central_entry_list_path: String::new(),
            allow_auto_dq: 1,
            short_formation_lap: 1,
            dump_entry_list: 0,
            formation_lap_type: 3,
            ignore_premature_disconnects: 1,
            do_driver_swap_broadcast: 1,
            latency_strategy: 0,
            config_version: 1,
        }
    }
}

pub const FILENAME: &str = "settings.json";
