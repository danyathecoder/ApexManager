use std::time::Duration;

use crate::{app::App, server_process::{self, ServerStatus}};

pub fn show(app: &mut App, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.heading("Server Control");
    ui.separator();

    let Some(ref dir) = app.server_dir.clone() else {
        ui.label("No server folder selected.");
        return;
    };

    let exe = server_process::exe_path(dir);
    ui.add(egui::Label::new(
        egui::RichText::new(exe.to_string_lossy().as_ref()).color(egui::Color32::GRAY).small(),
    ));

    ui.add_space(8.0);

    let (status_text, status_color) = match &app.server_status {
        ServerStatus::Stopped => ("STOPPED", egui::Color32::GRAY),
        ServerStatus::Running => ("RUNNING", egui::Color32::GREEN),
        ServerStatus::Crashed(code) => {
            let s = format!("CRASHED (exit {})", code);
            // Use a static-ish approach — we format dynamically and use as_str
            return show_with_crash(app, ctx, ui, &s, dir);
        }
    };

    ui.add(
        egui::Label::new(egui::RichText::new(status_text).color(status_color).size(20.0).strong()),
    );

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let is_running = matches!(app.server_status, ServerStatus::Running);

        if ui.add_enabled(!is_running, egui::Button::new("Start")).clicked() {
            match server_process::start(dir) {
                Ok(child) => {
                    app.server_process = Some(child);
                    app.server_status = ServerStatus::Running;
                    app.status_message = "Server started.".to_string();
                }
                Err(e) => {
                    app.status_message = format!("Failed to start: {e}");
                }
            }
        }

        if ui.add_enabled(is_running, egui::Button::new("Stop")).clicked() {
            stop_server(app);
        }

        if ui.button("Restart").clicked() {
            stop_server(app);
            let dir = dir.clone();
            match server_process::start(&dir) {
                Ok(child) => {
                    app.server_process = Some(child);
                    app.server_status = ServerStatus::Running;
                    app.status_message = "Server restarted.".to_string();
                }
                Err(e) => {
                    app.status_message = format!("Failed to restart: {e}");
                }
            }
        }
    });

    ui.add_space(12.0);
    let save_restart_label = if app.dirty.is_empty() {
        "Save All & Restart"
    } else {
        "Save All & Restart ●"
    };
    if ui.button(save_restart_label).clicked() {
        app.save_all();
        stop_server(app);
        let dir = dir.clone();
        match server_process::start(&dir) {
            Ok(child) => {
                app.server_process = Some(child);
                app.server_status = ServerStatus::Running;
                app.status_message = "Saved and restarted.".to_string();
            }
            Err(e) => {
                app.status_message = format!("Failed to start after save: {e}");
            }
        }
    }

    if matches!(app.server_status, ServerStatus::Running) {
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

fn show_with_crash(app: &mut App, ctx: &egui::Context, ui: &mut egui::Ui, status: &str, dir: &std::path::Path) {
    ui.add(egui::Label::new(
        egui::RichText::new(status).color(egui::Color32::RED).size(20.0).strong(),
    ));
    ui.label("Check the Log Viewer for details.");
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        if ui.button("Start").clicked() {
            match server_process::start(dir) {
                Ok(child) => {
                    app.server_process = Some(child);
                    app.server_status = ServerStatus::Running;
                    app.status_message = "Server started.".to_string();
                }
                Err(e) => {
                    app.status_message = format!("Failed to start: {e}");
                }
            }
        }
    });
    ctx.request_repaint_after(Duration::from_secs(1));
}

fn stop_server(app: &mut App) {
    if let Some(ref mut child) = app.server_process {
        let _ = child.kill();
        let _ = child.wait();
    }
    app.server_process = None;
    app.server_status = ServerStatus::Stopped;
    app.status_message = "Server stopped.".to_string();
}
