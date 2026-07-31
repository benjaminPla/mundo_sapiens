use crate::app::{AppFacade, RecordSaleInput};
use crate::presentation::theme;

#[derive(Default)]
pub struct State {
    design_id: Option<i64>,
    qty:       String,
}

pub fn show(ui: &mut egui::Ui, state: &mut State, facade: &dyn AppFacade) {
    ui.heading("Registrar Venda");
    ui.label("As unidades saem dos lotes mais antigos em estado Pronto (FIFO).");
    ui.add_space(theme::SPACING_MD);

    let designs = facade.list_designs();

    egui::Grid::new("sale_form")
        .num_columns(2)
        .spacing([theme::SPACING_LG, theme::SPACING_MD])
        .show(ui, |ui| {
            ui.label("Design:");
            let selected = designs
                .iter()
                .find(|d| Some(d.id) == state.design_id)
                .map(|d| d.name.clone())
                .unwrap_or_else(|| "Selecione...".to_string());
            egui::ComboBox::from_id_salt("sale_design")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    for d in &designs {
                        ui.selectable_value(&mut state.design_id, Some(d.id), &d.name);
                    }
                });
            ui.end_row();

            ui.label("Quantidade:");
            ui.text_edit_singleline(&mut state.qty);
            ui.end_row();
        });

    ui.add_space(theme::SPACING_MD);
    if ui.button("Registrar Venda").clicked() {
        if let (Some(design_id), Ok(qty)) = (state.design_id, state.qty.parse::<i64>()) {
            let _ = facade.record_sale(RecordSaleInput { design_id, qty });
        }
    }
}
