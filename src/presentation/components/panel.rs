use crate::presentation::tokens;
use egui::{CornerRadius, Stroke, Ui};

pub fn panel(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let rect = egui::Frame::NONE
        .fill(tokens::GREY)
        .corner_radius(CornerRadius::same(tokens::RADIUS))
        .inner_margin(tokens::SPACING_MEDIUM)
        .show(ui, add_contents)
        .response
        .rect;

    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(tokens::STROKE_MEDIUM, tokens::DARK_GREY),
    );
    ui.painter().line_segment(
        [rect.left_top(), rect.left_bottom()],
        Stroke::new(tokens::STROKE_MEDIUM, tokens::DARK_GREY),
    );

    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(tokens::STROKE_MEDIUM, tokens::WHITE),
    );
    ui.painter().line_segment(
        [rect.right_top(), rect.right_bottom()],
        Stroke::new(tokens::STROKE_MEDIUM, tokens::WHITE),
    );
}
