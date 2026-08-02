mod domain;
mod infrastructure;
mod presentation;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Mundo Sapiens",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            presentation::tokens::apply(&cc.egui_ctx);
            Ok(Box::new(presentation::MundoSapiensApp::new()))
        }),
    )
}
