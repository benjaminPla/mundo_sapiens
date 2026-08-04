use crate::domain::sellers::repository::SellersRepository;
use crate::domain::sellers::value_objects::{SellerCompanyName, SellerContactName, SellerEmail, SellerId, SellerPhone};
use crate::domain::sellers::Seller;
use super::errors::SellersAppError;

pub struct SellerCreateInput {
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email:        Option<String>,
    pub phone:        Option<String>,
}

pub struct SellersCreateUseCase<R: SellersRepository> {
    sellers_repo: R,
}

impl<R: SellersRepository> SellersCreateUseCase<R> {
    pub fn new(sellers_repo: R) -> Self {
        Self { sellers_repo }
    }

    pub async fn execute(&self, input: SellerCreateInput) -> Result<(), SellersAppError> {
        let company_name = SellerCompanyName::new(input.company_name)?;
        let contact_name = input.contact_name.map(SellerContactName::new).transpose()?;
        let email        = input.email.map(SellerEmail::new).transpose()?;
        let phone        = input.phone.map(SellerPhone::new).transpose()?;
        let seller = Seller { company_name, contact_name, email, id: SellerId::from(0), phone };
        self.sellers_repo.create(seller).await?;
        Ok(())
    }
}
