use crate::app::{App, ConfigFile, NavSection};
use crate::ui::{theme, widgets::section_label};

const SIDEBAR_W: f32 = 205.0;

pub fn show(app: &mut App, ctx: &egui::Context) {
    if !app.sidebar_open {
        return;
    }

    egui::SidePanel::left("sidebar")
        .default_width(SIDEBAR_W)
        .width_range(160.0..=360.0)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(10, 15, 32))
                .inner_margin(egui::Margin::symmetric(8, 8))
                .stroke(egui::Stroke::new(1.0, theme::BORDER)),
        )
        .show(ctx, |ui| {
            // Dynamic button width based on current sidebar size
            let btn_w = ui.available_width();
            // ── Folder header ─────────────────────────────────────────────
            if let Some(ref dir) = app.server_dir {
                let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("server");
                ui.label(
                    egui::RichText::new(format!("▸ {name}"))
                        .color(theme::PRIMARY)
                        .size(12.0)
                        .strong(),
                );
            } else {
                ui.label(
                    egui::RichText::new("No folder open")
                        .color(theme::MUTED)
                        .size(11.5),
                );
            }

            ui.add_space(6.0);

            // ── Folder actions ────────────────────────────────────────────
            ui.horizontal(|ui| {
                if ui.add_sized([90.0, 22.0], egui::Button::new("Open Folder…")).clicked() {
                    if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                        app.open_folder(dir);
                    }
                }
                if ui
                    .add_enabled(
                        app.server_dir.is_some(),
                        egui::Button::new("New Config").min_size(egui::vec2(80.0, 22.0)),
                    )
                    .on_hover_text("Reset all settings to sensible defaults (does not overwrite until Save All)")
                    .clicked()
                {
                    app.reset_to_defaults();
                }
            });

            ui.add_space(6.0);
            ui.separator();

            // ── Navigation ────────────────────────────────────────────────
            section_label(ui, "CONFIGURATION");
            nav_btn(ui, app, NavSection::ServerConfig, "Server Config",  Some(ConfigFile::ServerConfig), btn_w);
            nav_btn(ui, app, NavSection::Settings,     "Settings",       Some(ConfigFile::Settings),     btn_w);
            nav_btn(ui, app, NavSection::Event,        "Event",          Some(ConfigFile::Event),        btn_w);
            nav_btn(ui, app, NavSection::EventRules,   "Event Rules",    Some(ConfigFile::EventRules),   btn_w);
            nav_btn(ui, app, NavSection::AssistRules,  "Assist Rules",   Some(ConfigFile::AssistRules),  btn_w);
            nav_btn(ui, app, NavSection::EntryList,    "Entry List",     Some(ConfigFile::EntryList),    btn_w);
            nav_btn(ui, app, NavSection::Bop,          "BoP",            Some(ConfigFile::Bop),          btn_w);

            ui.add_space(2.0);
            section_label(ui, "TOOLS");
            nav_btn(ui, app, NavSection::AdminCommands, "Admin Commands", None, btn_w);
            nav_btn(ui, app, NavSection::LogViewer,     "Log Viewer",     None, btn_w);
            nav_btn(ui, app, NavSection::Results,       "Results",        None, btn_w);
            nav_btn(ui, app, NavSection::Reference,     "Reference",      None, btn_w);

            ui.separator();

            // ── Presets ───────────────────────────────────────────────────
            section_label(ui, "PRESETS");

            let preset_label = app
                .selected_preset
                .clone()
                .unwrap_or_else(|| "— select —".to_string());

            egui::ComboBox::from_id_salt("preset_select")
                .selected_text(&preset_label)
                .width(btn_w)
                .show_ui(ui, |ui| {
                    for name in app.presets.clone() {
                        let active = app.selected_preset.as_deref() == Some(&name);
                        if ui.selectable_label(active, &name).clicked() {
                            app.selected_preset = Some(name);
                        }
                    }
                    if app.presets.is_empty() {
                        ui.label(
                            egui::RichText::new("No presets yet")
                                .color(theme::MUTED)
                                .italics(),
                        );
                    }
                });

            ui.horizontal(|ui| {
                let has = app.selected_preset.is_some();
                if ui.add_enabled(has, egui::Button::new("Load").min_size(egui::vec2(58.0, 20.0))).clicked() {
                    app.load_selected_preset();
                }
                if ui
                    .add_enabled(has, egui::Button::new("Delete").min_size(egui::vec2(58.0, 20.0)))
                    .on_hover_text("Permanently delete this preset file")
                    .clicked()
                {
                    app.delete_selected_preset();
                }
            });

            ui.add_space(4.0);
            ui.label(egui::RichText::new("Save current config as:").size(11.0).color(theme::MUTED));
            ui.add_sized([btn_w, 20.0], egui::TextEdit::singleline(&mut app.preset_name).hint_text("Preset name…"));
            if ui
                .add_enabled(
                    !app.preset_name.trim().is_empty(),
                    egui::Button::new("Save as Preset").min_size(egui::vec2(btn_w, 22.0)),
                )
                .clicked()
            {
                app.save_current_preset();
            }

            // ── Save All (pinned to bottom) ───────────────────────────────
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(4.0);
                let dirty = app.dirty.len();
                let label = if dirty == 0 {
                    "Save All".to_string()
                } else {
                    format!("Save All  ({dirty} unsaved)")
                };
                if ui
                    .add_enabled(dirty > 0, egui::Button::new(label).min_size(egui::vec2(btn_w, 26.0)))
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
    file: Option<ConfigFile>,
    btn_w: f32,
) {
    let is_active = app.nav == section;
    let is_dirty  = file.map(|f| app.dirty.contains(&f)).unwrap_or(false);

    let text = if is_dirty {
        egui::RichText::new(format!("● {label}")).color(theme::AMBER)
    } else {
        egui::RichText::new(label).color(theme::TEXT)
    };

    let btn = egui::Button::new(text).selected(is_active).min_size(egui::vec2(btn_w, 22.0));
    if ui.add(btn).clicked() {
        app.nav = section;
    }
}
