// Palette — Tailwind Slate + blue/purple accents
// #0F172A bg  #1E293B card  #3B82F6 primary  #A855F7 accent
// #F8FAFC text  #94A3B8 muted  #334155 border

pub const BG:      egui::Color32 = egui::Color32::from_rgb(15,  23,  42);
pub const CARD:    egui::Color32 = egui::Color32::from_rgb(30,  41,  59);
pub const PRIMARY: egui::Color32 = egui::Color32::from_rgb(59,  130, 246);
pub const ACCENT:  egui::Color32 = egui::Color32::from_rgb(168, 85,  247);
pub const TEXT:    egui::Color32 = egui::Color32::from_rgb(248, 250, 252);
pub const MUTED:   egui::Color32 = egui::Color32::from_rgb(148, 163, 184);
pub const BORDER:  egui::Color32 = egui::Color32::from_rgb(51,  65,  85);
pub const DEEP:    egui::Color32 = egui::Color32::from_rgb(8,   12,  24);

pub const GREEN:   egui::Color32 = egui::Color32::from_rgb(74,  222, 128);
pub const RED:     egui::Color32 = egui::Color32::from_rgb(248, 113, 113);
pub const AMBER:   egui::Color32 = egui::Color32::from_rgb(251, 191, 36);

pub fn apply(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();

    v.panel_fill       = BG;
    v.window_fill      = CARD;
    v.faint_bg_color   = egui::Color32::from_rgb(20, 30, 50);
    v.extreme_bg_color = DEEP;
    v.code_bg_color    = CARD;

    v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(59, 130, 246, 65);
    v.selection.stroke  = egui::Stroke::new(1.0, PRIMARY);
    v.hyperlink_color   = PRIMARY;

    v.window_corner_radius = egui::CornerRadius::same(8);
    v.window_stroke        = egui::Stroke::new(1.0, BORDER);
    v.menu_corner_radius   = egui::CornerRadius::same(6);

    let r = egui::CornerRadius::same(5);

    v.widgets.noninteractive.corner_radius = r;
    v.widgets.noninteractive.bg_fill       = BG;
    v.widgets.noninteractive.bg_stroke     = egui::Stroke::new(1.0, BORDER);
    v.widgets.noninteractive.fg_stroke     = egui::Stroke::new(1.0, TEXT);

    v.widgets.inactive.corner_radius = r;
    v.widgets.inactive.bg_fill       = CARD;
    v.widgets.inactive.weak_bg_fill  = egui::Color32::from_rgb(22, 32, 52);
    v.widgets.inactive.bg_stroke     = egui::Stroke::new(1.0, BORDER);
    v.widgets.inactive.fg_stroke     = egui::Stroke::new(1.0, TEXT);

    v.widgets.hovered.corner_radius = r;
    v.widgets.hovered.bg_fill       = egui::Color32::from_rgb(37, 99, 200);
    v.widgets.hovered.weak_bg_fill  = egui::Color32::from_rgb(25, 60, 120);
    v.widgets.hovered.bg_stroke     = egui::Stroke::new(1.0, PRIMARY);
    v.widgets.hovered.fg_stroke     = egui::Stroke::new(1.5, TEXT);

    v.widgets.active.corner_radius = r;
    v.widgets.active.bg_fill       = PRIMARY;
    v.widgets.active.weak_bg_fill  = PRIMARY;
    v.widgets.active.bg_stroke     = egui::Stroke::new(1.0, TEXT);
    v.widgets.active.fg_stroke     = egui::Stroke::new(2.0, TEXT);

    v.widgets.open.corner_radius = r;
    v.widgets.open.bg_fill       = CARD;
    v.widgets.open.bg_stroke     = egui::Stroke::new(1.0, ACCENT);

    ctx.set_visuals(v);
}
