use crate::application::sellers::create::{SellerCreateInput, SellersCreateUseCase};
use crate::application::sellers::errors::SellersAppError;
use crate::application::sellers::list::SellersListUseCase;
use crate::domain::sellers::value_objects::{SellerContactName, SellerEmail, SellerPhone};
use crate::domain::sellers::Seller;
use crate::infrastructure::sellers::pg_sellers_repository::PgSellersRepository;
use crate::presentation::components;
use crate::presentation::tokens;
use egui_extras::{Column, TableBuilder};

// ── Form ─────────────────────────────────────────────────────────────────
struct SellerForm {
    contact_name: String,
    email:        String,
    name:         String,
    phone:        String,
}

impl SellerForm {
    fn new() -> Self {
        Self {
            contact_name: "".to_string(),
            email:        "".to_string(),
            name:         "".to_string(),
            phone:        "".to_string(),
        }
    }
}

impl From<&SellerForm> for SellerCreateInput {
    fn from(form: &SellerForm) -> Self {
        let to_option = |s: &String| if s.trim().is_empty() { None } else { Some(s.clone()) };
        Self {
            company_name: form.name.clone(),
            contact_name: to_option(&form.contact_name),
            email:        to_option(&form.email),
            phone:        to_option(&form.phone),
        }
    }
}

// ── Screen ───────────────────────────────────────────────────────────────
pub struct ScreenSellers {
    create_task:   Option<tokio::task::JoinHandle<Result<(), SellersAppError>>>,
    form:          SellerForm,
    list_task:     Option<tokio::task::JoinHandle<Result<Vec<Seller>, SellersAppError>>>,
    sellers:       Option<Vec<Seller>>,
    window_create: bool,
}

impl ScreenSellers {
    pub fn new() -> Self {
        Self {
            create_task:   None,
            form:          SellerForm::new(),
            list_task:     None,
            sellers:       None,
            window_create: false,
        }
    }

    fn spawn_list(&mut self, ctx: &egui::Context, runtime: &tokio::runtime::Handle, sellers_repo: &PgSellersRepository) {
        let use_case = SellersListUseCase::new(sellers_repo.clone());
        let ctx      = ctx.clone();
        self.list_task = Some(runtime.spawn(async move {
            let result = use_case.execute().await;
            ctx.request_repaint();
            result
        }));
    }

    fn poll_list(&mut self, runtime: &tokio::runtime::Handle) {
        let Some(task) = &self.list_task else { return };
        if !task.is_finished() { return }
        let task = self.list_task.take().unwrap();
        match runtime.block_on(task) {
            Ok(Ok(sellers)) => self.sellers = Some(sellers),
            Ok(Err(err))    => eprintln!("Failed to list sellers: {err}"), // TODO: surface error in UI
            Err(err)        => eprintln!("List task panicked: {err}"),
        }
    }

    fn spawn_create(&mut self, ctx: &egui::Context, runtime: &tokio::runtime::Handle, sellers_repo: &PgSellersRepository) {
        let input    = SellerCreateInput::from(&self.form);
        let use_case = SellersCreateUseCase::new(sellers_repo.clone());
        let ctx      = ctx.clone();
        self.create_task = Some(runtime.spawn(async move {
            let result = use_case.execute(input).await;
            ctx.request_repaint();
            result
        }));
    }

    fn poll_create(&mut self, runtime: &tokio::runtime::Handle) {
        let Some(task) = &self.create_task else { return };
        if !task.is_finished() { return }
        let task = self.create_task.take().unwrap();
        match runtime.block_on(task) {
            Ok(Ok(())) => {
                self.form    = SellerForm::new();
                self.sellers = None;
            }
            Ok(Err(err)) => {
                eprintln!("Failed to create seller: {err}"); // TODO: surface error in UI
                self.window_create = true;
            }
            Err(err) => eprintln!("Create task panicked: {err}"),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Handle, sellers_repo: &PgSellersRepository) {
        self.poll_list(runtime);
        self.poll_create(runtime);

        if self.sellers.is_none() && self.list_task.is_none() {
            self.spawn_list(ui.ctx(), runtime, sellers_repo);
        }
        let sellers = self.sellers.as_deref().unwrap_or(&[]);

        components::depth_panel(ui, |ui| {
            components::header(ui, "Fornecedor", Some(Box::new(|ui: &mut egui::Ui| {
                if components::button(ui, "Criar", tokens::BTN_DEFAULT_SIZE).clicked() {
                    self.window_create = true;
                }
            })));

            components::table(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(false)
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .column(Column::remainder())
                    .column(Column::auto())
                    .header(tokens::TABLE_HEIGHT, |mut header| {
                        header.col(|ui| { components::table_header(ui, "Nome"); });
                        header.col(|ui| { components::table_header(ui, "Contato"); });
                        header.col(|ui| { components::table_header(ui, "Email"); });
                        header.col(|ui| { components::table_header(ui, "Telefone"); });
                        header.col(|ui| { components::table_header(ui, "Ações"); });
                    })
                    .body(|mut body| {
                        for (i, seller) in sellers.iter().enumerate() {
                            body.row(tokens::TABLE_HEIGHT, |mut r| {
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(seller.company_name.value()); }); });
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(seller.contact_name.as_ref().map(SellerContactName::value).unwrap_or("")); }); });
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(seller.email.as_ref().map(SellerEmail::value).unwrap_or("")); }); });
                                r.col(|ui| { components::table_row(ui, i, |ui| { ui.label(seller.phone.as_ref().map(SellerPhone::value).unwrap_or("")); }); });
                                r.col(|ui| {
                                    components::table_row(ui, i, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = tokens::SPACING_SMALL;
                                            components::button(ui, "Editar", tokens::BTN_DEFAULT_SIZE);
                                            components::button(ui, "Excluir", tokens::BTN_DEFAULT_SIZE);
                                        });
                                    });
                                });
                            });
                        }
                    });
            });
        });

        let action = components::window(ui.ctx(), "Criar", &mut self.window_create, |ui| {
            egui::Grid::new("seller_form")
                .num_columns(2)
                .spacing([tokens::SPACING_MEDIUM, tokens::SPACING_SMALL])
                .show(ui, |ui| {
                    ui.label("Nome da empresa");
                    ui.text_edit_singleline(&mut self.form.name);
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
                self.spawn_create(ui.ctx(), runtime, sellers_repo);
            }
            components::WindowAction::Cancel => { self.form = SellerForm::new() }
            components::WindowAction::Close  => {}
            components::WindowAction::None   => {}
        }
    }
}
