use crate::app::{AppFacade, CreateDesignInput, CreateSellerInput};
use crate::presentation::theme;

#[derive(Default)]
pub struct State {
    new_design_name:      String,
    new_design_seller_id: Option<i64>,
    new_seller_name:      String,
}

pub fn show(ui: &mut egui::Ui, state: &mut State, facade: &dyn AppFacade) {
    ui.heading("Designs e Fornecedores");
    ui.add_space(theme::SPACING_MD);

    ui.columns(2, |columns| {
        designs_column(&mut columns[0], state, facade);
        sellers_column(&mut columns[1], state, facade);
    });
}

fn designs_column(ui: &mut egui::Ui, state: &mut State, facade: &dyn AppFacade) {
    ui.strong("Designs");
    let sellers = facade.list_sellers();
    for d in facade.list_designs() {
        ui.label(&d.name);
    }
    ui.add_space(theme::SPACING_SM);

    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut state.new_design_name);
        let selected = sellers
            .iter()
            .find(|s| Some(s.id) == state.new_design_seller_id)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "Fornecedor...".to_string());
        egui::ComboBox::from_id_salt("new_design_seller")
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for s in &sellers {
                    ui.selectable_value(&mut state.new_design_seller_id, Some(s.id), &s.name);
                }
            });
    });
    if ui.button("Adicionar Design").clicked() {
        if let Some(seller_id) = state.new_design_seller_id {
            let _ = facade.create_design(CreateDesignInput {
                name: state.new_design_name.clone(),
                seller_id,
            });
        }
    }
}

fn sellers_column(ui: &mut egui::Ui, state: &mut State, facade: &dyn AppFacade) {
    ui.strong("Fornecedores");
    for s in facade.list_sellers() {
        ui.label(&s.name);
    }
    ui.add_space(theme::SPACING_SM);

    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut state.new_seller_name);
        if ui.button("Adicionar Fornecedor").clicked() {
            let _ = facade.create_seller(CreateSellerInput {
                name: state.new_seller_name.clone(),
                contact: None,
            });
        }
    });
}
