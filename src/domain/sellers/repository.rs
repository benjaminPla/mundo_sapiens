use super::Seller;

pub trait SellersRepository {
    async fn create(&self, seller: Seller) -> Result<(), SellerRepoError>;
    async fn list(&self) -> Result<Vec<Seller>, SellerRepoError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SellerRepoError {
    #[error("erro de banco de dados: {0}")]
    Database(String),
    #[error("erro de mapeamento: {0}")]
    Mapping(String),
}
