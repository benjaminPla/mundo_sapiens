use egui::{Color32, Context, CornerRadius, Stroke, StrokeKind};
use std::sync::Arc;

const VT323: &[u8] = include_bytes!("../../assets/fonts/VT323-Regular.ttf");

pub const PURPLE: Color32 = Color32::from_rgb(139, 79, 224);
pub const CYAN: Color32 = Color32::from_rgb(63, 198, 217);
pub const YELLOW: Color32 = Color32::from_rgb(242, 210, 0);

pub const BACKGROUND: Color32 = Color32::from_rgb(247, 247, 248);
pub const SURFACE: Color32 = Color32::from_rgb(255, 255, 255);
pub const BORDER: Color32 = Color32::from_rgb(226, 226, 230);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(26, 26, 26);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(107, 107, 112);

pub const SUCCESS: Color32 = Color32::from_rgb(46, 158, 91);
pub const WARNING: Color32 = Color32::from_rgb(224, 161, 0);
pub const DANGER: Color32 = Color32::from_rgb(214, 69, 69);

pub const SPACING_SM: f32 = 4.0;
pub const SPACING_MD: f32 = 8.0;
pub const SPACING_LG: f32 = 16.0;
pub const RADIUS: u8 = 0;

/// Desktop backdrop behind the floating windows.
pub const DESKTOP: Color32 = CYAN;
/// Title bar fill for every retro window.
pub const TITLE_BAR: Color32 = PURPLE;
pub const TITLE_TEXT: Color32 = Color32::WHITE;
pub const WINDOW_BORDER_WIDTH: f32 = 2.0;

const TITLE_BAR_HEIGHT: f32 = 26.0;
const CONTROL_SIZE: f32 = 16.0;
const MINIMIZED_WIDTH: f32 = 220.0;

pub fn apply(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts
        .font_data
        .insert("vt323".to_owned(), Arc::new(egui::FontData::from_static(VT323)));
    fonts
        .families
        .insert(egui::FontFamily::Proportional, vec!["vt323".to_owned()]);
    fonts
        .families
        .insert(egui::FontFamily::Monospace, vec!["vt323".to_owned()]);
    ctx.set_fonts(fonts);

    let mut visuals = egui::Visuals::light();

    visuals.override_text_color = Some(TEXT_PRIMARY);
    visuals.panel_fill = DESKTOP;
    visuals.window_fill = SURFACE;
    visuals.window_shadow = egui::Shadow::NONE;
    visuals.faint_bg_color = BACKGROUND;
    visuals.extreme_bg_color = SURFACE;

    visuals.widgets.inactive.bg_fill = SURFACE;
    visuals.widgets.inactive.weak_bg_fill = SURFACE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.5, TEXT_PRIMARY);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, PURPLE);
    visuals.widgets.active.bg_fill = PURPLE;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.selection.bg_fill = PURPLE;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

    let corner_radius = CornerRadius::same(RADIUS);
    visuals.window_corner_radius = corner_radius;
    visuals.widgets.inactive.corner_radius = corner_radius;
    visuals.widgets.hovered.corner_radius = corner_radius;
    visuals.widgets.active.corner_radius = corner_radius;

    ctx.set_theme(egui::ThemePreference::Light);
    ctx.set_visuals(visuals);

    ctx.style_mut_of(egui::Theme::Light, |style| {
        style.spacing.item_spacing = egui::vec2(SPACING_MD, SPACING_MD);
        style.spacing.button_padding = egui::vec2(SPACING_MD, SPACING_SM);
    });
}

/// Open/collapsed/maximized state of one retro floating window.
pub struct WindowChrome {
    pub open:      bool,
    pub minimized: bool,
    pub maximized: bool,
}

impl Default for WindowChrome {
    fn default() -> Self {
        Self { open: false, minimized: false, maximized: false }
    }
}

