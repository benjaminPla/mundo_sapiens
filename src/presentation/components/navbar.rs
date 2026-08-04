use crate::presentation::screens::Screen;
use crate::presentation::tokens;

fn nav_link(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let font   = egui::FontId::proportional(tokens::FONT_SIZE_LARGE);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font.clone(), tokens::PURPLE);
    let (rect, response) = ui.allocate_exact_size(galley.size(), egui::Sense::click());
    let color = if response.hovered() { tokens::WHITE } else { tokens::BLACK };
    ui.painter().text(rect.left_center(), egui::Align2::LEFT_CENTER, text, font, color);
    ui.add_space(tokens::SPACING_LARGE);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn navbar(ui: &mut egui::Ui) -> Option<Screen> {
    let mut clicked = None;

    ui.horizontal(|ui| {
        if nav_link(ui, "Dashboard").clicked()        { clicked = Some(Screen::Dashboard)  }
        if nav_link(ui, "Registrar Venda").clicked()  { clicked = Some(Screen::Sale)       }
        if nav_link(ui, "Registrar Compra").clicked() { clicked = Some(Screen::Purchase)   }
        if nav_link(ui, "Avançar Produção").clicked() { clicked = Some(Screen::Production) }
        if nav_link(ui, "Designs").clicked()          { clicked = Some(Screen::Designs)    }
        if nav_link(ui, "Fornecedores").clicked()     { clicked = Some(Screen::Sellers)    }
    });

    clicked
}
