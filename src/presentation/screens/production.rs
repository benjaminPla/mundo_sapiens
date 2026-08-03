use crate::presentation::components;
use crate::presentation::tokens;

// temp
struct ProductionRow {
    design_name: &'static str,
    stage:       &'static str,
}
const PRODUCTION_ROWS: &'static [ProductionRow] = &[
    ProductionRow {
        design_name: "design_name_0",
        stage: "Cortado",
    },
    ProductionRow {
        design_name: "design_name_1",
        stage: "Imantado",
    },
];

pub struct ScreenProduction {}

impl ScreenProduction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        components::depth_panel(ui, |ui| {
            components::header(ui, "Avançar Produção", None);
            ui.label("Avance as etapas de produção de um design.");
            ui.add_space(tokens::SPACING_SMALL);
            for row in PRODUCTION_ROWS {
                ui.label(format!("{} — {}", row.design_name, row.stage));
            }
            ui.add_space(tokens::SPACING_SMALL);

            ui.add_space(tokens::SPACING_MEDIUM);
            ui.horizontal(|ui| {
                ui.button("Imantar (Comprado → Imantado)");
                ui.button("Cortar (Imantado → Cortado)");
                ui.button("Marcar Pronto (Cortado → Pronto)");
            });
        });
    }
}
