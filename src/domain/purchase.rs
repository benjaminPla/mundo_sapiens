use chrono::NaiveDate;

pub struct Purchase {
    pub cost:      f64,
    pub date:      NaiveDate,
    pub design_id: i64,
    pub id:        i64,
    pub seller_id: i64,
}
