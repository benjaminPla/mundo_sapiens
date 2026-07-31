use crate::domain::{Design, Seller};

use super::inputs::{
    AdvanceProductionInput, CreateDesignInput, CreateSellerInput, RecordPurchaseInput,
    RecordSaleInput,
};
use super::{AppError, DashboardRow};

pub trait AppFacade {
    fn get_dashboard(&self) -> Vec<DashboardRow>;
    fn list_designs(&self)  -> Vec<Design>;
    fn list_sellers(&self)  -> Vec<Seller>;

    fn record_purchase(&self, input: RecordPurchaseInput)       -> Result<(), AppError>;
    fn advance_production(&self, input: AdvanceProductionInput) -> Result<(), AppError>;
    fn record_sale(&self, input: RecordSaleInput)               -> Result<(), AppError>;

    fn create_design(&self, input: CreateDesignInput) -> Result<Design, AppError>;
    fn create_seller(&self, input: CreateSellerInput) -> Result<Seller, AppError>;
}
