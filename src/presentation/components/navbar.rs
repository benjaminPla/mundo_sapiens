use crate::presentation::screens::Screen;

pub fn navbar(ui: &mut egui::Ui) -> Option<Screen> {
    let mut clicked = None;

    ui.horizontal(|ui| {
        if ui.selectable_label(false, "Dashboard").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            clicked = Some(Screen::Dashboard);
        }
        if ui.selectable_label(false, "Designs").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            clicked = Some(Screen::Designs);
        }
        if ui.selectable_label(false, "Avançar Produção").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            clicked = Some(Screen::Production);
        }
        if ui.selectable_label(false, "Registrar Compra").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            clicked = Some(Screen::Purchase);
        }
        if ui.selectable_label(false, "Registrar Venda").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            clicked = Some(Screen::Sale);
        }
        if ui.selectable_label(false, "Fornecedores").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            clicked = Some(Screen::Sellers);
        }
    });

    clicked
}
