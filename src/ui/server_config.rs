use crate::app::{App, ConfigFile};
use crate::ui::widgets::{card, help_panel, help_row};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Server Config");
    ui.add_space(4.0);

    help_panel(ui, "server_config", |ui| {
        help_row(ui, "UDP Port",          "Car telemetry data port. Must be forwarded in your router. Default: 9231.");
        help_row(ui, "TCP Port",          "Client connection port. Must be forwarded in your router. Default: 9323.");
        help_row(ui, "Max Connections",   "Simultaneous TCP connections allowed. Match or exceed Max Car Slots.");
        help_row(ui, "Public IP",         "Leave blank — the server auto-detects it. Only set if behind multi-homed NAT.");
        help_row(ui, "LAN Discovery",     "Broadcasts server presence on local network. Safe to leave on.");
        help_row(ui, "Register to Lobby", "Lists server on the public ACC server browser. Requires internet access.");
    });

    ui.add_space(4.0);

    let cfg = &mut app.server_config;
    let mut changed = false;

    card(ui, "NETWORK", |ui| {
        egui::Grid::new("sc_network").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
            ui.label("UDP Port").on_hover_text("Car-data port (default 9231)");
            changed |= ui.add(egui::DragValue::new(&mut cfg.udp_port).range(1..=65535)).changed();
            ui.end_row();

            ui.label("TCP Port").on_hover_text("Client connection port (default 9323)");
            changed |= ui.add(egui::DragValue::new(&mut cfg.tcp_port).range(1..=65535)).changed();
            ui.end_row();

            ui.label("Max Connections").on_hover_text("Maximum simultaneous TCP connections");
            changed |= ui.add(egui::DragValue::new(&mut cfg.max_connections).range(1..=200)).changed();
            ui.end_row();

            ui.label("Public IP").on_hover_text("Override public IP (leave blank for auto-detect)");
            let mut ip = cfg.public_ip.clone().unwrap_or_default();
            let r = ui.add(egui::TextEdit::singleline(&mut ip).hint_text("auto"));
            if r.changed() {
                cfg.public_ip = if ip.is_empty() { None } else { Some(ip) };
                changed = true;
            }
            ui.end_row();
        });
    });

    card(ui, "DISCOVERY", |ui| {
        egui::Grid::new("sc_discovery").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
            ui.label("LAN Discovery").on_hover_text("Broadcast on LAN for auto-discovery");
            let mut v = cfg.lan_discovery != 0;
            if ui.checkbox(&mut v, "").changed() { cfg.lan_discovery = v as u8; changed = true; }
            ui.end_row();

            ui.label("Register to Lobby").on_hover_text("List server on the public ACC server browser");
            let mut v = cfg.register_to_lobby != 0;
            if ui.checkbox(&mut v, "").changed() { cfg.register_to_lobby = v as u8; changed = true; }
            ui.end_row();
        });
    });

    card(ui, "MISC", |ui| {
        egui::Grid::new("sc_misc").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
            ui.label("Config Version");
            changed |= ui.add(egui::DragValue::new(&mut cfg.config_version)).changed();
            ui.end_row();
        });
    });

    if changed {
        app.dirty.insert(ConfigFile::ServerConfig);
    }
}
