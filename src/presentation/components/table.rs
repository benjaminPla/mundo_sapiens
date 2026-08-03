use crate::presentation::tokens;
use egui::Ui;

pub fn table(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let prev_spacing = ui.spacing().item_spacing;
    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
    add_contents(ui);
    ui.spacing_mut().item_spacing = prev_spacing;
}

pub fn table_header(ui: &mut Ui, text: &str) {
    let width = ui.available_size();
    egui::Frame::NONE
        .fill(tokens::PURPLE)
        .show(ui, |ui| {
            ui.set_min_size(width);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.strong(egui::RichText::new(text).color(tokens::WHITE));
            });
        });
}

pub fn table_row(ui: &mut Ui, row_index: usize, add_contents: impl FnOnce(&mut Ui)) {
    let width = ui.available_size();
    let fill  = if row_index % 2 == 0 { tokens::LIGHT_CYAN } else { tokens::WHITE };
    egui::Frame::NONE
        .fill(fill)
        .show(ui, |ui| {
            ui.set_min_size(width);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), add_contents);
        });
}
