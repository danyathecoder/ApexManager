#![windows_subsystem = "windows"]

mod app;
mod config;
mod data;
mod server_process;
mod ui;
mod util;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Apex Manager")
            .with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Apex Manager",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
