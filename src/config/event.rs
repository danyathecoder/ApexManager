use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub track: String,
    pub pre_race_waiting_time_seconds: u32,
    pub session_over_time_seconds: u32,
    pub ambient_temp: f32,
    pub cloud_level: f32,
    pub rain: f32,
    pub weather_randomness: u8,
    pub post_qualy_seconds: u32,
    pub post_race_seconds: u32,
    pub sessions: Vec<Session>,
    pub meta_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simracer_weather_conditions: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fixed_condition_qualification: Option<u8>,
    pub config_version: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub hour_of_day: u8,
    pub day_of_weekend: u8,
    pub time_multiplier: u8,
    pub session_type: String,
    pub session_duration_minutes: u32,
}

pub const FILENAME: &str = "event.json";
