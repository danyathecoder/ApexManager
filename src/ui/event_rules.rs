use crate::app::{App, ConfigFile};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Event Rules");
    ui.separator();

    let r = &mut app.event_rules;
    let mut changed = false;

    egui::Grid::new("event_rules_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
        ui.label("Qualify Standing Type").on_hover_text("1 = fastest lap, 2 = average lap");
        egui::ComboBox::from_id_salt("qst")
            .selected_text(if r.qualify_standing_type == 1 { "1 – Fastest Lap" } else { "2 – Average Lap" })
            .show_ui(ui, |ui| {
                if ui.selectable_value(&mut r.qualify_standing_type, 1, "1 – Fastest Lap").changed() { changed = true; }
                if ui.selectable_value(&mut r.qualify_standing_type, 2, "2 – Average Lap").changed() { changed = true; }
            });
        ui.end_row();

        ui.label("Pit Window Length (s)").on_hover_text("-1 = disabled");
        changed |= ui.add(egui::DragValue::new(&mut r.pit_window_length_sec).range(-1..=86400)).changed();
        ui.end_row();

        ui.label("Driver Stint Time (s)").on_hover_text("-1 = disabled. Interdependent with Max Total Driving Time.");
        changed |= ui.add(egui::DragValue::new(&mut r.driver_stint_time_sec).range(-1..=86400)).changed();
        if r.driver_stint_time_sec != -1 || r.max_total_driving_time != -1 {
            ui.end_row();
            ui.label("");
            ui.colored_label(egui::Color32::YELLOW, "⚠ Stint time and max driving time are interdependent — both must be set.");
        }
        ui.end_row();

        ui.label("Mandatory Pitstop Count");
        changed |= ui.add(egui::DragValue::new(&mut r.mandatory_pitstop_count)).changed();
        ui.end_row();

        ui.label("Max Total Driving Time (s)").on_hover_text("-1 = disabled");
        changed |= ui.add(egui::DragValue::new(&mut r.max_total_driving_time).range(-1..=86400)).changed();
        ui.end_row();

        ui.label("Max Drivers Count");
        changed |= ui.add(egui::DragValue::new(&mut r.max_drivers_count).range(1..=30)).changed();
        ui.end_row();

        ui.label("Refuelling Allowed in Race");
        if ui.checkbox(&mut r.is_refuelling_allowed_in_race, "").changed() { changed = true; }
        ui.end_row();

        ui.label("Refuelling Time Fixed");
        if ui.checkbox(&mut r.is_refuelling_time_fixed, "").changed() { changed = true; }
        ui.end_row();

        ui.label("Mandatory Pitstop – Refuelling");
        if ui.checkbox(&mut r.is_mandatory_pitstop_refuelling_required, "").changed() { changed = true; }
        ui.end_row();

        ui.label("Mandatory Pitstop – Tyre Change");
        if ui.checkbox(&mut r.is_mandatory_pitstop_tyre_change_required, "").changed() { changed = true; }
        ui.end_row();

        ui.label("Mandatory Pitstop – Driver Swap");
        if ui.checkbox(&mut r.is_mandatory_pitstop_swap_driver_required, "").changed() { changed = true; }
        ui.end_row();

        ui.label("Tyre Set Count").on_hover_text("(experimental)");
        ui.horizontal(|ui| {
            changed |= ui.add(egui::DragValue::new(&mut r.tyre_set_count).range(1..=50)).changed();
            ui.colored_label(egui::Color32::GOLD, "(experimental)");
        });
        ui.end_row();
    });

    if changed {
        app.dirty.insert(ConfigFile::EventRules);
    }
}
