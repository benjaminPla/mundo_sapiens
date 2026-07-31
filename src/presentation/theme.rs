use egui::{Color32, Context, CornerRadius, Stroke};
use std::sync::Arc;

// ── Font ─────────────────────────────────────────────────────────────────
const VT323: &[u8]                 = include_bytes!("../../assets/fonts/VT323-Regular.ttf");
// ── Colors ───────────────────────────────────────────────────────────────
pub const BLACK: Color32           = Color32::from_rgb(0, 0, 0);
pub const CYAN: Color32            = Color32::from_rgb(63, 198, 217);
pub const ERROR: Color32          = Color32::from_rgb(214, 69, 69);
pub const LIGHT_GREY: Color32      = Color32::from_rgb(247, 247, 248);
pub const PURPLE: Color32          = Color32::from_rgb(139, 79, 224);
pub const SUCCESS: Color32         = Color32::from_rgb(46, 158, 91);
pub const WARNING: Color32         = Color32::from_rgb(224, 161, 0);
pub const WHITE: Color32           = Color32::from_rgb(255, 255, 255);
pub const YELLOW: Color32          = Color32::from_rgb(242, 210, 0);
// ── Spacing and Radius ───────────────────────────────────────────────────
pub const SPACING_SMALL: f32       = 4.0;
pub const SPACING_MEDIUM: f32      = 8.0;
pub const SPACING_LARGE: f32       = 16.0;
pub const RADIUS: u8               = 0;
// ── ? ────────────────────────────────────────────────────────────────────
// pub const BORDER: Color32          = Color32::from_rgb(226, 226, 230);
// pub const TEXT_SECONDARY: Color32  = Color32::from_rgb(107, 107, 112);
// pub const TITLE_BAR: Color32       = PURPLE;
// pub const TITLE_TEXT: Color32      = Color32::WHITE;
// pub const WINDOW_BORDER_WIDTH: f32 = 2.0;
// const TITLE_BAR_HEIGHT: f32        = 26.0;
// const CONTROL_SIZE: f32            = 16.0;
// const MINIMIZED_WIDTH: f32         = 220.0;

pub fn apply(ctx: &Context) {
    let corner_radius = CornerRadius::same(RADIUS);
    let mut fonts     = egui::FontDefinitions::default();
    let mut visuals   = egui::Visuals::light();

    // ── Font ─────────────────────────────────────────────────────────────────
    fonts.font_data.insert("vt323".to_owned(), Arc::new(egui::FontData::from_static(VT323)));
    fonts.families.insert(egui::FontFamily::Proportional, vec!["vt323".to_owned()]);
    ctx.set_fonts(fonts);
    visuals.override_text_color            = Some(BLACK);
    // ── General ──────────────────────────────────────────────────────────────
    visuals.extreme_bg_color               = WHITE;
    visuals.faint_bg_color                 = LIGHT_GREY;
    visuals.panel_fill                     = CYAN;
    // ── Selection ────────────────────────────────────────────────────────────
    visuals.selection.bg_fill              = PURPLE;
    visuals.selection.stroke               = Stroke::new(1.0, Color32::WHITE);
    // ── Widgets ──────────────────────────────────────────────────────────────
    visuals.widgets.active.bg_fill         = PURPLE;
    visuals.widgets.active.corner_radius   = corner_radius;
    visuals.widgets.active.fg_stroke       = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.hovered.bg_stroke      = Stroke::new(1.5, PURPLE);
    visuals.widgets.hovered.corner_radius  = corner_radius;
    visuals.widgets.inactive.bg_fill       = WHITE;
    visuals.widgets.inactive.bg_stroke     = Stroke::new(1.5, BLACK);
    visuals.widgets.inactive.corner_radius = corner_radius;
    visuals.widgets.inactive.weak_bg_fill  = WHITE;
    // ── Windows ──────────────────────────────────────────────────────────────
    visuals.window_corner_radius           = corner_radius;
    visuals.window_fill                    = WHITE;
    visuals.window_shadow                  = egui::Shadow::NONE;

    ctx.set_theme(egui::ThemePreference::Light);
    ctx.set_visuals(visuals);
    ctx.style_mut_of(egui::Theme::Light, |style| {
        style.spacing.item_spacing   = egui::vec2(SPACING_MEDIUM, SPACING_MEDIUM);
        style.spacing.button_padding = egui::vec2(SPACING_MEDIUM, SPACING_SMALL);
    });
}

// pub struct WindowChrome {
    // pub open:      bool,
    // pub minimized: bool,
    // pub maximized: bool,
// }

// impl Default for WindowChrome {
    // fn default() -> Self {
        // Self { open: false, minimized: false, maximized: false }
    // }
