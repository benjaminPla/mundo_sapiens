use crate::presentation::theme;

// temp
struct DesignRow {
    name: &'static str,
}
const DESIGN_ROWS: &'static [DesignRow] = &[
    DesignRow { name: "design_name_0" },
    DesignRow { name: "design_name_1" },
];

pub struct ViewSale {}

impl ViewSale {
    pub fn show(ui: &mut egui::Ui) {
        ui.heading("Registrar Venda");
        ui.label("As unidades saem dos lotes mais antigos em estado Pronto (FIFO).");
        ui.add_space(theme::SPACING_MEDIUM);

        egui::Grid::new("sale_form")
            .num_columns(2)
            .spacing([theme::SPACING_LARGE, theme::SPACING_MEDIUM])
            .show(ui, |ui| {
                ui.label("Design:");
                ui.label(DESIGN_ROWS[0].name);
                ui.end_row();

                ui.label("Quantidade:");
                ui.label("0");
                ui.end_row();
            });

        ui.add_space(theme::SPACING_MEDIUM);
        ui.button("Registrar Venda");
    }
}
