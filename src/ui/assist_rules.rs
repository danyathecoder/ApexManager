use crate::app::{App, ConfigFile};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Assist Rules");
    ui.separator();

    let a = &mut app.assist_rules;
    let mut changed = false;

    egui::Grid::new("assist_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
        ui.label("Stability Control Max (%)").on_hover_text("0 = disabled, 100 = unrestricted");
        changed |= ui.add(egui::Slider::new(&mut a.stability_control_level_max, 0..=100).suffix("%")).changed();
        ui.end_row();

        let mut b = a.disable_autosteer != 0;
        ui.label("Disable Auto Steer");
        if ui.checkbox(&mut b, "").changed() { a.disable_autosteer = b as u8; changed = true; }
        ui.end_row();

        let mut b = a.disable_auto_lights != 0;
        ui.label("Disable Auto Lights");
        if ui.checkbox(&mut b, "").changed() { a.disable_auto_lights = b as u8; changed = true; }
        ui.end_row();

        let mut b = a.disable_auto_wiper != 0;
        ui.label("Disable Auto Wiper");
        if ui.checkbox(&mut b, "").changed() { a.disable_auto_wiper = b as u8; changed = true; }
        ui.end_row();

        let mut b = a.disable_auto_engine_start != 0;
        ui.label("Disable Auto Engine Start");
        if ui.checkbox(&mut b, "").changed() { a.disable_auto_engine_start = b as u8; changed = true; }
        ui.end_row();

        let mut b = a.disable_auto_pit_limiter != 0;
        ui.label("Disable Auto Pit Limiter");
        if ui.checkbox(&mut b, "").changed() { a.disable_auto_pit_limiter = b as u8; changed = true; }
        ui.end_row();

        let mut b = a.disable_auto_gear != 0;
        ui.label("Disable Auto Gear");
        if ui.checkbox(&mut b, "").changed() { a.disable_auto_gear = b as u8; changed = true; }
        ui.end_row();

        let mut b = a.disable_auto_clutch != 0;
        ui.label("Disable Auto Clutch");
        if ui.checkbox(&mut b, "").changed() { a.disable_auto_clutch = b as u8; changed = true; }
        ui.end_row();

        let mut b = a.disable_ideal_line != 0;
        ui.label("Disable Ideal Line");
        if ui.checkbox(&mut b, "").changed() { a.disable_ideal_line = b as u8; changed = true; }
        ui.end_row();
    });

    if changed {
        app.dirty.insert(ConfigFile::AssistRules);
    }
}
