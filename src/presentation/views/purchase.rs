use crate::presentation::theme;

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

pub struct ViewPurchase {}

impl ViewPurchase {
    pub fn show(ui: &mut egui::Ui) {
        ui.heading("Registrar Compra");
        ui.label("Registre a compra de um design a um fornecedor.");
        ui.add_space(theme::SPACING_MEDIUM);

        egui::Grid::new("purchase_form")
            .num_columns(2)
            .spacing([theme::SPACING_LARGE, theme::SPACING_MEDIUM])
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

        ui.add_space(theme::SPACING_MEDIUM);
        ui.button("Salvar Compra");
    }
}
