use egui::{Color32, Context, CornerRadius, Stroke};
use std::sync::Arc;

// ── Colors ───────────────────────────────────────────────────────────────
pub const BLACK:            Color32 = Color32::from_rgb(0,   0,   0);
pub const CYAN:             Color32 = Color32::from_rgb(63,  198, 217);
pub const DARK_GREY:        Color32 = Color32::from_rgb(45, 45, 45);
// pub const ERROR:         Color32 = Color32::from_rgb(214, 69,  69);
pub const GREY:             Color32 = Color32::from_rgb(212, 208, 200);
pub const LIGHT_CYAN:       Color32 = Color32::from_rgb(191, 235, 236);
pub const PURPLE:           Color32 = Color32::from_rgb(139, 79,  224);
// pub const SUCCESS:       Color32 = Color32::from_rgb(46,  158, 91);
pub const WARNING:          Color32 = Color32::from_rgb(224, 161, 0);
pub const WHITE:            Color32 = Color32::from_rgb(255, 255, 255);
// pub const YELLOW:        Color32 = Color32::from_rgb(242, 210, 0);
// ── Fonts ────────────────────────────────────────────────────────────────
pub const FONT_SIZE_MEDIUM: f32     = 12.0;
pub const FONT_SIZE_LARGE:  f32     = 14.0;
const VT323:                &[u8]   = include_bytes!("../../assets/fonts/VT323-Regular.ttf");
// ── Radius ───────────────────────────────────────────────────────────────
pub const RADIUS:           u8      = 0;
// ── Spacing ──────────────────────────────────────────────────────────────
pub const SPACING_LARGE:    f32     = 8.0;
pub const SPACING_MEDIUM:   f32     = 4.0;
pub const SPACING_SMALL:    f32     = 2.0;
// ── Stokes ───────────────────────────────────────────────────────────────
pub const STROKE_MEDIUM:    f32     = 1.0;
pub const STROKE_LARGE:     f32     = 1.5;


pub fn apply(ctx: &Context) {
    let corner_radius = CornerRadius::same(RADIUS);
    let mut fonts     = egui::FontDefinitions::default();
    let mut visuals   = egui::Visuals::light();

    // ── Font ─────────────────────────────────────────────────────────────────
    fonts.font_data.insert("vt323".to_owned(), Arc::new(egui::FontData::from_static(VT323)));
    fonts.families.insert(egui::FontFamily::Proportional, vec!["vt323".to_owned()]);
    ctx.set_fonts(fonts);
    visuals.override_text_color              = Some(BLACK);
    // ── General ──────────────────────────────────────────────────────────────
    visuals.extreme_bg_color                 = WHITE;
    visuals.faint_bg_color                   = WHITE;
    visuals.panel_fill                       = CYAN;
    // ── Selection ────────────────────────────────────────────────────────────
    visuals.selection.bg_fill                = PURPLE;
    visuals.selection.stroke                 = Stroke::new(STROKE_LARGE, PURPLE);
    // ── Separators ───────────────────────────────────────────────────────────
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(STROKE_MEDIUM, BLACK);
    // ── Widgets ──────────────────────────────────────────────────────────────
    visuals.widgets.active.bg_fill           = PURPLE;
    visuals.widgets.active.corner_radius     = corner_radius;
    visuals.widgets.active.fg_stroke         = Stroke::new(STROKE_MEDIUM, WHITE);
    visuals.widgets.hovered.bg_stroke        = Stroke::new(STROKE_LARGE, PURPLE);
    visuals.widgets.hovered.corner_radius    = corner_radius;
    visuals.widgets.inactive.bg_fill         = WHITE;
    visuals.widgets.inactive.bg_stroke       = Stroke::new(STROKE_LARGE, BLACK);
    visuals.widgets.inactive.corner_radius   = corner_radius;
    visuals.widgets.inactive.weak_bg_fill    = WHITE;
    // ── Windows ──────────────────────────────────────────────────────────────
    visuals.window_corner_radius             = corner_radius;
    visuals.window_fill                      = WHITE;
    visuals.window_shadow                    = egui::Shadow::NONE;

    ctx.set_theme(egui::ThemePreference::Light);
    ctx.set_visuals(visuals);
    ctx.style_mut_of(egui::Theme::Light, |style| {
        style.spacing.item_spacing   = egui::vec2(SPACING_MEDIUM, SPACING_MEDIUM);
        style.spacing.button_padding = egui::vec2(SPACING_MEDIUM, SPACING_SMALL);
    });
}
