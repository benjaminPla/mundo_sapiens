use crate::presentation::tokens;
use egui::{Align2, FontId, Response, Sense, Stroke, Ui, Vec2};

pub fn button(
    ui: &mut Ui,
    text: &str,
    size: Vec2,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    ui.painter().rect_filled(rect, tokens::RADIUS, tokens::LIGHT_CYAN);

    let (tl, br) = if response.is_pointer_button_down_on() {
        (tokens::DARK_GREY, tokens::WHITE)
    } else {
        (tokens::WHITE, tokens::DARK_GREY)
    };

    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(tokens::STROKE_MEDIUM, tl),
    );
    ui.painter().line_segment(
        [rect.left_top(), rect.left_bottom()],
        Stroke::new(tokens::STROKE_MEDIUM, tl),
    );

    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(tokens::STROKE_MEDIUM, br),
    );
    ui.painter().line_segment(
        [rect.right_top(), rect.right_bottom()],
        Stroke::new(tokens::STROKE_MEDIUM, br),
    );

    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        FontId::proportional(tokens::FONT_SIZE_MEDIUM),
        tokens::BLACK,
    );

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
