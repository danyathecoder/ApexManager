use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Event {
    pub track: String,
    pub event_type: String,
    pub pre_race_waiting_time_seconds: u32,
    pub session_over_time_seconds: u32,
    pub ambient_temp: f32,
    pub track_temp: f32,
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

impl Default for Event {
    fn default() -> Self {
        Self {
            track: "monza".to_string(),
            event_type: String::new(),
            pre_race_waiting_time_seconds: 80,
            session_over_time_seconds: 120,
            ambient_temp: 26.0,
            track_temp: 30.0,
            cloud_level: 0.3,
            rain: 0.0,
            weather_randomness: 1,
            post_qualy_seconds: 10,
            post_race_seconds: 15,
            sessions: vec![
                Session {
                    session_type: "P".to_string(),
                    hour_of_day: 10,
                    day_of_weekend: 1,
                    time_multiplier: 1,
                    session_duration_minutes: 20,
                },
                Session {
                    session_type: "Q".to_string(),
                    hour_of_day: 14,
                    day_of_weekend: 2,
                    time_multiplier: 1,
                    session_duration_minutes: 15,
                },
                Session {
                    session_type: "R".to_string(),
                    hour_of_day: 15,
                    day_of_weekend: 2,
                    time_multiplier: 1,
                    session_duration_minutes: 60,
                },
            ],
            meta_data: String::new(),
            simracer_weather_conditions: None,
            is_fixed_condition_qualification: None,
            config_version: 1,
        }
    }
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
