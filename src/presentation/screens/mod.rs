pub mod catalog;
pub mod dashboard;
pub mod production;
pub mod purchase;
pub mod sale;

use crate::app::AppFacade;
use crate::presentation::theme::{self, WindowChrome};

#[derive(PartialEq, Clone, Copy)]
pub enum Screen {
    Dashboard,
    Purchase,
    Production,
    Sale,
    Catalog,
}

impl Screen {
    pub const ALL: [Screen; 5] = [
        Screen::Dashboard,
        Screen::Purchase,
        Screen::Production,
        Screen::Sale,
        Screen::Catalog,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Screen::Dashboard  => "Dashboard",
            Screen::Purchase   => "Registrar Compra",
            Screen::Production => "Avançar Produção",
            Screen::Sale       => "Registrar Venda",
            Screen::Catalog    => "Designs e Fornecedores",
        }
    }
}

pub struct State {
    pub purchase:   purchase::State,
    pub production: production::State,
    pub sale:       sale::State,
    pub catalog:    catalog::State,
    pub windows:    WindowsState,
}

impl Default for State {
    fn default() -> Self {
        let mut windows = WindowsState::default();
        windows.get_mut(Screen::Dashboard).open = true;
        Self {
            purchase: purchase::State::default(),
            production: production::State::default(),
            sale: sale::State::default(),
            catalog: catalog::State::default(),
            windows,
        }
    }
}

#[derive(Default)]
pub struct WindowsState {
    dashboard:  WindowChrome,
    purchase:   WindowChrome,
    production: WindowChrome,
    sale:       WindowChrome,
    catalog:    WindowChrome,
}

impl WindowsState {
    pub fn get_mut(&mut self, screen: Screen) -> &mut WindowChrome {
        match screen {
            Screen::Dashboard  => &mut self.dashboard,
            Screen::Purchase   => &mut self.purchase,
            Screen::Production => &mut self.production,
            Screen::Sale       => &mut self.sale,
            Screen::Catalog    => &mut self.catalog,
        }
    }
}

/// Launcher toolbar: opens (or refocuses) a window for each screen.
pub fn launcher(ui: &mut egui::Ui, windows: &mut WindowsState) {
    ui.heading("Mundo Sapiens");
    ui.separator();
    for screen in Screen::ALL {
        let chrome = windows.get_mut(screen);
        if ui.selectable_label(chrome.open, screen.title()).clicked() {
            chrome.open = true;
            chrome.minimized = false;
        }
    }
}

/// Draws every open screen as its own floating retro window.
pub fn show_windows(ctx: &egui::Context, state: &mut State, facade: &dyn AppFacade) {
    for (i, screen) in Screen::ALL.into_iter().enumerate() {
        let default_pos = egui::pos2(24.0 + i as f32 * 28.0, 24.0 + i as f32 * 28.0);
        let id = egui::Id::new("retro_window").with(i);
        let chrome = state.windows.get_mut(screen);

        match screen {
            Screen::Dashboard => {
                theme::retro_window(ctx, id, screen.title(), chrome, default_pos, |ui| {
                    dashboard::show(ui, facade);
                });
            }
            Screen::Purchase => {
                let purchase_state = &mut state.purchase;
                theme::retro_window(ctx, id, screen.title(), chrome, default_pos, |ui| {
                    purchase::show(ui, purchase_state, facade);
                });
            }
            Screen::Production => {
                let production_state = &mut state.production;
                theme::retro_window(ctx, id, screen.title(), chrome, default_pos, |ui| {
                    production::show(ui, production_state, facade);
                });
            }
            Screen::Sale => {
                let sale_state = &mut state.sale;
                theme::retro_window(ctx, id, screen.title(), chrome, default_pos, |ui| {
                    sale::show(ui, sale_state, facade);
                });
            }
            Screen::Catalog => {
                let catalog_state = &mut state.catalog;
                theme::retro_window(ctx, id, screen.title(), chrome, default_pos, |ui| {
                    catalog::show(ui, catalog_state, facade);
                });
            }
        }
    }
}
