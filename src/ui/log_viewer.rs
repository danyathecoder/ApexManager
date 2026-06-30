use std::time::Duration;

use crate::app::App;

pub fn show(app: &mut App, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.heading("Log Viewer");
    ui.separator();

    if app.log_files.is_empty() {
        app.scan_logs();
    }

    ui.horizontal(|ui| {
        ui.checkbox(&mut app.log_auto_scroll, "Auto-scroll");
        if ui.button("Refresh").clicked() {
            app.scan_logs();
            if let Some(path) = app.selected_log.clone() {
                app.load_log(path);
            }
        }
    });
    ui.add_space(4.0);

    let log_files = app.log_files.clone();

    egui::SidePanel::left("log_files_panel").exact_width(200.0).show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for path in &log_files {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                let selected = app.selected_log.as_deref() == Some(path.as_path());
                if ui.selectable_label(selected, name).clicked() {
                    app.load_log(path.clone());
                }
            }
        });
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        if app.selected_log.is_none() {
            ui.centered_and_justified(|ui| { ui.label("Select a log file."); });
            return;
        }

        let content = app.log_content.clone();
        let auto_scroll = app.log_auto_scroll;

        let mut scroll = egui::ScrollArea::vertical();
        if auto_scroll {
            scroll = scroll.stick_to_bottom(true);
        }
        scroll.show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut content.as_str())
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );
        });

        if auto_scroll && matches!(app.server_status, crate::server_process::ServerStatus::Running) {
            if let Some(path) = app.selected_log.clone() {
                if let Ok(fresh) = std::fs::read_to_string(&path) {
                    if fresh != app.log_content {
                        app.log_content = fresh;
                    }
                }
            }
            ctx.request_repaint_after(Duration::from_secs(2));
        }
    });
}
