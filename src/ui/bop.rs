use crate::{
    app::{App, ConfigFile},
    config::bop::BopEntry,
    data::{cars::CAR_MODELS, tracks::TRACKS},
};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Balance of Performance");
    ui.separator();

    let bop = &mut app.bop;
    let mut changed = false;
    let mut to_remove: Option<usize> = None;

    egui::Grid::new("bop_grid")
        .num_columns(5)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.strong("Track");
            ui.strong("Car Model");
            ui.strong("Ballast (kg)");
            ui.strong("Restrictor (%)");
            ui.strong("");
            ui.end_row();

            for (i, entry) in bop.entries.iter_mut().enumerate() {
                egui::ComboBox::from_id_salt(format!("bop_track_{i}"))
                    .selected_text(&entry.track)
                    .width(140.0)
                    .show_ui(ui, |ui| {
                        for t in TRACKS {
                            if ui.selectable_value(&mut entry.track, t.key.to_string(), t.key).changed() {
                                changed = true;
                            }
                        }
                    });

                let car_label = CAR_MODELS
                    .iter()
                    .find(|c| c.id == entry.car_model)
                    .map(|c| c.name)
                    .unwrap_or("?");
                egui::ComboBox::from_id_salt(format!("bop_car_{i}"))
                    .selected_text(car_label)
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        for c in CAR_MODELS {
                            if ui.selectable_value(&mut entry.car_model, c.id, c.name).changed() {
                                changed = true;
                            }
                        }
                    });

                changed |= ui.add(egui::DragValue::new(&mut entry.ballast_kg).range(0..=40)).changed();
                changed |= ui.add(egui::DragValue::new(&mut entry.restrictor).range(0..=40)).changed();

                if ui.small_button("✕").clicked() {
                    to_remove = Some(i);
                }
                ui.end_row();
            }
        });

    if let Some(i) = to_remove {
        bop.entries.remove(i);
        changed = true;
    }

    if ui.button("+ Add Entry").clicked() {
        bop.entries.push(BopEntry::default());
        changed = true;
    }

    if changed {
        app.dirty.insert(ConfigFile::Bop);
    }
}
