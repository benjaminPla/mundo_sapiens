pub mod catalog;
pub mod dashboard;
pub mod production;
pub mod purchase;
pub mod sale;

use catalog::ViewCatalog;
use crate::presentation::menu::Menu;
use dashboard::ViewDashboard;
use production::ViewProduction;
use purchase::ViewPurchase;
use sale::ViewSale;

#[derive(PartialEq)]
pub enum View {
    Catalog,
    Dashboard,
    Production,
    Purchase,
    Sale,
}

impl View {
    pub fn show(ui: &mut egui::Ui, active_view: &View) {
        match &active_view {
            View::Catalog    => ViewCatalog::show(ui),
            View::Dashboard  => ViewDashboard::show(ui),
            View::Production => ViewProduction::show(ui),
            View::Purchase   => ViewPurchase::show(ui),
            View::Sale       => ViewSale::show(ui),
        }
    }
}


impl From<&Menu> for View {
    fn from(menu_option: &Menu) -> Self {
        match menu_option {
            Menu::Catalog    => Self::Catalog,
            Menu::Dashboard  => Self::Dashboard,
            Menu::Production => Self::Production,
            Menu::Purchase   => Self::Purchase,
            Menu::Sale       => Self::Sale,
        }
    }

}
