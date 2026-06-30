use std::time::Duration;

use crate::{
    app::App,
    server_process::{self, ServerStatus},
    ui::theme,
};

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("topbar")
        .frame(
            egui::Frame::new()
                .fill(theme::DEEP)
                .inner_margin(egui::Margin::symmetric(14, 7))
                .stroke(egui::Stroke::new(1.0, theme::BORDER)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // ── Sidebar toggle ────────────────────────────────────────
                let toggle_label = if app.sidebar_open { "◀" } else { "▶" };
                if ui
                    .add(egui::Button::new(toggle_label).min_size(egui::vec2(24.0, 24.0)))
                    .on_hover_text(if app.sidebar_open { "Hide sidebar" } else { "Show sidebar" })
                    .clicked()
                {
                    app.sidebar_open = !app.sidebar_open;
                }
                ui.add_space(6.0);

                // ── Left: brand + folder breadcrumb ──────────────────────
                ui.label(
                    egui::RichText::new("APEX MANAGER")
                        .size(12.0)
                        .strong()
                        .color(egui::Color32::from_rgb(55, 75, 130)),
                );

                if let Some(ref dir) = app.server_dir.clone() {
                    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("server");
                    ui.label(egui::RichText::new("▸").color(theme::BORDER));
                    ui.label(egui::RichText::new(name).color(theme::MUTED));
                }

                // ── Right: server controls ────────────────────────────────
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let dir = app.server_dir.clone();
                    let Some(ref dir) = dir else {
                        ui.label(
                            egui::RichText::new("Open a server folder to get started")
                                .color(theme::MUTED)
                                .italics(),
                        );
                        return;
                    };

                    match app.server_status.clone() {
                        // ── Stopped ──────────────────────────────────────
                        ServerStatus::Stopped => {
                            let btn = egui::Button::new(
                                egui::RichText::new("▶  Start Server").size(14.0).strong().color(theme::TEXT),
                            )
                            .min_size(egui::vec2(155.0, 32.0))
                            .fill(egui::Color32::from_rgb(21, 128, 61));

                            if ui.add(btn).on_hover_text("Launch accServer.exe").clicked() {
                                match server_process::start(dir) {
                                    Ok(child) => {
                                        app.server_process = Some(child);
                                        app.server_status  = ServerStatus::Running;
                                        app.status_message = "Server started.".to_string();
                                    }
                                    Err(e) => app.status_message = format!("Failed to start: {e}"),
                                }
                            }
                        }

                        // ── Running ──────────────────────────────────────
                        ServerStatus::Running => {
                            ctx.request_repaint_after(Duration::from_secs(1));

                            // right_to_left: rightmost first in code
                            if ui
                                .add(
                                    egui::Button::new(egui::RichText::new("↺  Restart").size(13.0))
                                        .min_size(egui::vec2(95.0, 32.0)),
                                )
                                .clicked()
                            {
                                stop_server(app);
                                match server_process::start(dir) {
                                    Ok(child) => {
                                        app.server_process = Some(child);
                                        app.server_status  = ServerStatus::Running;
                                        app.status_message = "Server restarted.".to_string();
                                    }
                                    Err(e) => app.status_message = format!("Failed to restart: {e}"),
                                }
                            }

                            if ui
                                .add(
                                    egui::Button::new(egui::RichText::new("■  Stop").size(13.0).color(theme::TEXT))
                                        .min_size(egui::vec2(80.0, 32.0))
                                        .fill(egui::Color32::from_rgb(153, 27, 27)),
                                )
                                .clicked()
                            {
                                stop_server(app);
                            }

                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("●  RUNNING")
                                    .color(theme::GREEN)
                                    .size(14.0)
                                    .strong(),
                            );
                        }

                        // ── Crashed ───────────────────────────────────────
                        ServerStatus::Crashed(code) => {
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("▶  Restart").size(13.0).strong().color(theme::TEXT),
                                    )
                                    .min_size(egui::vec2(105.0, 32.0))
                                    .fill(egui::Color32::from_rgb(21, 128, 61)),
                                )
                                .clicked()
                            {
                                match server_process::start(dir) {
                                    Ok(child) => {
                                        app.server_process = Some(child);
                                        app.server_status  = ServerStatus::Running;
                                        app.status_message = "Server started.".to_string();
                                    }
                                    Err(e) => app.status_message = format!("Failed to start: {e}"),
                                }
                            }

                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new(format!("●  CRASHED ({})", code))
                                    .color(theme::RED)
                                    .size(14.0)
                                    .strong(),
                            );
                        }
                    }
                });
            });
        });
}

fn stop_server(app: &mut App) {
    if let Some(ref mut child) = app.server_process {
        let _ = child.kill();
        let _ = child.wait();
    }
    app.server_process = None;
    app.server_status  = ServerStatus::Stopped;
    app.status_message = "Server stopped.".to_string();
}
