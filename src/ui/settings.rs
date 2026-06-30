use crate::app::{App, ConfigFile};

const CAR_GROUPS: &[&str] = &["FreeForAll", "GT3", "GT4", "GTC", "TCX"];
const FORMATION_LAP_TYPES: &[(&str, &str)] = &[
    ("0", "Old limiter lap"),
    ("1", "New formation lap"),
    ("3", "Free (no limiter)"),
];

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Settings");
    ui.separator();

    let s = &mut app.settings;
    let mut changed = false;

    egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            ui.label("Server Name");
            changed |= ui.text_edit_singleline(&mut s.server_name).changed();
            ui.end_row();

            ui.label("Admin Password");
            changed |= ui.add(egui::TextEdit::singleline(&mut s.admin_password).password(true)).changed();
            ui.end_row();

            ui.label("Password");
            changed |= ui.add(egui::TextEdit::singleline(&mut s.password).password(true)).changed();
            ui.end_row();

            ui.label("Spectator Password");
            changed |= ui.add(egui::TextEdit::singleline(&mut s.spectator_password).password(true)).changed();
            ui.end_row();

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

            ui.label("Track Medals Required").on_hover_text("0 = none, 1-3 medals required");
            changed |= ui.add(egui::DragValue::new(&mut s.track_medals_requirement).range(0..=3)).changed();
            ui.end_row();

            ui.label("Safety Rating Required").on_hover_text("-1 = disabled, 0-99");
            changed |= ui.add(egui::DragValue::new(&mut s.safety_rating_requirement).range(-1..=99)).changed();
            ui.end_row();

            ui.label("Racecraft Rating Required").on_hover_text("-1 = disabled, 0-99");
            changed |= ui.add(egui::DragValue::new(&mut s.racecraft_rating_requirement).range(-1..=99)).changed();
            ui.end_row();

            ui.label("Formation Lap Type").on_hover_text("0 = old limiter, 1 = new formation, 3 = free");
            egui::ComboBox::from_id_salt("formation_lap")
                .selected_text(
                    FORMATION_LAP_TYPES
                        .iter()
                        .find(|&&(v, _)| v.parse::<u8>().ok() == Some(s.formation_lap_type))
                        .map(|&(_, label)| label)
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

            let mut b = s.dump_leaderboards != 0;
            ui.label("Dump Leaderboards");
            if ui.checkbox(&mut b, "").changed() { s.dump_leaderboards = b as u8; changed = true; }
            ui.end_row();

            let mut b = s.is_race_locked != 0;
            ui.label("Race Locked").on_hover_text("Prevent spectators from joining during race");
            if ui.checkbox(&mut b, "").changed() { s.is_race_locked = b as u8; changed = true; }
            ui.end_row();

            let mut b = s.randomize_track_when_empty != 0;
            ui.label("Randomize Track When Empty");
            if ui.checkbox(&mut b, "").changed() { s.randomize_track_when_empty = b as u8; changed = true; }
            ui.end_row();

            let mut b = s.allow_auto_dq != 0;
            ui.label("Allow Auto DQ");
            if ui.checkbox(&mut b, "").changed() { s.allow_auto_dq = b as u8; changed = true; }
            ui.end_row();

            let mut b = s.short_formation_lap != 0;
            ui.label("Short Formation Lap");
            if ui.checkbox(&mut b, "").changed() { s.short_formation_lap = b as u8; changed = true; }
            ui.end_row();

            let mut b = s.dump_entry_list != 0;
            ui.label("Dump Entry List");
            if ui.checkbox(&mut b, "").changed() { s.dump_entry_list = b as u8; changed = true; }
            ui.end_row();

            let mut b = s.ignore_premature_disconnects != 0;
            ui.label("Ignore Premature Disconnects");
            if ui.checkbox(&mut b, "").changed() { s.ignore_premature_disconnects = b as u8; changed = true; }
            ui.end_row();

            ui.label("Central Entry List Path").on_hover_text("(private servers only)");
            ui.horizontal(|ui| {
                changed |= ui.text_edit_singleline(&mut s.central_entry_list_path).changed();
                if ui.small_button("Browse…").clicked() {
                    if let Some(p) = rfd::FileDialog::new().pick_folder() {
                        s.central_entry_list_path = p.to_string_lossy().into_owned();
                        changed = true;
                    }
                }
            });
            ui.end_row();
        });

    if changed {
        app.dirty.insert(ConfigFile::Settings);
    }
}
