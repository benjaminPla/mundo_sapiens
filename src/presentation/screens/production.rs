use crate::app::{AdvanceProductionInput, AppFacade, BatchState};
use crate::presentation::theme;

#[derive(Default)]
pub struct State {
    design_id: Option<i64>,
    qty:       String,
}

pub fn show(ui: &mut egui::Ui, state: &mut State, facade: &dyn AppFacade) {
    ui.heading("Avançar Produção");
    ui.label("Avance as etapas de produção de um design.");
    ui.add_space(theme::SPACING_MD);

    let designs = facade.list_designs();

    egui::Grid::new("production_form")
        .num_columns(2)
        .spacing([theme::SPACING_LG, theme::SPACING_MD])
        .show(ui, |ui| {
            ui.label("Design:");
            let selected = designs
                .iter()
                .find(|d| Some(d.id) == state.design_id)
                .map(|d| d.name.clone())
                .unwrap_or_else(|| "Selecione...".to_string());
            egui::ComboBox::from_id_salt("production_design")
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
    ui.horizontal(|ui| {
        step_button(ui, "Imantar (Comprado → Imantado)",    state, facade, BatchState::Purchased,  BatchState::Magnetized);
        step_button(ui, "Cortar (Imantado → Cortado)",      state, facade, BatchState::Magnetized, BatchState::Cut);
        step_button(ui, "Marcar Pronto (Cortado → Pronto)", state, facade, BatchState::Cut,        BatchState::Ready);
    });
}

fn step_button(
    ui:         &mut egui::Ui,
    label:      &str,
    state:      &State,
    facade:     &dyn AppFacade,
    from_state: BatchState,
    to_state:   BatchState,
) {
    if ui.button(label).clicked() {
        if let (Some(design_id), Ok(qty)) = (state.design_id, state.qty.parse::<i64>()) {
            let _ = facade.advance_production(AdvanceProductionInput {
                design_id,
                from_state,
                to_state,
                qty,
            });
        }
    }
}
