use crate::app::{App, ConfigFile};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Server Config");
    ui.separator();

    let cfg = &mut app.server_config;
    let mut changed = false;

    egui::Grid::new("server_config_grid")
        .num_columns(2)
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            ui.label("UDP Port").on_hover_text("UDP port for car data (default 9231)");
            changed |= ui.add(egui::DragValue::new(&mut cfg.udp_port).range(1..=65535)).changed();
            ui.end_row();

            ui.label("TCP Port").on_hover_text("TCP port for client connections (default 9232)");
            changed |= ui.add(egui::DragValue::new(&mut cfg.tcp_port).range(1..=65535)).changed();
            ui.end_row();

            ui.label("Max Connections").on_hover_text("Maximum simultaneous TCP connections");
            changed |= ui.add(egui::DragValue::new(&mut cfg.max_connections).range(1..=200)).changed();
            ui.end_row();

            ui.label("LAN Discovery").on_hover_text("Broadcast on LAN for auto-discovery");
            let mut lan = cfg.lan_discovery != 0;
            if ui.checkbox(&mut lan, "").changed() {
                cfg.lan_discovery = lan as u8;
                changed = true;
            }
            ui.end_row();

            ui.label("Register to Lobby").on_hover_text("List on the ACC public server list");
            let mut lobby = cfg.register_to_lobby != 0;
            if ui.checkbox(&mut lobby, "").changed() {
                cfg.register_to_lobby = lobby as u8;
                changed = true;
            }
            ui.end_row();

            ui.label("Public IP").on_hover_text("Override public IP (leave empty for auto-detect)");
            let mut ip = cfg.public_ip.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut ip).changed() {
                cfg.public_ip = if ip.is_empty() { None } else { Some(ip) };
                changed = true;
            }
            ui.end_row();

            ui.label("Config Version");
            changed |= ui.add(egui::DragValue::new(&mut cfg.config_version)).changed();
            ui.end_row();
        });

    if changed {
        app.dirty.insert(ConfigFile::ServerConfig);
    }
}
