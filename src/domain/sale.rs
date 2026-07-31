use chrono::NaiveDate;

pub struct Sale {
    pub date: NaiveDate,
    pub id:   i64,
    pub note: Option<String>,
}

pub struct SaleLine {
    pub batch_id: i64,
    pub id:       i64,
    pub qty:      i64,
    pub sale_id:  i64,
}
