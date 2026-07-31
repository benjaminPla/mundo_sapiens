use crate::app::{AppFacade, RecordPurchaseInput};
use crate::presentation::theme;

#[derive(Default)]
pub struct State {
    cost:      String,
    design_id: Option<i64>,
    qty:       String,
    seller_id: Option<i64>,
}

pub fn show(ui: &mut egui::Ui, state: &mut State, facade: &dyn AppFacade) {
    ui.heading("Registrar Compra");
    ui.label("Registre a compra de um design a um fornecedor.");
    ui.add_space(theme::SPACING_MD);

    let designs = facade.list_designs();
    let sellers = facade.list_sellers();

    egui::Grid::new("purchase_form")
        .num_columns(2)
        .spacing([theme::SPACING_LG, theme::SPACING_MD])
        .show(ui, |ui| {
            ui.label("Design:");
            let selected = designs
                .iter()
                .find(|d| Some(d.id) == state.design_id)
                .map(|d| d.name.clone())
                .unwrap_or_else(|| "Selecione...".to_string());
            egui::ComboBox::from_id_salt("purchase_design")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    for d in &designs {
                        ui.selectable_value(&mut state.design_id, Some(d.id), &d.name);
                    }
                });
            ui.end_row();

            ui.label("Fornecedor:");
            let selected = sellers
                .iter()
                .find(|s| Some(s.id) == state.seller_id)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "Selecione...".to_string());
            egui::ComboBox::from_id_salt("purchase_seller")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    for s in &sellers {
                        ui.selectable_value(&mut state.seller_id, Some(s.id), &s.name);
                    }
                });
            ui.end_row();

            ui.label("Quantidade:");
            ui.text_edit_singleline(&mut state.qty);
            ui.end_row();

            ui.label("Custo:");
            ui.text_edit_singleline(&mut state.cost);
            ui.end_row();
        });

    ui.add_space(theme::SPACING_MD);
    if ui.button("Salvar Compra").clicked() {
        if let (Some(design_id), Some(seller_id), Ok(qty), Ok(cost)) = (
            state.design_id,
            state.seller_id,
            state.qty.parse::<i64>(),
            state.cost.parse::<f64>(),
        ) {
            let _ = facade.record_purchase(RecordPurchaseInput {
                design_id,
                seller_id,
                qty,
                cost,
            });
        }
    }
}
