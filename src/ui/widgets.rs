use crate::ui::theme;

/// Titled card frame. Wraps content in a bordered, rounded surface.
pub fn card(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(theme::CARD)
        .stroke(egui::Stroke::new(1.0, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            if !title.is_empty() {
                ui.label(
                    egui::RichText::new(title)
                        .strong()
                        .size(11.0)
                        .color(theme::ACCENT),
                );
                ui.add_space(6.0);
            }
            add_contents(ui);
        });
    ui.add_space(8.0);
}

/// Small muted section header label (e.g. "CONFIGURATION").
pub fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.add_space(4.0);
    ui.label(egui::RichText::new(text).size(10.0).color(theme::MUTED));
    ui.add_space(2.0);
}

/// Collapsible quick-reference panel. Starts collapsed; egui remembers state per session.
/// Put `help_row` calls inside the closure — they render inside a 2-column grid.
pub fn help_panel(ui: &mut egui::Ui, id: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::CollapsingHeader::new(
        egui::RichText::new("ℹ  Quick Reference")
            .color(theme::MUTED)
            .size(11.5),
    )
    .id_salt(id)
    .default_open(false)
    .show(ui, |ui| {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(10, 16, 34))
            .inner_margin(egui::Margin::same(10))
            .corner_radius(egui::CornerRadius::same(4))
            .show(ui, |ui| {
                egui::Grid::new(format!("{id}_help"))
                    .num_columns(2)
                    .spacing([14.0, 4.0])
                    .show(ui, add_contents);
            });
    });
    ui.add_space(6.0);
}

/// One row inside `help_panel`: highlighted field name + plain description.
pub fn help_row(ui: &mut egui::Ui, field: &str, desc: &str) {
    ui.label(egui::RichText::new(field).strong().color(theme::ACCENT).size(11.5));
    ui.label(egui::RichText::new(desc).color(theme::TEXT).size(11.5));
    ui.end_row();
}
