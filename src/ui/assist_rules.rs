use crate::app::{App, ConfigFile};
use crate::ui::widgets::{help_panel, help_row};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Assist Rules");
    ui.add_space(4.0);

    help_panel(ui, "assist_rules", |ui| {
        help_row(ui, "Stability Control Max", "0 = stability control fully disabled for all. 100 = each player can choose freely (0–100%). Set to 0 for serious/competitive events.");
        help_row(ui, "Disable Auto Steer",    "When checked, forces auto-steer OFF for all drivers regardless of their personal settings.");
        help_row(ui, "Disable Auto Lights",   "Forces automatic lights management OFF — drivers must use lights manually.");
        help_row(ui, "Disable Auto Wiper",    "Forces automatic wipers OFF — drivers must activate wipers manually in rain.");
        help_row(ui, "Disable Auto Engine",   "Forces auto engine start/stop OFF — drivers must manage the engine themselves.");
        help_row(ui, "Disable Pit Limiter",   "Forces automatic pit lane speed limiter OFF — drivers must press the button.");
        help_row(ui, "Disable Auto Gear",     "Forces automatic gearbox OFF — drivers must shift manually.");
        help_row(ui, "Disable Auto Clutch",   "Forces automatic clutch OFF — drivers must use the clutch pedal.");
        help_row(ui, "Disable Ideal Line",    "Hides the driving line aid for all drivers.");
    });

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
