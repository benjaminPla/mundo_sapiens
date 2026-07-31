use crate::presentation::theme;

// temp
struct DesingRow {
    name: &'static str,
}
const DESIGN_ROWS: &'static [DesingRow] = &[
    DesingRow { name: "design_name_0" },
    DesingRow { name: "design_name_1" },
];
struct SellersRow {
    id:   u8,
    name: &'static str,
}
const SELLER_ROWS: &'static [SellersRow] = &[
    SellersRow {
        id: 0,
        name: "seller_name_0",
    },
    SellersRow {
        id: 1,
        name: "seller_name_1",
    },
];

pub struct ViewCatalog {}

impl ViewCatalog {
    pub fn show(ui: &mut egui::Ui) {
        ui.heading("Designs e Fornecedores");
        ui.add_space(theme::SPACING_MEDIUM);

        ui.columns(2, |columns| {
            Self::designs_column(&mut columns[0]);
            Self::sellers_column(&mut columns[1]);
        });
    }

    fn designs_column(ui: &mut egui::Ui) {
        ui.strong("Designs");
        for d in DESIGN_ROWS { ui.label(d.name); }
        ui.add_space(theme::SPACING_SMALL);
    }

    fn sellers_column(ui: &mut egui::Ui) {
        ui.strong("Fornecedores");
        for s in SELLER_ROWS { ui.label(s.name); }
        ui.add_space(theme::SPACING_SMALL);
    }
}
