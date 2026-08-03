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
struct SellerRow {
    name: &'static str,
}
const SELLER_ROWS: &'static [SellerRow] = &[
    SellerRow { name: "seller_name_0" },
    SellerRow { name: "seller_name_1" },
];

pub struct ScreenPurchase {}

impl ScreenPurchase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        components::depth_panel(ui, |ui| {
            ui.heading("Registrar Compra");
            ui.label("Registre a compra de um design a um fornecedor.");
            ui.add_space(tokens::SPACING_SMALL);
            egui::Grid::new("purchase_form")
                .num_columns(2)
                .spacing([tokens::SPACING_LARGE, tokens::SPACING_MEDIUM])
                .show(ui, |ui| {
                    ui.label("Design:");
                    ui.label(DESIGN_ROWS[0].name);
                    ui.end_row();

                    ui.label("Fornecedor:");
                    ui.label(SELLER_ROWS[0].name);
                    ui.end_row();

                    ui.label("Quantidade:");
                    ui.label("0");
                    ui.end_row();

                    ui.label("Custo:");
                    ui.label("0.0");
                    ui.end_row();
                });

            ui.add_space(tokens::SPACING_MEDIUM);
            ui.button("Salvar Compra");
        });
    }
}
