use crate::data::{cars::CAR_MODELS, categories::{CUP_CATEGORIES, DRIVER_CATEGORIES, SESSION_TYPES}, tracks::TRACKS};

#[derive(Default, PartialEq, Clone)]
enum Tab { #[default] Tracks, Cars, DriverCategories, CupCategories, SessionTypes }

pub fn show(_app: &mut crate::app::App, ui: &mut egui::Ui) {
    ui.heading("Reference");
    ui.separator();

    // Tab state lives in egui memory
    let tab = ui.memory_mut(|m| m.data.get_temp_mut_or_default::<Tab>(egui::Id::new("ref_tab")).clone());

    ui.horizontal(|ui| {
        if ui.selectable_label(tab == Tab::Tracks, "Tracks").clicked() {
            ui.memory_mut(|m| *m.data.get_temp_mut_or_default::<Tab>(egui::Id::new("ref_tab")) = Tab::Tracks);
        }
        if ui.selectable_label(tab == Tab::Cars, "Cars").clicked() {
            ui.memory_mut(|m| *m.data.get_temp_mut_or_default::<Tab>(egui::Id::new("ref_tab")) = Tab::Cars);
        }
        if ui.selectable_label(tab == Tab::DriverCategories, "Driver Categories").clicked() {
            ui.memory_mut(|m| *m.data.get_temp_mut_or_default::<Tab>(egui::Id::new("ref_tab")) = Tab::DriverCategories);
        }
        if ui.selectable_label(tab == Tab::CupCategories, "Cup Categories").clicked() {
            ui.memory_mut(|m| *m.data.get_temp_mut_or_default::<Tab>(egui::Id::new("ref_tab")) = Tab::CupCategories);
        }
        if ui.selectable_label(tab == Tab::SessionTypes, "Session Types").clicked() {
            ui.memory_mut(|m| *m.data.get_temp_mut_or_default::<Tab>(egui::Id::new("ref_tab")) = Tab::SessionTypes);
        }
    });

    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        match tab {
            Tab::Tracks => show_tracks(ui),
            Tab::Cars => show_cars(ui),
            Tab::DriverCategories => show_driver_cats(ui),
            Tab::CupCategories => show_cup_cats(ui),
            Tab::SessionTypes => show_session_types(ui),
        }
    });
}

fn show_tracks(ui: &mut egui::Ui) {
    egui::Grid::new("ref_tracks").num_columns(3).striped(true).spacing([16.0, 3.0]).show(ui, |ui| {
        ui.strong("Track Key");
        ui.strong("Pit Boxes");
        ui.strong("Private Slots");
        ui.end_row();
        for t in TRACKS {
            ui.label(t.key);
            ui.label(t.pit_boxes.to_string());
            ui.label(t.private_slots.to_string());
            ui.end_row();
        }
    });
}

fn show_cars(ui: &mut egui::Ui) {
    egui::Grid::new("ref_cars").num_columns(3).striped(true).spacing([16.0, 3.0]).show(ui, |ui| {
        ui.strong("ID");
        ui.strong("Name");
        ui.strong("Class");
        ui.end_row();
        for c in CAR_MODELS {
            ui.label(c.id.to_string());
            ui.label(c.name);
            ui.label(c.class);
            ui.end_row();
        }
    });
}

fn show_driver_cats(ui: &mut egui::Ui) {
    egui::Grid::new("ref_dcat").num_columns(2).striped(true).spacing([16.0, 3.0]).show(ui, |ui| {
        ui.strong("ID");
        ui.strong("Name");
        ui.end_row();
        for &(id, name) in DRIVER_CATEGORIES {
            ui.label(id.to_string());
            ui.label(name);
            ui.end_row();
        }
    });
}

fn show_cup_cats(ui: &mut egui::Ui) {
    egui::Grid::new("ref_ccat").num_columns(2).striped(true).spacing([16.0, 3.0]).show(ui, |ui| {
        ui.strong("ID");
        ui.strong("Name");
        ui.end_row();
        for &(id, name) in CUP_CATEGORIES {
            ui.label(id.to_string());
            ui.label(name);
            ui.end_row();
        }
    });
}

fn show_session_types(ui: &mut egui::Ui) {
    egui::Grid::new("ref_stypes").num_columns(2).striped(true).spacing([16.0, 3.0]).show(ui, |ui| {
        ui.strong("Key");
        ui.strong("Type");
        ui.end_row();
        for &(key, name) in SESSION_TYPES {
            ui.label(key);
            ui.label(name);
            ui.end_row();
        }
    });
}
