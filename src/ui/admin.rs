use crate::app::App;

pub fn show(app: &mut App, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.heading("Admin Commands");
    ui.separator();

    ui.colored_label(
        egui::Color32::from_rgb(180, 180, 60),
        "Connect to the server in-game, then paste these commands into chat.",
    );
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label("Car #:");
        ui.add(egui::DragValue::new(&mut app.admin_car_number).range(1..=998));
    });
    let car = app.admin_car_number;

    ui.add_space(4.0);

    egui::CollapsingHeader::new("Session Control").default_open(true).show(ui, |ui| {
        cmd_btn(app, ctx, ui, "Next Session",  "/next",    "Advance to the next session");
        cmd_btn(app, ctx, ui, "Restart",       "/restart", "Restart the current session");
    });

    egui::CollapsingHeader::new("Player Actions").default_open(true).show(ui, |ui| {
        cmd_btn(app, ctx, ui, "Kick",      &format!("/kick {car}"),      "Kick car from session");
        cmd_btn(app, ctx, ui, "Ban",       &format!("/ban {car}"),       "Ban car from session");
        cmd_btn(app, ctx, ui, "DQ",        &format!("/dq {car}"),        "Disqualify car");
        cmd_btn(app, ctx, ui, "Clear DQ",  &format!("/clear {car}"),     "Clear DQ on car");
        cmd_btn(app, ctx, ui, "Clear All", "/clear_all",                 "Clear all penalties");
    });

    egui::CollapsingHeader::new("Penalties").default_open(true).show(ui, |ui| {
        ui.horizontal(|ui| {
            cmd_btn(app, ctx, ui, "TP 5s",        &format!("/tp5 {car}"),  "5s time penalty");
            cmd_btn(app, ctx, ui, "TP 5s (col)",  &format!("/tp5c {car}"), "5s collision penalty");
        });
        ui.horizontal(|ui| {
            cmd_btn(app, ctx, ui, "TP 15s",       &format!("/tp15 {car}"),  "15s time penalty");
            cmd_btn(app, ctx, ui, "TP 15s (col)", &format!("/tp15c {car}"), "15s collision penalty");
        });
        ui.horizontal(|ui| {
            cmd_btn(app, ctx, ui, "Drive-Through",      &format!("/dt {car}"),  "Drive-through penalty");
            cmd_btn(app, ctx, ui, "Drive-Through (col)",&format!("/dtc {car}"), "Drive-through (collision)");
        });
        cmd_btn(app, ctx, ui, "Stop-Go 10s", &format!("/sg10 {car}"), "10s stop-go penalty");
        cmd_btn(app, ctx, ui, "Stop-Go 20s", &format!("/sg20 {car}"), "20s stop-go penalty");
        cmd_btn(app, ctx, ui, "Stop-Go 30s", &format!("/sg30 {car}"), "30s stop-go penalty");
    });

    egui::CollapsingHeader::new("BoP Overrides").show(ui, |ui| {
        cmd_btn(app, ctx, ui, &format!("Ballast {} +0kg", car), &format!("/ballast {car} 0"), "Set ballast on car (edit cmd after copy)");
        cmd_btn(app, ctx, ui, &format!("Restrictor {} +0%", car), &format!("/restrictor {car} 0"), "Set restrictor on car (edit cmd after copy)");
    });

    egui::CollapsingHeader::new("Diagnostics").show(ui, |ui| {
        cmd_btn(app, ctx, ui, "Manual Entry List",  "/manual entrylist",    "Reload entry list manually");
        cmd_btn(app, ctx, ui, "Debug Formation",    "/debug formation",     "Formation lap debug info");
        cmd_btn(app, ctx, ui, "Debug Bandwidth",    "/debug bandwidth",     "Bandwidth diagnostics");
        cmd_btn(app, ctx, ui, "Debug QoS",          "/debug qos",           "QoS diagnostics");
    });
}

fn cmd_btn(app: &mut App, ctx: &egui::Context, ui: &mut egui::Ui, label: &str, cmd: &str, tooltip: &str) {
    if ui.button(label).on_hover_text(format!("{tooltip}\n\nCopies: {cmd}")).clicked() {
        ctx.copy_text(cmd.to_string());
        app.set_toast("Copied!");
    }
}
