mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::infrastructure::app_bootstrap::AppBootstrap;
use crate::infrastructure::sellers::pg_sellers_repository::PgSellersRepository;
use crate::presentation::components;
use crate::presentation::screens::{Screen, Screens};
use crate::presentation::tokens;

pub struct AppState {
    active_screen: Screen,
    runtime:       tokio::runtime::Runtime,
    screens:       Screens,
    sellers_repo:  PgSellersRepository,
}

impl AppState {
    pub fn new() -> Self {
        let (pool, runtime) = match AppBootstrap::execute() {
            Ok(bootstrap) => bootstrap,
            Err(_error) => {
                eprintln!("Failed during bootstrap: {_error}");
                std::process::exit(1);
            }
        };
        let sellers_repo = PgSellersRepository::new(pool.clone());
        Self {
            active_screen: Screen::Dashboard,
            runtime,
            screens: Screens::new(),
            sellers_repo,
        }
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(tokens::CYAN).inner_margin(tokens::SPACING_MEDIUM))
            .show(ui, |ui| {
                if let Some(screen) = components::navbar(ui) { self.active_screen = screen };
                self.screens.show(ui, self.active_screen, self.runtime.handle(), &self.sellers_repo);
            });
    }
}


fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Mundo Sapiens",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            presentation::tokens::apply(&cc.egui_ctx);
            Ok(Box::new(AppState::new()))
        }),
    )
}
