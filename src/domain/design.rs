use crate::domain::sellers::value_objects::SellerId;

pub struct Design {
    pub id:        i64,
    pub name:      String,
    pub seller_id: SellerId,
}
