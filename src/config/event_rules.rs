use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

impl Default for EventRules {
    fn default() -> Self {
        Self {
            qualify_standing_type: 1,  // fastest lap
            superpole_max_car: -1,     // disabled
            pit_window_length_sec: -1, // disabled
            driver_stint_time_sec: -1, // disabled
            mandatory_pitstop_count: 0,
            max_total_driving_time: -1, // disabled
            max_drivers_count: 1,
            is_refuelling_allowed_in_race: true,
            is_refuelling_time_fixed: false,
            is_mandatory_pitstop_refuelling_required: false,
            is_mandatory_pitstop_tyre_change_required: false,
            is_mandatory_pitstop_swap_driver_required: false,
            tyre_set_count: 50,
        }
    }
}

pub const FILENAME: &str = "eventRules.json";
