pub mod menu;
pub mod theme;
pub mod views;

use crate::infrastructure::app_bootstrap::AppBootstrap;
use views::View;

pub struct MundoSapiensApp {
    active_view: views::View,
}

impl MundoSapiensApp {
    pub fn new() -> Self {
        match AppBootstrap::execute() {
            Ok(_)      => {},
            Err(_error) => {
                eprintln!("Failed during bootstrap"); // actuall handle the error from infrastructure
                std::process::exit(1);
            }

        }
        Self { active_view: View::Dashboard }
    }
}

impl eframe::App for MundoSapiensApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ── Menu ─────────────────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            for menu_option in menu::Menu::ALL {
                let view = View::from(&menu_option);
                if ui.selectable_label(self.active_view == view, menu_option.title()).clicked() {
                    self.active_view = view;
                }
            }
        });

        ui.separator();

        // ── Main ─────────────────────────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme::CYAN))
            .show(ui, |ui| { View::show(ui, &self.active_view) });
    }
}
