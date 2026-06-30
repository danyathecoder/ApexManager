use crate::app::{App, ConfigFile};
use crate::ui::widgets::{card, help_panel, help_row};

const CAR_GROUPS: &[&str] = &["FreeForAll", "GT3", "GT4", "GTC", "TCX"];
const FORMATION_LAP_TYPES: &[(&str, &str)] = &[
    ("0", "Old limiter lap"),
    ("1", "New formation lap"),
    ("3", "Free (no limiter)"),
];

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Settings");
    ui.add_space(4.0);

    help_panel(ui, "settings", |ui| {
        help_row(ui, "Server Name",         "Shown in the server browser. Keep it short and descriptive.");
        help_row(ui, "Admin Password",      "Required before /admin command works in-game. Blank = admin disabled.");
        help_row(ui, "Join Password",       "Leave blank for a public server. Set to restrict access.");
        help_row(ui, "Car Group",           "FreeForAll = all classes; GT3 / GT4 / GTC / TCX = restrict to that class only.");
        help_row(ui, "Max Car Slots",       "Max cars on track. Server warns in log if > 10 without rating requirements.");
        help_row(ui, "Track Medals",        "0 = no requirement. 1–3 medals required to join.");
        help_row(ui, "Safety Rating",       "-1 = disabled. 0–99 minimum Safety Rating required.");
        help_row(ui, "Racecraft Rating",    "-1 = disabled. 0–99 minimum Racecraft Rating required.");
        help_row(ui, "Formation Lap",       "0 = old limiter lap. 1 = new formation lap. 3 = free (no speed limiter).");
        help_row(ui, "Race Locked",         "Prevents spectators from joining during the race session.");
        help_row(ui, "Short Formation Lap", "Skips part of the formation lap for shorter events.");
        help_row(ui, "Allow Auto DQ",       "Server auto-disqualifies cars that violate rules without admin input.");
    });

    ui.add_space(4.0);

    let s = &mut app.settings;
    let mut changed = false;

    card(ui, "SERVER IDENTITY", |ui| {
        egui::Grid::new("s_identity").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
            ui.label("Server Name");
            changed |= ui.add(egui::TextEdit::singleline(&mut s.server_name).desired_width(f32::INFINITY)).changed();
            ui.end_row();

            ui.label("Admin Password");
            changed |= ui.add(egui::TextEdit::singleline(&mut s.admin_password).password(true).desired_width(f32::INFINITY)).changed();
            ui.end_row();

            ui.label("Join Password");
            changed |= ui.add(egui::TextEdit::singleline(&mut s.password).password(true).desired_width(f32::INFINITY)).changed();
            ui.end_row();

            ui.label("Spectator Password");
            changed |= ui.add(egui::TextEdit::singleline(&mut s.spectator_password).password(true).desired_width(f32::INFINITY)).changed();
            ui.end_row();
        });
    });

    card(ui, "SESSION SETUP", |ui| {
        egui::Grid::new("s_session").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
            ui.label("Car Group");
            egui::ComboBox::from_id_salt("car_group")
                .selected_text(&s.car_group)
                .show_ui(ui, |ui| {
                    for &g in CAR_GROUPS {
                        if ui.selectable_value(&mut s.car_group, g.to_string(), g).changed() {
                            changed = true;
                        }
                    }
                });
            ui.end_row();

            ui.label("Max Car Slots").on_hover_text("Maximum cars on track simultaneously");
            changed |= ui.add(egui::DragValue::new(&mut s.max_car_slots).range(1..=120)).changed();
            ui.end_row();

            ui.label("Track Medals Required").on_hover_text("0 = none, 1–3 medals required");
            changed |= ui.add(egui::DragValue::new(&mut s.track_medals_requirement).range(0..=3)).changed();
            ui.end_row();

            ui.label("Safety Rating Required").on_hover_text("-1 = disabled, 0–99");
            changed |= ui.add(egui::DragValue::new(&mut s.safety_rating_requirement).range(-1..=99)).changed();
            ui.end_row();

            ui.label("Racecraft Rating Required").on_hover_text("-1 = disabled, 0–99");
            changed |= ui.add(egui::DragValue::new(&mut s.racecraft_rating_requirement).range(-1..=99)).changed();
            ui.end_row();

            ui.label("Formation Lap");
            egui::ComboBox::from_id_salt("formation_lap")
                .selected_text(
                    FORMATION_LAP_TYPES
                        .iter()
                        .find(|&&(v, _)| v.parse::<u8>().ok() == Some(s.formation_lap_type))
                        .map(|&(_, l)| l)
                        .unwrap_or("?"),
                )
                .show_ui(ui, |ui| {
                    for &(val, label) in FORMATION_LAP_TYPES {
                        let v: u8 = val.parse().unwrap();
                        if ui.selectable_value(&mut s.formation_lap_type, v, label).changed() {
                            changed = true;
                        }
                    }
                });
            ui.end_row();
        });
    });

    card(ui, "RACE OPTIONS", |ui| {
        egui::Grid::new("s_options").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
            macro_rules! cb {
                ($label:expr, $field:expr) => {{
                    ui.label($label);
                    let mut b = $field != 0;
                    if ui.checkbox(&mut b, "").changed() { $field = b as u8; changed = true; }
                    ui.end_row();
                }};
            }

            cb!("Race Locked",               s.is_race_locked);
            cb!("Short Formation Lap",       s.short_formation_lap);
            cb!("Allow Auto DQ",             s.allow_auto_dq);
            cb!("Dump Leaderboards",         s.dump_leaderboards);
            cb!("Dump Entry List",           s.dump_entry_list);
            cb!("Ignore Premature Disconnects", s.ignore_premature_disconnects);
            cb!("Randomize Track When Empty",s.randomize_track_when_empty);
        });
    });

    card(ui, "ADVANCED", |ui| {
        egui::Grid::new("s_advanced").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
            ui.label("Central Entry List Path").on_hover_text("Private servers only");
            ui.horizontal(|ui| {
                changed |= ui.add(egui::TextEdit::singleline(&mut s.central_entry_list_path).desired_width(f32::INFINITY)).changed();
                if ui.small_button("Browse…").clicked() {
                    if let Some(p) = rfd::FileDialog::new().pick_folder() {
                        s.central_entry_list_path = p.to_string_lossy().into_owned();
                        changed = true;
                    }
                }
            });
            ui.end_row();
        });
    });

    if changed {
        app.dirty.insert(ConfigFile::Settings);
    }
}
