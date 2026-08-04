use crate::domain::sellers::repository::SellersRepository;
use crate::domain::sellers::Seller;
use super::errors::SellersAppError;

pub struct SellersListUseCase<R: SellersRepository> {
    sellers_repo: R,
}

impl<R: SellersRepository> SellersListUseCase<R> {
    pub fn new(sellers_repo: R) -> Self {
        Self { sellers_repo }
    }

    pub async fn execute(&self) -> Result<Vec<Seller>, SellersAppError> {
        let sellers = self.sellers_repo.list().await?;
        Ok(sellers)
    }
}
