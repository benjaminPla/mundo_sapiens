use crate::presentation::components;
use crate::presentation::tokens;
use egui_extras::{Column, TableBuilder};

// temp
struct SellersRow {
    contact_name: &'static str,
    email:        &'static str,
    company_name:         &'static str,
    phone:        &'static str,
}
const SELLERTS_ROWS: &'static [SellersRow] = &[
    SellersRow { company_name: "company_name_0", contact_name: "contact_name_0", email: "email@email.com", phone: "+1234567890"  },
    SellersRow { company_name: "company_name_1", contact_name: "contact_name_1", email: "email@email.com", phone: "+1234567890"  },
];

struct SellerForm {
    contact_name: String,
    email:        String,
    company_name: String,
    phone:        String,
}

impl SellerForm {
    fn new() -> Self {
        Self {
            contact_name: "".to_string(),
            email:        "".to_string(),
            company_name: "".to_string(),
            phone:        "".to_string(),
        }
    }
}

pub struct ScreenSellers {
    form:          SellerForm,
    window_create: bool,
}

impl ScreenSellers {
    pub fn new() -> Self {
        Self {
            form:          SellerForm::new(),
            window_create: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        components::panel(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Fornecedor");
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if components::button(ui, "Criar", egui::vec2(40.0, 20.0)).clicked() {
                            self.window_create = true;
                        }
                    },
                );
            });

            ui.separator();

            TableBuilder::new(ui)
                .striped(true)
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::auto())
                .header(tokens::SPACING_SMALL, |mut header| {
                    header.col(|ui| { components::table_header(ui, "Nome"); });
                    header.col(|ui| { components::table_header(ui, "Contato"); });
                    header.col(|ui| { components::table_header(ui, "Email"); });
                    header.col(|ui| { components::table_header(ui, "Telefone"); });
                    header.col(|ui| { components::table_header(ui, "Ações"); });
                })
                .body(|mut body| {
                    for row in SELLERTS_ROWS {
                        body.row(tokens::SPACING_SMALL, |mut r| {
                            r.col(|ui| { ui.label(row.company_name); });
                            r.col(|ui| { ui.label(row.contact_name); });
                            r.col(|ui| { ui.label(row.email); });
                            r.col(|ui| { ui.label(row.phone); });
                            r.col(|ui| { 
                                ui.horizontal(|ui| {
                                    components::button(ui, "Editar", egui::vec2(40.0, 20.0));
                                    components::button(ui, "Excluir", egui::vec2(40.0, 20.0));
                                });
                            });
                        });
                    }
                });
        });

        let action = components::window(ui.ctx(), "Criar", &mut self.window_create, |ui| {
            egui::Grid::new("seller_form")
                .num_columns(2)
                .spacing([tokens::SPACING_MEDIUM, tokens::SPACING_SMALL])
                .show(ui, |ui| {
                    ui.label("Nome da empresa");
                    ui.text_edit_singleline(&mut self.form.company_name);
                    ui.end_row();

                    ui.label("Nome do contato");
                    ui.text_edit_singleline(&mut self.form.contact_name);
                    ui.end_row();

                    ui.label("Email");
                    ui.text_edit_singleline(&mut self.form.email);
                    ui.end_row();

                    ui.label("Telefone");
                    ui.text_edit_singleline(&mut self.form.phone);
                    ui.end_row();
                });
        });

        match action {
            components::WindowAction::Save => {
                self.form.company_name.clear();
                self.form.contact_name.clear();
                self.form.email.clear();
                self.form.phone.clear();
            }
            components::WindowAction::Cancel => {
                self.form.company_name.clear();
                self.form.contact_name.clear();
                self.form.email.clear();
                self.form.phone.clear();
            }
            components::WindowAction::Close => {}
            components::WindowAction::None => {}
        }
    }
}
