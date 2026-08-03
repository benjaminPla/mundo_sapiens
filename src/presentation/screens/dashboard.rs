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
        components::depth_panel(ui, |ui| {
            components::header(ui, "Dashboard", None);

            ui.label("Situação do estoque por design");
            ui.add_space(tokens::SPACING_SMALL);

            components::table(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(false)
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .header(tokens::TABLE_HEIGHT, |mut header| {
                        header.col(|ui| { components::table_header(ui, "Design"); });
                        header.col(|ui| { components::table_header(ui, "Em Produção"); });
                        header.col(|ui| { components::table_header(ui, "Pronto"); });
                    })
                    .body(|mut body| {
                        for (i, row) in ROWS.iter().enumerate() {
                            body.row(tokens::TABLE_HEIGHT, |mut r| {
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(row.design_name); }); });
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(row.in_production); }); });
                                r.col(|ui| {
                                    components::table_row(ui, i, |ui| {
                                        let low_stock = row.ready <= Self::LOW_STOCK_THRESHOLD;
                                        let color = if low_stock { tokens::WARNING } else { tokens::BLACK };
                                        ui.colored_label(color, row.ready.to_string());
                                    });
                                });
                            });
                        }
                    });
            });
        });
    }
}
