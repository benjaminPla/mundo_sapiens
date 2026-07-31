mod facade;
mod inputs;

pub use crate::domain::{Batch, BatchState, Design, Sale, Seller};
pub use facade::AppFacade;
pub use inputs::{
    AdvanceProductionInput, CreateDesignInput, CreateSellerInput, RecordPurchaseInput,
    RecordSaleInput,
};

#[derive(Debug)]
pub struct AppError(pub String);

pub struct DashboardRow {
    pub design_id:     i64,
    pub design_name:   String,
    pub in_production: i64,
    pub ready:         i64,
}
