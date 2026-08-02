use crate::presentation::components;
use crate::presentation::tokens;
use egui_extras::{Column, TableBuilder};

// temp
struct DashboardRow {
    design_name:   &'static str,
    in_production: &'static str,
    ready:         u8,
}
const ROWS: &'static [DashboardRow] = &[
    DashboardRow {
        design_name: "design_name_0",
        in_production: "in_production_0",
        ready: 10,
    },
    DashboardRow {
        design_name: "design_name_1",
        in_production: "in_production_2",
        ready: 1,
    },
];


pub struct ScreenDashboard {}

impl ScreenDashboard {
    const LOW_STOCK_THRESHOLD: u8 = 2;

    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {

        components::panel(ui, |ui| {
            ui.heading("Dashboard");
            ui.label("Situação do estoque por design");
            ui.add_space(tokens::SPACING_SMALL);
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .header(24.0, |mut header| {
                    header.col(|ui| { ui.strong("Design"); });
                    header.col(|ui| { ui.strong("Em Produção"); });
                    header.col(|ui| { ui.strong("Pronto"); });
                })
                .body(|mut body| {
                    for row in ROWS {
                        body.row(28.0, |mut r| {
                            r.col(|ui| {
                                ui.label(row.design_name);
                            });
                            r.col(|ui| {
                                ui.label(row.in_production.to_string());
                            });
                            r.col(|ui| {
                                let low_stock = row.ready <= Self::LOW_STOCK_THRESHOLD;
                                let color = if low_stock { tokens::WARNING } else { tokens::BLACK };
                                ui.colored_label(color, row.ready.to_string());
                            });
                        });
                    }
                });
        });
    }
}
