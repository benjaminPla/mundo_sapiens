use crate::presentation::components;
use crate::presentation::tokens;
use egui_extras::{Column, TableBuilder};

// temp
struct DesignRow {
    name:        &'static str,
    seller_name: &'static str,
}
const DESIGN_ROWS: &'static [DesignRow] = &[
    DesignRow { name: "design_name_0", seller_name: "seller_name_0" },
    DesignRow { name: "design_name_1", seller_name: "seller_name_1" },
];

struct DesignForm {
    name:      String,
    seller_id: String,
}

impl DesignForm {
    fn new() -> Self {
        Self {
            name:      "".to_string(),
            seller_id: "".to_string(),
        }
    }
}

pub struct ScreenDesigns {
    form:          DesignForm,
    window_create: bool,
}

impl ScreenDesigns {
    pub fn new() -> Self {
        Self {
            form:          DesignForm::new(),
            window_create: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        components::depth_panel(ui, |ui| {
            components::header(ui, "Designs", Some(Box::new(|ui: &mut egui::Ui| {
                if components::button(ui, "Criar", tokens::BTN_DEFAULT_SIZE).clicked() {
                    self.window_create = true;
                }
            })));

            components::table(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(false)
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .header(tokens::TABLE_HEIGHT, |mut header| {
                        header.col(|ui| { components::table_header(ui, "Nome"); });
                        header.col(|ui| { components::table_header(ui, "Fornecedor"); });
                    })
                    .body(|mut body| {
                        for (i, row) in DESIGN_ROWS.iter().enumerate() {
                            body.row(tokens::TABLE_HEIGHT, |mut r| {
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(row.name); }); });
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(row.seller_name); }); });
                            });
                        }
                    });
            });
        });

        let action = components::window(ui.ctx(), "Criar", &mut self.window_create, |ui| {
            egui::Grid::new("design_form")
                .num_columns(2)
                .spacing([tokens::SPACING_MEDIUM, tokens::SPACING_SMALL])
                .show(ui, |ui| {
                    ui.label("Nome");
                    ui.text_edit_singleline(&mut self.form.name);
                    ui.end_row();

                    ui.label("Fornecedor");
                    ui.text_edit_singleline(&mut self.form.seller_id);
                    ui.end_row();
                });
        });

        match action {
            components::WindowAction::Save => {
                self.form.name.clear();
                self.form.seller_id.clear();
            }
            components::WindowAction::Cancel => {
                self.form.name.clear();
                self.form.seller_id.clear();
            }
            components::WindowAction::Close => {}
            components::WindowAction::None => {}
        }
    }
}
