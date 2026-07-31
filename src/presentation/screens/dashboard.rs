use crate::app::AppFacade;
use crate::presentation::theme;
use egui_extras::{Column, TableBuilder};

const LOW_STOCK_THRESHOLD: i64 = 2;

pub fn show(ui: &mut egui::Ui, facade: &dyn AppFacade) {
    ui.heading("Dashboard");
    ui.label("Situação do estoque por design");
    ui.add_space(theme::SPACING_MD);

    let rows = facade.get_dashboard();

    TableBuilder::new(ui)
        .striped(true)
        .column(Column::remainder().at_least(150.0))
        .column(Column::auto().at_least(120.0))
        .column(Column::auto().at_least(120.0))
        .header(24.0, |mut header| {
            header.col(|ui| {
                ui.strong("Design");
            });
            header.col(|ui| {
                ui.strong("Em Produção");
            });
            header.col(|ui| {
                ui.strong("Pronto");
            });
        })
        .body(|mut body| {
            for row in &rows {
                body.row(28.0, |mut r| {
                    r.col(|ui| {
                        ui.label(&row.design_name);
                    });
                    r.col(|ui| {
                        ui.label(row.in_production.to_string());
                    });
                    r.col(|ui| {
                        let low_stock = row.ready <= LOW_STOCK_THRESHOLD;
                        let color = if low_stock {
                            theme::WARNING
                        } else {
                            theme::TEXT_PRIMARY
                        };
                        ui.colored_label(color, row.ready.to_string());
                    });
                });
            }
        });
}
