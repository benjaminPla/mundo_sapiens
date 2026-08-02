pub mod components;
pub mod tokens;
pub mod screens;

use crate::infrastructure::app_bootstrap::AppBootstrap;
use screens::{Screen, Screens};

pub struct MundoSapiensApp {
    active_screen: Screen,
    screens:       Screens,
}

impl MundoSapiensApp {
    pub fn new() -> Self {
        match AppBootstrap::execute() {
            Ok(_)      => {},
            Err(_error) => {
                eprintln!("Failed during bootstrap"); // TODO: handle errors
                std::process::exit(1);
            }

        }
        Self { active_screen: Screen::Dashboard, screens: Screens::new() }
    }
}

impl eframe::App for MundoSapiensApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(tokens::CYAN).inner_margin(tokens::SPACING_MEDIUM))
            .show(ui, |ui| {
                if let Some(screen) = components::navbar(ui) { self.active_screen = screen };
                self.screens.show(ui, self.active_screen);
            });
    }
}