// }

// pub fn retro_window(
    // ctx: &Context,
    // id: egui::Id,
    // title: &str,
    // chrome: &mut WindowChrome,
    // default_pos: egui::Pos2,
    // add_contents: impl FnOnce(&mut egui::Ui),
// ) {
    // if !chrome.open {
        // return;
    // }

    // let mut window = egui::Window::new(title)
        // .id(id)
        // .title_bar(false)
        // .collapsible(false)
        // .resizable(!chrome.maximized && !chrome.minimized)
        // .default_pos(default_pos)
        // .default_width(360.0)
        // .frame(
            // egui::Frame::NONE
                // .fill(WHITE)
                // .stroke(Stroke::new(WINDOW_BORDER_WIDTH, BLACK))
                // .corner_radius(CornerRadius::ZERO),
        // );

    // if chrome.maximized {
        // window = window.fixed_rect(ctx.content_rect());
    // } else if chrome.minimized {
        // window = window.fixed_size(egui::vec2(MINIMIZED_WIDTH, TITLE_BAR_HEIGHT));
    // }

    // window.show(ctx, |ui| {
        // draw_title_bar(ui, title, chrome);
        // if !chrome.minimized {
            // egui::Frame::NONE.inner_margin(SPACING_MEDIUM).show(ui, add_contents);
        // }
    // });
// }

// fn draw_title_bar(ui: &mut egui::Ui, title: &str, chrome: &mut WindowChrome) {
    // let (rect, _) = ui.allocate_exact_size(
        // egui::vec2(ui.available_width(), TITLE_BAR_HEIGHT),
        // egui::Sense::hover(),
    // );
    // ui.painter().rect_filled(rect, 0, TITLE_BAR);
    // ui.painter().text(
        // rect.left_center() + egui::vec2(SPACING_MEDIUM, 0.0),
        // egui::Align2::LEFT_CENTER,
        // title,
        // egui::FontId::proportional(14.0),
        // TITLE_TEXT,
    // );

    // let mut right = rect.right() - SPACING_SMALL;

    // right -= CONTROL_SIZE;
    // if control_button(ui, control_rect(rect, right), ControlKind::Close) {
        // chrome.open = false;
    // }

    // right -= CONTROL_SIZE + SPACING_SMALL;
    // if control_button(ui, control_rect(rect, right), ControlKind::Maximize) {
        // chrome.maximized = !chrome.maximized;
        // chrome.minimized = false;
    // }

    // right -= CONTROL_SIZE + SPACING_SMALL;
    // if control_button(ui, control_rect(rect, right), ControlKind::Minimize) {
        // chrome.minimized = !chrome.minimized;
        // chrome.maximized = false;
    // }
// }

// #[derive(Clone, Copy)]
// enum ControlKind {
    // Close,
    // Maximize,
    // Minimize,
// }

// fn control_rect(bar: egui::Rect, right_x: f32) -> egui::Rect {
    // egui::Rect::from_min_size(
        // egui::pos2(right_x, bar.center().y - CONTROL_SIZE / 2.0),
        // egui::Vec2::splat(CONTROL_SIZE),
    // )
// }

// fn control_button(ui: &mut egui::Ui, rect: egui::Rect, kind: ControlKind) -> bool {
    // let salt = match kind {
        // ControlKind::Close => "retro_close",
        // ControlKind::Maximize => "retro_maximize",
        // ControlKind::Minimize => "retro_minimize",
    // };
    // let id = ui.id().with(salt).with(rect.min.x as i32);
    // let response = ui.interact(rect, id, egui::Sense::click());

    // let bg = if response.hovered() { YELLOW } else { WHITE };
    // ui.painter().rect_filled(rect, 0, bg);
    // ui.painter()
        // .rect_stroke(rect, 0, Stroke::new(1.0, BLACK), StrokeKind::Inside);

    // let glyph = rect.shrink(4.0);
    // match kind {
        // ControlKind::Close => {
            // ui.painter()
                // .line_segment([glyph.left_top(), glyph.right_bottom()], Stroke::new(1.5, BLACK));
            // ui.painter()
                // .line_segment([glyph.left_bottom(), glyph.right_top()], Stroke::new(1.5, BLACK));
        // }
        // ControlKind::Maximize => {
            // ui.painter()
                // .rect_stroke(glyph, 0, Stroke::new(1.5, BLACK), StrokeKind::Inside);
        // }
        // ControlKind::Minimize => {
            // let y = glyph.bottom();
            // ui.painter().line_segment(
                // [egui::pos2(glyph.left(), y), egui::pos2(glyph.right(), y)],
                // Stroke::new(1.5, BLACK),
            // );
        // }
    // }

    // response.clicked()
// }
