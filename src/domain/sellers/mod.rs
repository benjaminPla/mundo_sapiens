pub mod repository;
pub mod value_objects;

use value_objects::{SellerCompanyName, SellerContactName, SellerEmail, SellerId, SellerPhone};

pub struct Seller {
    pub company_name: SellerCompanyName,
    pub contact_name: Option<SellerContactName>,
    pub email:        Option<SellerEmail>,
    pub id:           SellerId,
    pub phone:        Option<SellerPhone>,
}

impl Seller {
    // ── Getters ──────────────────────────────────────────────────────────────
    pub fn get_company_name(&self) -> &SellerCompanyName         { &self.company_name }
    pub fn get_contact_name(&self) -> Option<&SellerContactName> { self.contact_name.as_ref() }
    pub fn get_email(&self)        -> Option<&SellerEmail>       { self.email.as_ref() }
    pub fn get_phone(&self)        -> Option<&SellerPhone>       { self.phone.as_ref() }
}
