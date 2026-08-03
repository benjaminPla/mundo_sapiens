use crate::presentation::components;
use crate::presentation::tokens;

// temp
struct DesignRow {
    name: &'static str,
}
const DESIGN_ROWS: &'static [DesignRow] = &[
    DesignRow { name: "design_name_0" },
    DesignRow { name: "design_name_1" },
];

pub struct ScreenSale {}

impl ScreenSale {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        components::depth_panel(ui, |ui| {
            components::header(ui, "Registrar Venda", None);
            ui.label("As unidades saem dos lotes mais antigos em estado Pronto (FIFO).");
            ui.add_space(tokens::SPACING_SMALL);
            egui::Grid::new("sale_form")
                .num_columns(2)
                .spacing([tokens::SPACING_LARGE, tokens::SPACING_MEDIUM])
                .show(ui, |ui| {
                    ui.label("Design:");
                    ui.label(DESIGN_ROWS[0].name);
                    ui.end_row();

                    ui.label("Quantidade:");
                    ui.label("0");
                    ui.end_row();
                });

            ui.add_space(tokens::SPACING_MEDIUM);
            ui.button("Registrar Venda");
        });
    }
}
