use crate::domain::BatchState;

pub struct RecordPurchaseInput {
    pub cost:      f64,
    pub design_id: i64,
    pub qty:       i64,
    pub seller_id: i64,
}

pub struct AdvanceProductionInput {
    pub design_id:  i64,
    pub from_state: BatchState,
    pub qty:        i64,
    pub to_state:   BatchState,
}

pub struct RecordSaleInput {
    pub design_id: i64,
    pub qty:       i64,
}

pub struct CreateDesignInput {
    pub name:      String,
    pub seller_id: i64,
}

pub struct CreateSellerInput {
    pub contact: Option<String>,
    pub name:    String,
}
