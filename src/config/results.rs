use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionResult {
    pub session_type: String,
    pub track_name: String,
    pub session_index: u32,
    pub race_weekend_index: u32,
    pub meta_data: String,
    pub server_name: String,
    pub is_wet_session: u8,
    pub best_lap: u32,
    #[serde(default)]
    pub best_splits: Vec<u32>,
    #[serde(default)]
    pub is_inverted_top_grid: u8,
    pub session_result: InnerResult,
    #[serde(default)]
    pub laps: Vec<Lap>,
    #[serde(default)]
    pub penalties: Vec<Penalty>,
    #[serde(default)]
    pub post_race_penalties: Vec<Penalty>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InnerResult {
    pub best_lap: u32,
    #[serde(default)]
    pub best_splits: Vec<u32>,
    pub is_wet_session: u8,
    #[serde(rename = "type", default)]
    pub session_type_num: u8,
    #[serde(rename = "leaderBoardLines", default)]
    pub leaderboard_lines: Vec<LeaderboardLine>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardLine {
    pub car: Car,
    pub current_driver: Driver,
    pub current_driver_index: u32,
    pub timing: Timing,
    pub missing_mandatory_pitstop: u8,
    #[serde(default)]
    pub driver_total_times: Vec<f64>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub car_id: u32,
    pub race_number: u32,
    pub cup_category: u8,
    pub car_model: u32,
    pub team_name: String,
    pub nationality: u32,
    #[serde(default)]
    pub drivers: Vec<Driver>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
    pub nationality: u32,
    pub player_id: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    pub last_lap: u32,
    #[serde(default)]
    pub last_splits: Vec<u32>,
    pub best_lap: u32,
    #[serde(default)]
    pub best_splits: Vec<u32>,
    pub total_time: u32,
    pub lap_count: u32,
    pub last_split_id: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lap {
    pub car_id: u32,
    pub driver_index: u32,
    pub lap_time: u32,
    #[serde(default)]
    pub splits: Vec<u32>,
    pub is_invalid: u8,
    pub is_valid_for_best: u8,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Penalty {
    pub car_id: u32,
    pub driver_index: u32,
    #[serde(default)]
    pub violated_driver_index: u32,
    pub reason: String,
    pub penalty: String,
    pub penalty_value: u32,
    pub violation_in_lap: u32,
    pub cleared_in_lap: u32,
    #[serde(default)]
    pub post_race_investigated: u8,
}
