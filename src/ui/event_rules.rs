use crate::app::{App, ConfigFile};
use crate::ui::widgets::{help_panel, help_row};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Event Rules");
    ui.add_space(4.0);

    help_panel(ui, "event_rules", |ui| {
        help_row(ui, "Qualify Standing",   "1 = fastest single lap (recommended). 2 = average lap time.");
        help_row(ui, "Superpole Laps",     "Number of hotlap attempts in superpole qualifying. 0 = unlimited.");
        help_row(ui, "Pit Window",         "-1 = disabled. Otherwise: earliest second of race when pits open.");
        help_row(ui, "Driver Stint",       "-1 = disabled. Max seconds a driver may drive before mandatory pit. Must also set Max Total Driving Time.");
        help_row(ui, "Max Total Driving",  "-1 = disabled. Total drive time cap for one driver. Interdependent with Driver Stint.");
        help_row(ui, "Mandatory Pitstops", "0 = none. 1+ = how many times each car must pit during the race.");
        help_row(ui, "Max Drivers",        "1 = solo race. 2–3 = endurance (driver swaps allowed up to this count).");
        help_row(ui, "Refuelling",         "Allow / require refuelling during mandatory pit stops.");
        help_row(ui, "Tyre Set Count",     "How many tyre sets each car has (1–50). 50 = effectively unlimited. Experimental.");
        help_row(ui, "Pitstop Requirements", "Force each mandatory stop to include: refuel, tyre change, or driver swap (any combination).");
    });

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
