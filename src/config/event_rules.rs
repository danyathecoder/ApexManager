use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct EventRules {
    pub qualify_standing_type: u8,
    pub superpole_max_car: i32,
    pub pit_window_length_sec: i32,
    pub driver_stint_time_sec: i32,
    pub mandatory_pitstop_count: u32,
    pub max_total_driving_time: i32,
    pub max_drivers_count: u32,
    pub is_refuelling_allowed_in_race: bool,
    pub is_refuelling_time_fixed: bool,
    pub is_mandatory_pitstop_refuelling_required: bool,
    pub is_mandatory_pitstop_tyre_change_required: bool,
    pub is_mandatory_pitstop_swap_driver_required: bool,
    pub tyre_set_count: u32,
}

pub const FILENAME: &str = "eventRules.json";
