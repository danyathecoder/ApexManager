use std::path::PathBuf;

use crate::{app::App, data::cars::name_for_id, util::format::format_laptime};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Results");
    ui.separator();

    if app.results_files.is_empty() {
        app.scan_results();
    }

    ui.horizontal(|ui| {
        ui.label("Filter:");
        for f in ["All", "P", "Q", "R"] {
            let selected = if f == "All" { app.results_filter.is_empty() } else { app.results_filter == f };
            if ui.selectable_label(selected, f).clicked() {
                app.results_filter = if f == "All" { String::new() } else { f.to_string() };
            }
        }
        if ui.button("Refresh").clicked() {
            app.scan_results();
            app.selected_result = None;
        }
    });

    ui.add_space(4.0);

    let filter = app.results_filter.clone();
    let files: Vec<PathBuf> = app
        .results_files
        .iter()
        .filter(|p| {
            if filter.is_empty() {
                return true;
            }
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            name.contains(&format!("_{filter}."))
        })
        .cloned()
        .collect();

    egui::SidePanel::left("results_files").exact_width(220.0).show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for path in &files {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                let selected = app
                    .selected_result
                    .as_ref()
                    .map(|_| app.results_files.first() == Some(path))
                    .unwrap_or(false);
                if ui.selectable_label(selected, name).clicked() {
                    load_result(app, path.clone());
                }
            }
        });
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        let Some(ref result) = app.selected_result.clone() else {
            ui.centered_and_justified(|ui| { ui.label("Select a result file."); });
            return;
        };

        let wet = if result.is_wet_session != 0 { "WET" } else { "DRY" };
        ui.horizontal(|ui| {
            ui.strong(&result.session_type);
            ui.label("—");
            ui.label(&result.track_name);
            ui.label(format!("[{wet}]"));
        });

        ui.label(format!(
            "Best Lap: {}",
            format_laptime(result.session_result.best_lap as u64)
        ));

        ui.add_space(4.0);
        ui.strong("Leaderboard");

        let lines = &result.session_result.leaderboard_lines;

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("leaderboard")
                .num_columns(9)
                .striped(true)
                .spacing([10.0, 3.0])
                .show(ui, |ui| {
                    ui.strong("Pos");
                    ui.strong("Car#");
                    ui.strong("Car");
                    ui.strong("Team");
                    ui.strong("Driver");
                    ui.strong("Best Lap");
                    ui.strong("Total");
                    ui.strong("Laps");
                    ui.strong("Miss Pit");
                    ui.end_row();

                    for (pos, line) in lines.iter().enumerate() {
                        let driver = &line.current_driver;
                        ui.label(format!("{}", pos + 1));
                        ui.label(format!("#{}", line.car.race_number));
                        ui.label(name_for_id(line.car.car_model));
                        ui.label(&line.car.team_name);
                        ui.label(format!("{} {}", driver.first_name, driver.last_name));
                        ui.label(format_laptime(line.timing.best_lap as u64));
                        ui.label(format_laptime(line.timing.total_time as u64));
                        ui.label(format!("{}", line.timing.lap_count));
                        ui.label(if line.missing_mandatory_pitstop != 0 { "YES" } else { "-" });
                        ui.end_row();
                    }
                });
        });
    });
}

fn load_result(app: &mut App, path: PathBuf) {
    match std::fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(result) => {
                app.selected_result = Some(result);
                app.status_message = format!("Loaded {}", path.display());
            }
            Err(e) => {
                app.status_message = format!("Error parsing result: {e}");
            }
        },
        Err(e) => {
            app.status_message = format!("Error reading result: {e}");
        }
    }
}
