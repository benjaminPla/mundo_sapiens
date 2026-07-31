use crate::presentation::theme;

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

pub struct ViewProduction {}

impl ViewProduction {
    pub fn show(ui: &mut egui::Ui) {
        ui.heading("Avançar Produção");
        ui.label("Avance as etapas de produção de um design.");
        ui.add_space(theme::SPACING_MEDIUM);

        for row in PRODUCTION_ROWS {
            ui.label(format!("{} — {}", row.design_name, row.stage));
        }
        ui.add_space(theme::SPACING_SMALL);

        ui.add_space(theme::SPACING_MEDIUM);
        ui.horizontal(|ui| {
            ui.button("Imantar (Comprado → Imantado)");
            ui.button("Cortar (Imantado → Cortado)");
            ui.button("Marcar Pronto (Cortado → Pronto)");
        });
    }
}
