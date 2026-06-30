use crate::{app::{App, ConfigFile}, config::event::Session, data::tracks::TRACKS, ui::widgets::{help_panel, help_row}};

struct WeatherPreset {
    label: &'static str,
    cloud: f32,
    rain: f32,
    randomness: u8,
}

const WEATHER_PRESETS: &[WeatherPreset] = &[
    WeatherPreset { label: "Clear",     cloud: 0.0, rain: 0.0, randomness: 0 },
    WeatherPreset { label: "Sunny",     cloud: 0.1, rain: 0.0, randomness: 1 },
    WeatherPreset { label: "Partly",    cloud: 0.4, rain: 0.0, randomness: 2 },
    WeatherPreset { label: "Overcast",  cloud: 0.8, rain: 0.0, randomness: 3 },
    WeatherPreset { label: "Light Rain",cloud: 0.7, rain: 0.3, randomness: 3 },
    WeatherPreset { label: "Rain",      cloud: 0.9, rain: 0.6, randomness: 4 },
    WeatherPreset { label: "Storm",     cloud: 1.0, rain: 1.0, randomness: 7 },
];

const RANDOMNESS_LABELS: &[&str] = &[
    "0 – Static",
    "1 – Minimal",
    "2 – Low",
    "3 – Moderate",
    "4 – Medium",
    "5 – High",
    "6 – Very High",
    "7 – Maximum",
];

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Event");
    ui.add_space(4.0);

    help_panel(ui, "event", |ui| {
        help_row(ui, "Track",              "ACC internal track key (e.g. monza, spa, nurburgring). Select from the dropdown.");
        help_row(ui, "Ambient Temp",       "Air temperature in °C. Affects tyre behaviour.");
        help_row(ui, "Cloud Level",        "0.0 = clear sky, 0.3 = light clouds, 1.0 = overcast.");
        help_row(ui, "Rain",               "0.0 = dry, 1.0 = heavy rain. Requires Wet Weather DLC.");
        help_row(ui, "Weather Randomness", "0 = fully static conditions. 1–7 = increasing variation during the event.");
        help_row(ui, "Pre-Race Wait",      "Seconds in lobby before the session opens. 80 s is the recommended default.");
        help_row(ui, "Session Over Time",  "Extra seconds after the session timer hits zero before the session ends.");
        help_row(ui, "Post-Qualy / Race",  "Pause in seconds between sessions (time for results screens).");
        help_row(ui, "Sessions",           "Add in order: P = Practice, Q = Qualifying, R = Race. ACC requires at least one non-race session.");
        help_row(ui, "Hour / Day",         "Hour of day (0–23) and day of weekend (1–3) when the session starts.");
        help_row(ui, "Time ×",             "Time-of-day speed multiplier. 1 = real time, 2 = 2× faster.");
    });

    ui.separator();

    let ev = &mut app.event;
    let mut changed = false;

    ui.strong("Track & Weather");
    egui::Grid::new("event_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
        ui.label("Track");
        egui::ComboBox::from_id_salt("track_combo")
            .selected_text(&ev.track)
            .show_ui(ui, |ui| {
                for t in TRACKS {
                    if ui.selectable_value(&mut ev.track, t.key.to_string(), t.key).changed() {
                        changed = true;
                    }
                }
            });
        ui.end_row();

        ui.label("Ambient Temp (°C)");
        changed |= ui.add(egui::DragValue::new(&mut ev.ambient_temp).range(0.0..=40.0).speed(0.5)).changed();
        ui.end_row();

        ui.label("Cloud Level").on_hover_text("0.0 (clear) – 1.0 (overcast)");
        changed |= ui.add(egui::Slider::new(&mut ev.cloud_level, 0.0..=1.0).step_by(0.1)).changed();
        ui.end_row();

        ui.label("Rain").on_hover_text("0.0 (dry) – 1.0 (heavy rain)");
        changed |= ui.add(egui::Slider::new(&mut ev.rain, 0.0..=1.0).step_by(0.1)).changed();
        ui.end_row();

        ui.label("Weather Randomness");
        let label = RANDOMNESS_LABELS.get(ev.weather_randomness as usize).copied().unwrap_or("?");
        changed |= ui.add(egui::Slider::new(&mut ev.weather_randomness, 0..=7).text(label)).changed();
        ui.end_row();

        ui.label("Presets");
        ui.horizontal(|ui| {
            for p in WEATHER_PRESETS {
                if ui.small_button(p.label).clicked() {
                    ev.cloud_level = p.cloud;
                    ev.rain = p.rain;
                    ev.weather_randomness = p.randomness;
                    changed = true;
                }
            }
        });
        ui.end_row();

        ui.label("Pre-Race Wait (s)");
        changed |= ui.add(egui::DragValue::new(&mut ev.pre_race_waiting_time_seconds)).changed();
        ui.end_row();

        ui.label("Session Over Time (s)");
        changed |= ui.add(egui::DragValue::new(&mut ev.session_over_time_seconds)).changed();
        ui.end_row();

        ui.label("Post-Qualy (s)");
        changed |= ui.add(egui::DragValue::new(&mut ev.post_qualy_seconds)).changed();
        ui.end_row();

        ui.label("Post-Race (s)");
        changed |= ui.add(egui::DragValue::new(&mut ev.post_race_seconds)).changed();
        ui.end_row();

        ui.label("Meta Data");
        changed |= ui.text_edit_singleline(&mut ev.meta_data).changed();
        ui.end_row();

        ui.label("Config Version");
        changed |= ui.add(egui::DragValue::new(&mut ev.config_version)).changed();
        ui.end_row();
    });

    ui.add_space(8.0);
    ui.strong("Sessions");

    let has_non_race = ev.sessions.iter().any(|s| s.session_type != "R");
    if !has_non_race && !ev.sessions.is_empty() {
        ui.colored_label(egui::Color32::RED, "⚠ No Practice or Qualifying session — ACC requires at least one non-race session.");
    }

    egui::Grid::new("sessions_grid")
        .num_columns(7)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.strong("Type");
            ui.strong("Hour");
            ui.strong("Day");
            ui.strong("Time ×");
            ui.strong("Duration (min)");
            ui.strong("");
            ui.end_row();

            let mut to_remove: Option<usize> = None;
            for (i, s) in ev.sessions.iter_mut().enumerate() {
                egui::ComboBox::from_id_salt(format!("stype_{i}"))
                    .selected_text(&s.session_type)
                    .width(50.0)
                    .show_ui(ui, |ui| {
                        for t in ["P", "Q", "R"] {
                            if ui.selectable_value(&mut s.session_type, t.to_string(), t).changed() {
                                changed = true;
                            }
                        }
                    });
                changed |= ui.add(egui::DragValue::new(&mut s.hour_of_day).range(0..=23)).changed();
                changed |= ui.add(egui::DragValue::new(&mut s.day_of_weekend).range(1..=3)).changed();
                changed |= ui.add(egui::DragValue::new(&mut s.time_multiplier).range(0..=24)).changed();
                changed |= ui.add(egui::DragValue::new(&mut s.session_duration_minutes)).changed();
                if ui.small_button("✕").clicked() {
                    to_remove = Some(i);
                }
                ui.end_row();
            }
            if let Some(i) = to_remove {
                ev.sessions.remove(i);
                changed = true;
            }
        });

    if ui.button("+ Add Session").clicked() {
        ev.sessions.push(Session {
            session_type: "P".to_string(),
            hour_of_day: 12,
            day_of_weekend: 1,
            time_multiplier: 1,
            session_duration_minutes: 20,
        });
        changed = true;
    }

    if changed {
        app.dirty.insert(ConfigFile::Event);
    }
}
