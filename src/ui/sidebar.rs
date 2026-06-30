use crate::app::{App, ConfigFile, NavSection};

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("sidebar").exact_width(170.0).show(ctx, |ui| {
        ui.add_space(4.0);

        if ui.button("Open Folder").clicked() {
            if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                app.open_folder(dir);
            }
        }

        ui.separator();
        ui.small("Configuration");

        nav_btn(ui, app, NavSection::ServerConfig, "Server Config", ConfigFile::ServerConfig);
        nav_btn(ui, app, NavSection::Settings, "Settings", ConfigFile::Settings);
        nav_btn(ui, app, NavSection::Event, "Event", ConfigFile::Event);
        nav_btn(ui, app, NavSection::EventRules, "Event Rules", ConfigFile::EventRules);
        nav_btn(ui, app, NavSection::AssistRules, "Assist Rules", ConfigFile::AssistRules);
        nav_btn(ui, app, NavSection::EntryList, "Entry List", ConfigFile::EntryList);
        nav_btn(ui, app, NavSection::Bop, "BoP", ConfigFile::Bop);

        ui.separator();
        ui.small("Tools");

        plain_nav_btn(ui, app, NavSection::AdminCommands, "Admin Commands");
        plain_nav_btn(ui, app, NavSection::Results, "Results");
        plain_nav_btn(ui, app, NavSection::Reference, "Reference");
        plain_nav_btn(ui, app, NavSection::ServerControl, "Server Control");
        plain_nav_btn(ui, app, NavSection::LogViewer, "Log Viewer");

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.add_space(4.0);
            if ui
                .add_enabled(!app.dirty.is_empty(), egui::Button::new("Save All"))
                .clicked()
            {
                app.save_all();
            }
        });
    });
}

fn nav_btn(
    ui: &mut egui::Ui,
    app: &mut App,
    section: NavSection,
    label: &str,
    file: ConfigFile,
) {
    let is_active = app.nav == section;
    let is_dirty = app.dirty.contains(&file);
    let display = if is_dirty {
        format!("• {label}")
    } else {
        label.to_string()
    };
    let btn = egui::Button::new(&display).selected(is_active);
    if ui.add_sized([160.0, 22.0], btn).clicked() {
        app.nav = section;
    }
}

fn plain_nav_btn(ui: &mut egui::Ui, app: &mut App, section: NavSection, label: &str) {
    let is_active = app.nav == section;
    let btn = egui::Button::new(label).selected(is_active);
    if ui.add_sized([160.0, 22.0], btn).clicked() {
        app.nav = section;
    }
}
