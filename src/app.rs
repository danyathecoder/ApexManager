use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

use crate::{
    config::{
        assist_rules::AssistRules, bop::Bop, entry_list::EntryList, event::Event,
        event_rules::EventRules, results::SessionResult, server_config::ServerConfig,
        settings::Settings,
    },
    server_process::ServerStatus,
    util::io,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigFile {
    ServerConfig,
    Settings,
    Event,
    EventRules,
    AssistRules,
    EntryList,
    Bop,
}

#[derive(Default, PartialEq, Clone)]
pub enum NavSection {
    #[default]
    ServerConfig,
    Settings,
    Event,
    EventRules,
    AssistRules,
    EntryList,
    Bop,
    AdminCommands,
    Results,
    Reference,

    LogViewer,
}

#[derive(Serialize, Deserialize, Default)]
struct AppPersist {
    last_server_dir: Option<PathBuf>,
}

pub struct App {
    pub server_dir: Option<PathBuf>,
    pub nav: NavSection,

    pub server_config: ServerConfig,
    pub settings: Settings,
    pub event: Event,
    pub event_rules: EventRules,
    pub assist_rules: AssistRules,
    pub entry_list: EntryList,
    pub bop: Bop,

    pub dirty: HashSet<ConfigFile>,
    pub status_message: String,
    pub toast_until: Option<Instant>,

    pub results_files: Vec<PathBuf>,
    pub selected_result: Option<SessionResult>,
    pub results_filter: String,

    pub server_process: Option<std::process::Child>,
    pub server_status: ServerStatus,

    pub log_files: Vec<PathBuf>,
    pub selected_log: Option<PathBuf>,
    pub log_content: String,
    pub log_auto_scroll: bool,

    pub admin_car_number: u32,

    pub presets: Vec<String>,
    pub selected_preset: Option<String>,
    pub preset_name: String,

    pub sidebar_open: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            server_dir: None,
            nav: NavSection::default(),
            server_config: ServerConfig::default(),
            settings: Settings::default(),
            event: Event::default(),
            event_rules: EventRules::default(),
            assist_rules: AssistRules::default(),
            entry_list: EntryList::default(),
            bop: Bop::default(),
            dirty: HashSet::new(),
            status_message: String::new(),
            toast_until: None,
            results_files: Vec::new(),
            selected_result: None,
            results_filter: String::new(),
            server_process: None,
            server_status: ServerStatus::Stopped,
            log_files: Vec::new(),
            selected_log: None,
            log_content: String::new(),
            log_auto_scroll: true,
            admin_car_number: 1,
            presets: Vec::new(),
            selected_preset: None,
            preset_name: String::new(),
            sidebar_open: true,
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let mut app = App::default();
        if let Some(path) = persist_path() {
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(p) = serde_json::from_str::<AppPersist>(&text) {
                    if let Some(dir) = p.last_server_dir {
                        if dir.exists() {
                            app.open_folder(dir);
                        }
                    }
                }
            }
        }
        app.scan_presets();
        app
    }

    pub fn open_folder(&mut self, dir: PathBuf) {
        if !crate::server_process::exe_path(&dir).exists() {
            self.status_message = format!(
                "accServer.exe not found in {}  — did you select the server/ subfolder?",
                dir.display()
            );
            return;
        }
        if let Some(ref mut child) = self.server_process {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.server_process = None;
        self.server_status = ServerStatus::Stopped;
        self.load_all(&dir);
        self.server_dir = Some(dir);

        // If the loaded config looks like ACC's untouched install template
        // (udpPort == 0 or 1), auto-apply our sensible defaults so the user
        // doesn't have to fill every field from scratch.
        if self.server_config.udp_port <= 1 {
            self.reset_to_defaults();
            self.status_message =
                "Fresh config detected — defaults applied. Review and click Save All.".to_string();
        } else {
            self.dirty.clear();
            self.status_message = format!("Loaded from {}", self.server_dir.as_ref().unwrap().display());
        }

        self.save_persist();
    }

    pub fn load_all(&mut self, dir: &Path) {
        let cfg = dir.join("cfg");
        self.server_config = load_or_default(&cfg, crate::config::server_config::FILENAME);
        self.settings = load_or_default(&cfg, crate::config::settings::FILENAME);
        self.event = load_or_default(&cfg, crate::config::event::FILENAME);
        self.event_rules = load_or_default(&cfg, crate::config::event_rules::FILENAME);
        self.assist_rules = load_or_default(&cfg, crate::config::assist_rules::FILENAME);
        self.entry_list = load_or_default(&cfg, crate::config::entry_list::FILENAME);
        self.bop = load_or_default(&cfg, crate::config::bop::FILENAME);
    }

    pub fn reset_to_defaults(&mut self) {
        self.server_config = ServerConfig::default();
        self.settings = Settings::default();
        self.event = Event::default();
        self.event_rules = EventRules::default();
        self.assist_rules = AssistRules::default();
        self.entry_list = EntryList::default();
        self.bop = Bop::default();
        self.dirty.extend([
            ConfigFile::ServerConfig,
            ConfigFile::Settings,
            ConfigFile::Event,
            ConfigFile::EventRules,
            ConfigFile::AssistRules,
            ConfigFile::EntryList,
            ConfigFile::Bop,
        ]);
        self.status_message = "Reset to defaults — click Save All to write files.".to_string();
    }

    pub fn save_all(&mut self) {
        let dirty: Vec<ConfigFile> = self.dirty.iter().copied().collect();
        for file in dirty {
            self.save_one(file);
        }
    }

    pub fn save_one(&mut self, file: ConfigFile) {
        let Some(ref dir) = self.server_dir.clone() else { return };
        let cfg = dir.join("cfg");
        if let Err(e) = std::fs::create_dir_all(&cfg) {
            self.status_message = format!("Error creating cfg dir: {e}");
            return;
        }
        let result = match file {
            ConfigFile::ServerConfig => io::save_json(
                &cfg.join(crate::config::server_config::FILENAME),
                &self.server_config,
            ),
            ConfigFile::Settings => io::save_json(
                &cfg.join(crate::config::settings::FILENAME),
                &self.settings,
            ),
            ConfigFile::Event => io::save_json(
                &cfg.join(crate::config::event::FILENAME),
                &self.event,
            ),
            ConfigFile::EventRules => io::save_json(
                &cfg.join(crate::config::event_rules::FILENAME),
                &self.event_rules,
            ),
            ConfigFile::AssistRules => io::save_json(
                &cfg.join(crate::config::assist_rules::FILENAME),
                &self.assist_rules,
            ),
            ConfigFile::EntryList => io::save_json(
                &cfg.join(crate::config::entry_list::FILENAME),
                &self.entry_list,
            ),
            ConfigFile::Bop => io::save_json(
                &cfg.join(crate::config::bop::FILENAME),
                &self.bop,
            ),
        };
        match result {
            Ok(()) => {
                self.dirty.remove(&file);
                self.status_message = format!("Saved {:?}", file);
            }
            Err(e) => {
                self.status_message = format!("Error saving {:?}: {e}", file);
            }
        }
    }

    pub fn scan_presets(&mut self) {
        self.presets = crate::config::preset::list_presets();
    }

    pub fn save_current_preset(&mut self) {
        let name = self.preset_name.trim().to_string();
        if name.is_empty() {
            return;
        }
        let preset = crate::config::preset::Preset {
            server_config: self.server_config.clone(),
            settings: self.settings.clone(),
            event: self.event.clone(),
            event_rules: self.event_rules.clone(),
            assist_rules: self.assist_rules.clone(),
            entry_list: self.entry_list.clone(),
            bop: self.bop.clone(),
        };
        match crate::config::preset::save_preset(&name, &preset) {
            Ok(()) => {
                self.scan_presets();
                self.selected_preset = Some(name.clone());
                self.set_toast(&format!("Preset '{name}' saved."));
            }
            Err(e) => self.status_message = format!("Failed to save preset: {e}"),
        }
    }

    pub fn load_selected_preset(&mut self) {
        let Some(name) = self.selected_preset.clone() else { return };
        match crate::config::preset::load_preset(&name) {
            Ok(preset) => {
                self.server_config = preset.server_config;
                self.settings = preset.settings;
                self.event = preset.event;
                self.event_rules = preset.event_rules;
                self.assist_rules = preset.assist_rules;
                self.entry_list = preset.entry_list;
                self.bop = preset.bop;
                self.dirty.extend([
                    ConfigFile::ServerConfig,
                    ConfigFile::Settings,
                    ConfigFile::Event,
                    ConfigFile::EventRules,
                    ConfigFile::AssistRules,
                    ConfigFile::EntryList,
                    ConfigFile::Bop,
                ]);
                self.set_toast(&format!("Preset '{name}' loaded — click Save All to write to disk."));
            }
            Err(e) => self.status_message = format!("Failed to load preset '{name}': {e}"),
        }
    }

    pub fn delete_selected_preset(&mut self) {
        let Some(name) = self.selected_preset.clone() else { return };
        match crate::config::preset::delete_preset(&name) {
            Ok(()) => {
                self.scan_presets();
                self.selected_preset = None;
                self.set_toast("Preset deleted.");
            }
            Err(e) => self.status_message = format!("Failed to delete preset: {e}"),
        }
    }

    pub fn set_toast(&mut self, msg: &str) {
        self.status_message = msg.to_string();
        self.toast_until = Some(Instant::now() + Duration::from_secs(2));
    }

    pub fn poll_server_process(&mut self) {
        if let Some(ref mut child) = self.server_process {
            let status = crate::server_process::poll_status(child);
            if matches!(status, ServerStatus::Crashed(_)) {
                let _ = child.wait();
                self.server_status = status;
                self.server_process = None;
            } else {
                self.server_status = status;
            }
        }
    }

    pub fn scan_results(&mut self) {
        let Some(ref dir) = self.server_dir else { return };
        let results_dir = dir.join("results");
        self.results_files = scan_dir_for_ext(&results_dir, "json");
        self.results_files.sort_by(|a, b| b.cmp(a));
    }

    pub fn scan_logs(&mut self) {
        let Some(ref dir) = self.server_dir else { return };
        let log_dir = dir.join("log");
        self.log_files = scan_dir_for_ext(&log_dir, "log");
        self.log_files.sort_by(|a, b| {
            let ma = std::fs::metadata(a).and_then(|m| m.modified()).ok();
            let mb = std::fs::metadata(b).and_then(|m| m.modified()).ok();
            mb.cmp(&ma)
        });
    }

    pub fn load_log(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.log_content = content;
                self.selected_log = Some(path);
            }
            Err(e) => {
                self.status_message = format!("Error reading log: {e}");
            }
        }
    }

    fn save_persist(&self) {
        let p = AppPersist { last_server_dir: self.server_dir.clone() };
        if let Some(path) = persist_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(text) = serde_json::to_string_pretty(&p) {
                let _ = std::fs::write(path, text);
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_server_process();

        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::new()
                    .fill(crate::ui::theme::DEEP)
                    .inner_margin(egui::Margin::symmetric(12, 5))
                    .stroke(egui::Stroke::new(1.0, crate::ui::theme::BORDER)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let msg = self.status_message.clone();
                    if let Some(until) = self.toast_until {
                        if Instant::now() < until {
                            ui.label(egui::RichText::new(&msg).color(crate::ui::theme::PRIMARY));
                            ctx.request_repaint_after(Duration::from_millis(100));
                        } else {
                            self.toast_until = None;
                            ui.label(egui::RichText::new(&msg).color(crate::ui::theme::MUTED));
                        }
                    } else {
                        ui.label(egui::RichText::new(&msg).color(crate::ui::theme::MUTED));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (status_text, status_color) = match &self.server_status {
                            ServerStatus::Stopped  => ("● Stopped".to_string(),        crate::ui::theme::MUTED),
                            ServerStatus::Running  => ("● Running".to_string(),         crate::ui::theme::GREEN),
                            ServerStatus::Crashed(c) => (format!("● Crashed ({})", c), crate::ui::theme::RED),
                        };
                        ui.label(egui::RichText::new(status_text).color(status_color).size(12.0));

                        if !self.dirty.is_empty() {
                            ui.add_space(12.0);
                            ui.label(
                                egui::RichText::new(format!("● {} unsaved", self.dirty.len()))
                                    .color(crate::ui::theme::AMBER)
                                    .size(12.0),
                            );
                        }
                    });
                });
            });

        crate::ui::topbar::show(self, ctx);
        crate::ui::sidebar::show(self, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.server_dir.is_none()
                && !matches!(self.nav, NavSection::Reference)
            {
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);
                    ui.heading("No server folder selected");
                    ui.add_space(8.0);
                    ui.label("Click \"Open Folder\" in the sidebar and select the server/ subfolder — not the root.");
                    ui.add_space(4.0);
                    ui.label("The correct folder contains accServer.exe directly inside it, along with cfg/, log/, and results/.");
                    ui.add_space(4.0);
                    ui.weak("Default Steam path:");
                    ui.weak("…\\Assetto Corsa Competizione Dedicated Server\\server\\");
                    ui.add_space(6.0);
                    ui.colored_label(egui::Color32::YELLOW, "⚠ Do not select the parent folder — it looks similar but the configs are in the wrong place.");
                });
                return;
            }

            // Log viewer and results handle their own internal layout (split panels / scroll)
            let needs_scroll = !matches!(
                self.nav,
                NavSection::LogViewer | NavSection::Results | NavSection::EntryList
            );

            let mut show_inner = |ui: &mut egui::Ui| {
                match self.nav.clone() {
                    NavSection::ServerConfig => crate::ui::server_config::show(self, ui),
                    NavSection::Settings => crate::ui::settings::show(self, ui),
                    NavSection::Event => crate::ui::event::show(self, ui),
                    NavSection::EventRules => crate::ui::event_rules::show(self, ui),
                    NavSection::AssistRules => crate::ui::assist_rules::show(self, ui),
                    NavSection::EntryList => crate::ui::entry_list::show(self, ui),
                    NavSection::Bop => crate::ui::bop::show(self, ui),
                    NavSection::AdminCommands => crate::ui::admin::show(self, ctx, ui),
                    NavSection::Results => crate::ui::results::show(self, ui),
                    NavSection::Reference => crate::ui::reference::show(self, ui),
                    NavSection::LogViewer => crate::ui::log_viewer::show(self, ctx, ui),
                }
            };

            if needs_scroll {
                egui::ScrollArea::vertical()
                    .id_salt("central_scroll")
                    .auto_shrink([false, false])
                    .show(ui, show_inner);
            } else {
                show_inner(ui);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_persist();
    }
}

fn load_or_default<T: serde::de::DeserializeOwned + Default>(dir: &Path, filename: &str) -> T {
    let path = dir.join(filename);
    if path.exists() {
        io::load_json(&path).unwrap_or_default()
    } else {
        T::default()
    }
}

fn scan_dir_for_ext(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some(ext))
        .collect()
}

fn persist_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("ApexManager").join("config.json"))
}
