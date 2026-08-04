pub mod dashboard;
pub mod designs;
pub mod production;
pub mod purchase;
pub mod sale;
pub mod sellers;

use crate::infrastructure::sellers::pg_sellers_repository::PgSellersRepository;
use dashboard::ScreenDashboard;
use designs::ScreenDesigns;
use production::ScreenProduction;
use purchase::ScreenPurchase;
use sale::ScreenSale;
use sellers::ScreenSellers;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Dashboard,
    Designs,
    Production,
    Purchase,
    Sale,
    Sellers,
}

pub struct Screens {
    dashboard:  ScreenDashboard,
    designs:    ScreenDesigns,
    production: ScreenProduction,
    purchase:   ScreenPurchase,
    sale:       ScreenSale,
    sellers:    ScreenSellers,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            dashboard:  ScreenDashboard::new(),
            designs:    ScreenDesigns::new(),
            production: ScreenProduction::new(),
            purchase:   ScreenPurchase::new(),
            sale:       ScreenSale::new(),
            sellers:    ScreenSellers::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, active: Screen, runtime: &tokio::runtime::Handle, sellers_repo: &PgSellersRepository) {
        match active {
            Screen::Dashboard  => self.dashboard.show(ui),
            Screen::Designs    => self.designs.show(ui),
            Screen::Production => self.production.show(ui),
            Screen::Purchase   => self.purchase.show(ui),
            Screen::Sale       => self.sale.show(ui),
            Screen::Sellers    => self.sellers.show(ui, runtime, sellers_repo),
        }
    }
}
