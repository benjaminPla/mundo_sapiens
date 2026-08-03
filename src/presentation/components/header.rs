use crate::presentation::tokens;
use egui::Ui;

pub fn header(ui: &mut Ui, title: &str, btns: Option<Box<dyn FnOnce(&mut Ui) + '_>>) {
    let width = ui.available_width();

    egui::Frame::NONE
        .fill(tokens::PURPLE)
        .inner_margin(tokens::SPACING_SMALL)
        .show(ui, |ui| {
            ui.set_min_width(width);
            ui.horizontal(|ui| {
                ui.strong(egui::RichText::new(title).color(tokens::WHITE));
                if let Some(btns) = btns {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), btns);
                }
            });
        });

    ui.separator();
}
