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
    ServerControl,
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
                            app.load_all(&dir);
                            app.server_dir = Some(dir);
                        }
                    }
                }
            }
        }
        app
    }

    pub fn open_folder(&mut self, dir: PathBuf) {
        if let Some(ref mut child) = self.server_process {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.server_process = None;
        self.server_status = ServerStatus::Stopped;
        self.load_all(&dir);
        self.server_dir = Some(dir);
        self.dirty.clear();
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
        self.status_message = format!("Loaded from {}", dir.display());
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

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(until) = self.toast_until {
                    if Instant::now() < until {
                        ui.label(&self.status_message.clone());
                        ctx.request_repaint_after(Duration::from_millis(100));
                    } else {
                        self.toast_until = None;
                        ui.label(&self.status_message.clone());
                    }
                } else {
                    ui.label(&self.status_message.clone());
                }
                if !self.dirty.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{} unsaved change(s)", self.dirty.len()));
                    });
                }
            });
        });

        crate::ui::sidebar::show(self, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.server_dir.is_none()
                && !matches!(self.nav, NavSection::Reference)
            {
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);
                    ui.heading("No server folder selected");
                    ui.add_space(8.0);
                    ui.label("Click \"Open Folder\" in the sidebar and select the server/ subfolder of your ACC server installation.");
                    ui.add_space(4.0);
                    ui.label("This folder must contain accServer.exe and a cfg/ subdirectory with the JSON config files.");
                    ui.add_space(4.0);
                    ui.weak("Default Steam path:  …\\Assetto Corsa Competizione Dedicated Server\\server\\");
                });
                return;
            }
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
                NavSection::ServerControl => crate::ui::process::show(self, ctx, ui),
                NavSection::LogViewer => crate::ui::log_viewer::show(self, ctx, ui),
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
