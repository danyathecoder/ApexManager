use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct AssistRules {
    pub stability_control_level_max: u8,
    pub disable_autosteer: u8,
    pub disable_auto_lights: u8,
    pub disable_auto_wiper: u8,
    pub disable_auto_engine_start: u8,
    pub disable_auto_pit_limiter: u8,
    pub disable_auto_gear: u8,
    pub disable_auto_clutch: u8,
    pub disable_ideal_line: u8,
}

pub const FILENAME: &str = "assistRules.json";
