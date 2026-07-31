use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BatchState {
    Cut,
    Magnetized,
    Purchased,
    Ready,
    Sold,
}

pub struct Batch {
    pub date:         NaiveDate,
    pub design_id:    i64,
    pub id:           i64,
    pub purchase_id:  i64,
    pub qty_produced: i64,
}

pub struct StockMovement {
    pub batch_id:   i64,
    pub date:       NaiveDate,
    pub from_state: Option<BatchState>,
    pub id:         i64,
    pub note:       Option<String>,
    pub qty:        i64,
    pub to_state:   BatchState,
}
