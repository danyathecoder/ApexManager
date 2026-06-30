use crate::{
    app::{App, ConfigFile},
    config::entry_list::{Driver, Entry},
    data::{cars::CAR_MODELS, categories::DRIVER_CATEGORIES},
};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Entry List");
    ui.separator();

    let el = &mut app.entry_list;
    let mut changed = false;

    let mut force = el.force_entry_list != 0;
    if ui.checkbox(&mut force, "Force Entry List").changed() {
        el.force_entry_list = force as u8;
        changed = true;
    }

    ui.add_space(4.0);

    let mut to_remove: Option<usize> = None;
    let mut to_clone: Option<usize> = None;

    let entries_len = el.entries.len();
    for i in 0..entries_len {
        let entry = &mut el.entries[i];
        let header_label = format!(
            "#{} – {}",
            entry.race_number,
            car_name(entry.forced_car_model)
        );

        egui::CollapsingHeader::new(&header_label)
            .id_salt(format!("entry_{i}"))
            .show(ui, |ui| {
                egui::Grid::new(format!("entry_grid_{i}"))
                    .num_columns(2)
                    .spacing([12.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Race Number").on_hover_text("1–998 or -1 for any");
                        changed |= ui.add(egui::DragValue::new(&mut entry.race_number).range(-1..=998)).changed();
                        ui.end_row();

                        ui.label("Car Model");
                        let car_display = car_name(entry.forced_car_model);
                        egui::ComboBox::from_id_salt(format!("car_model_{i}"))
                            .selected_text(car_display)
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut entry.forced_car_model, -1, "Free (any)").changed() {
                                    changed = true;
                                }
                                for c in CAR_MODELS {
                                    if ui.selectable_value(&mut entry.forced_car_model, c.id as i32, &format!("{} [{}]", c.name, c.class)).changed() {
                                        changed = true;
                                    }
                                }
                            });
                        ui.end_row();

                        ui.label("Default Grid Position").on_hover_text("-1 = not set");
                        changed |= ui.add(egui::DragValue::new(&mut entry.default_grid_position).range(-1..=120)).changed();
                        ui.end_row();

                        ui.label("Ballast (kg)");
                        changed |= ui.add(egui::DragValue::new(&mut entry.ballast_kg).range(0..=40)).changed();
                        ui.end_row();

                        ui.label("Restrictor (%)");
                        changed |= ui.add(egui::DragValue::new(&mut entry.restrictor).range(0..=40)).changed();
                        ui.end_row();

                        let mut b = entry.override_driver_info != 0;
                        ui.label("Override Driver Info");
                        if ui.checkbox(&mut b, "").changed() { entry.override_driver_info = b as u8; changed = true; }
                        ui.end_row();

                        let mut b = entry.is_server_admin != 0;
                        ui.label("Is Server Admin");
                        if ui.checkbox(&mut b, "").changed() { entry.is_server_admin = b as u8; changed = true; }
                        ui.end_row();

                        ui.label("Custom Car");
                        changed |= ui.text_edit_singleline(&mut entry.custom_car).changed();
                        ui.end_row();
                    });

                ui.add_space(4.0);
                ui.label("Drivers:");

                let mut drv_to_remove: Option<usize> = None;
                for (j, drv) in entry.drivers.iter_mut().enumerate() {
                    ui.push_id(format!("drv_{i}_{j}"), |ui| {
                        egui::Grid::new(format!("drv_grid_{i}_{j}"))
                            .num_columns(2)
                            .spacing([8.0, 2.0])
                            .show(ui, |ui| {
                                ui.label("Steam ID").on_hover_text("Must start with 'S'");
                                let id_color = if !drv.player_id.starts_with('S') && !drv.player_id.is_empty() {
                                    egui::Color32::RED
                                } else {
                                    ui.visuals().text_color()
                                };
                                ui.add(egui::TextEdit::singleline(&mut drv.player_id).text_color(id_color));
                                ui.end_row();
                                ui.label("First Name");
                                changed |= ui.text_edit_singleline(&mut drv.first_name).changed();
                                ui.end_row();
                                ui.label("Last Name");
                                changed |= ui.text_edit_singleline(&mut drv.last_name).changed();
                                ui.end_row();
                                ui.label("Short Name");
                                changed |= ui.text_edit_singleline(&mut drv.short_name).changed();
                                ui.end_row();
                                ui.label("Category");
                                egui::ComboBox::from_id_salt(format!("drv_cat_{i}_{j}"))
                                    .selected_text(category_name(drv.driver_category))
                                    .show_ui(ui, |ui| {
                                        for &(id, name) in DRIVER_CATEGORIES {
                                            if ui.selectable_value(&mut drv.driver_category, id, name).changed() {
                                                changed = true;
                                            }
                                        }
                                    });
                                ui.end_row();
                            });
                        if ui.small_button(format!("Remove Driver {}", j + 1)).clicked() {
                            drv_to_remove = Some(j);
                        }
                    });
                    ui.separator();
                }
                if let Some(j) = drv_to_remove {
                    entry.drivers.remove(j);
                    changed = true;
                }
                if ui.small_button("+ Add Driver").clicked() {
                    entry.drivers.push(Driver::default());
                    changed = true;
                }

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui.small_button("Duplicate Entry").clicked() {
                        to_clone = Some(i);
                    }
                    if ui.small_button("Remove Entry").clicked() {
                        to_remove = Some(i);
                    }
                });
            });
    }

    if let Some(i) = to_clone {
        let cloned = el.entries[i].clone();
        el.entries.insert(i + 1, cloned);
        changed = true;
    }
    if let Some(i) = to_remove {
        el.entries.remove(i);
        changed = true;
    }

    ui.add_space(4.0);
    if ui.button("+ Add Entry").clicked() {
        el.entries.push(Entry::default());
        changed = true;
    }

    if changed {
        app.dirty.insert(ConfigFile::EntryList);
    }
}

fn car_name(id: i32) -> &'static str {
    if id == -1 {
        return "Free (any)";
    }
    CAR_MODELS
        .iter()
        .find(|c| c.id == id as u32)
        .map(|c| c.name)
        .unwrap_or("Unknown")
}

fn category_name(id: u8) -> &'static str {
    DRIVER_CATEGORIES
        .iter()
        .find(|&&(i, _)| i == id)
        .map(|&(_, n)| n)
        .unwrap_or("?")
}
