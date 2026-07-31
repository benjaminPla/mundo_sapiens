mod app;
mod domain;
mod infra;
mod presentation;

use app::AppFacade;
use presentation::screens;

fn main() -> eframe::Result<()> {
    let facade: Box<dyn AppFacade> = match infra::bootstrap() {
        Ok(facade) => facade,
        Err(err) => {
            eprintln!("failed to start local database: {}", err.0);
            std::process::exit(1);
        }
    };

    eframe::run_native(
        "Mundo Sapiens",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            presentation::theme::apply(&cc.egui_ctx);
            Ok(Box::new(MundoSapiensApp::new(facade)))
        }),
    )
}

struct MundoSapiensApp {
    facade: Box<dyn AppFacade>,
    screen_state: screens::State,
}

impl MundoSapiensApp {
    fn new(facade: Box<dyn AppFacade>) -> Self {
        Self {
            facade,
            screen_state: screens::State::default(),
        }
    }
}

impl eframe::App for MundoSapiensApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        egui::Panel::left("launcher").show(ui, |ui| {
            screens::launcher(ui, &mut self.screen_state.windows);
        });
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(presentation::theme::DESKTOP))
            .show(ui, |_ui| {});

        screens::show_windows(&ctx, &mut self.screen_state, self.facade.as_ref());
    }
}
