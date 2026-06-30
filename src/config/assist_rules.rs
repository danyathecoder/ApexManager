use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

impl Default for AssistRules {
    fn default() -> Self {
        Self {
            stability_control_level_max: 100, // unrestricted — players choose their own level
            disable_autosteer: 0,
            disable_auto_lights: 0,
            disable_auto_wiper: 0,
            disable_auto_engine_start: 0,
            disable_auto_pit_limiter: 0,
            disable_auto_gear: 0,
            disable_auto_clutch: 0,
            disable_ideal_line: 0,
        }
    }
}

pub const FILENAME: &str = "assistRules.json";
