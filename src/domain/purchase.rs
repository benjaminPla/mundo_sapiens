use chrono::NaiveDate;

use crate::domain::sellers::value_objects::SellerId;

pub struct Purchase {
    pub cost:      f64,
    pub date:      NaiveDate,
    pub design_id: i64,
    pub id:        i64,
    pub seller_id: SellerId,
}
