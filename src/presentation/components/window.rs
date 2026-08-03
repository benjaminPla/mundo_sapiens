use crate::presentation::components;
use crate::presentation::tokens;
use egui::Context;

const BTN_HEIGHT: f32 = 14.0;
const BTN_WIDTH:  f32 = 14.0;
const HEIGHT:     f32 = 20.0;

#[derive(PartialEq)]
pub enum WindowAction {
    Cancel,
    Close,
    None,
    Save,
}

pub fn window(
    ctx:          &Context,
    title:        &str,
    is_open:      &mut bool,
    add_contents: impl FnOnce(&mut egui::Ui),
) -> WindowAction {
    if !*is_open { return WindowAction::None }

    let mut action = WindowAction::None;

    egui::Window::new(title)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            components::raised_panel(ui, |ui| {
                let (bar, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), HEIGHT), egui::Sense::empty());
                ui.painter().rect_filled(bar, tokens::RADIUS, tokens::PURPLE);
                ui.painter().text(
                    bar.left_center() + egui::vec2(tokens::SPACING_MEDIUM, 0.0),
                    egui::Align2::LEFT_CENTER,
                    title,
                    egui::FontId::proportional(tokens::FONT_SIZE_LARGE),
                    tokens::WHITE,
                );

                let close_size = egui::vec2(BTN_WIDTH, BTN_HEIGHT);
                let close_rect = egui::Rect::from_center_size(
                    egui::pos2(bar.right() - tokens::SPACING_SMALL - close_size.x / 2.0, bar.center().y),
                    close_size,
                );
                if ui.put(close_rect, |ui: &mut egui::Ui| components::button(ui, "×", close_size)).clicked() {
                    action = WindowAction::Close;
                }

                ui.add_space(tokens::SPACING_SMALL);

                components::depth_panel(ui, |ui| {
                    add_contents(ui);

                    ui.horizontal(|ui| {
                        if components::button(ui, "Salvar", tokens::BTN_DEFAULT_SIZE).clicked() { action = WindowAction::Save }
                        if components::button(ui, "Cancelar", tokens::BTN_DEFAULT_SIZE).clicked() { action = WindowAction::Cancel }
                    });
                });
            });
        });

    if action != WindowAction::None { *is_open = false }

    action
}