/// Renders `title`/`chrome` as an old-desktop-style floating window: a solid
/// title bar with minimize/maximize/close controls and a sharp-cornered,
/// flat-bordered body. `add_contents` is skipped while minimized.
pub fn retro_window(
    ctx: &Context,
    id: egui::Id,
    title: &str,
    chrome: &mut WindowChrome,
    default_pos: egui::Pos2,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    if !chrome.open {
        return;
    }

    let mut window = egui::Window::new(title)
        .id(id)
        .title_bar(false)
        .collapsible(false)
        .resizable(!chrome.maximized && !chrome.minimized)
        .default_pos(default_pos)
        .default_width(360.0)
        .frame(
            egui::Frame::NONE
                .fill(SURFACE)
                .stroke(Stroke::new(WINDOW_BORDER_WIDTH, TEXT_PRIMARY))
                .corner_radius(CornerRadius::ZERO),
        );

    if chrome.maximized {
        window = window.fixed_rect(ctx.content_rect());
    } else if chrome.minimized {
        window = window.fixed_size(egui::vec2(MINIMIZED_WIDTH, TITLE_BAR_HEIGHT));
    }

    window.show(ctx, |ui| {
        draw_title_bar(ui, title, chrome);
        if !chrome.minimized {
            egui::Frame::NONE.inner_margin(SPACING_MD).show(ui, add_contents);
        }
    });
}

fn draw_title_bar(ui: &mut egui::Ui, title: &str, chrome: &mut WindowChrome) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), TITLE_BAR_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0, TITLE_BAR);
    ui.painter().text(
        rect.left_center() + egui::vec2(SPACING_MD, 0.0),
        egui::Align2::LEFT_CENTER,
        title,
        egui::FontId::proportional(14.0),
        TITLE_TEXT,
    );

    let mut right = rect.right() - SPACING_SM;

    right -= CONTROL_SIZE;
    if control_button(ui, control_rect(rect, right), ControlKind::Close) {
        chrome.open = false;
    }

    right -= CONTROL_SIZE + SPACING_SM;
    if control_button(ui, control_rect(rect, right), ControlKind::Maximize) {
        chrome.maximized = !chrome.maximized;
        chrome.minimized = false;
    }

    right -= CONTROL_SIZE + SPACING_SM;
    if control_button(ui, control_rect(rect, right), ControlKind::Minimize) {
        chrome.minimized = !chrome.minimized;
        chrome.maximized = false;
    }
}

#[derive(Clone, Copy)]
enum ControlKind {
    Close,
    Maximize,
    Minimize,
}

fn control_rect(bar: egui::Rect, right_x: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(right_x, bar.center().y - CONTROL_SIZE / 2.0),
        egui::Vec2::splat(CONTROL_SIZE),
    )
}

fn control_button(ui: &mut egui::Ui, rect: egui::Rect, kind: ControlKind) -> bool {
    let salt = match kind {
        ControlKind::Close => "retro_close",
        ControlKind::Maximize => "retro_maximize",
        ControlKind::Minimize => "retro_minimize",
    };
    let id = ui.id().with(salt).with(rect.min.x as i32);
    let response = ui.interact(rect, id, egui::Sense::click());

    let bg = if response.hovered() { YELLOW } else { SURFACE };
    ui.painter().rect_filled(rect, 0, bg);
    ui.painter()
        .rect_stroke(rect, 0, Stroke::new(1.0, TEXT_PRIMARY), StrokeKind::Inside);

    let glyph = rect.shrink(4.0);
    match kind {
        ControlKind::Close => {
            ui.painter()
                .line_segment([glyph.left_top(), glyph.right_bottom()], Stroke::new(1.5, TEXT_PRIMARY));
            ui.painter()
                .line_segment([glyph.left_bottom(), glyph.right_top()], Stroke::new(1.5, TEXT_PRIMARY));
        }
        ControlKind::Maximize => {
            ui.painter()
                .rect_stroke(glyph, 0, Stroke::new(1.5, TEXT_PRIMARY), StrokeKind::Inside);
        }
        ControlKind::Minimize => {
            let y = glyph.bottom();
            ui.painter().line_segment(
                [egui::pos2(glyph.left(), y), egui::pos2(glyph.right(), y)],
                Stroke::new(1.5, TEXT_PRIMARY),
            );
        }
    }

    response.clicked()
}
