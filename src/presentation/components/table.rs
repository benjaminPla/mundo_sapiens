use crate::presentation::tokens;

pub fn table_header(ui: &mut egui::Ui, text: &str) {
    egui::Frame::NONE
        .fill(tokens::PURPLE)
        .inner_margin(tokens::SPACING_SMALL)
        .show(ui, |ui| { ui.strong( egui::RichText::new(text).color(tokens::WHITE)) });
}
