use sqlx::PgPool;

use crate::domain::sellers::repository::{SellerRepoError, SellersRepository};
use crate::domain::sellers::value_objects::{SellerCompanyName, SellerContactName, SellerEmail, SellerPhone};
use crate::domain::sellers::Seller;

#[derive(Clone)]
pub struct PgSellersRepository {
    pool: PgPool,
}

impl PgSellersRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

impl SellersRepository for PgSellersRepository {
    async fn create(&self, seller: Seller) -> Result<(), SellerRepoError> {
        sqlx::query(
            "INSERT INTO sellers (company_name, contact_name, email, phone) VALUES ($1, $2, $3, $4)",
        )
        .bind(seller.company_name.value())
        .bind(seller.contact_name.as_ref().map(SellerContactName::value))
        .bind(seller.email.as_ref().map(SellerEmail::value))
        .bind(seller.phone.as_ref().map(SellerPhone::value))
        .execute(&self.pool)
        .await
        .map_err(|err| SellerRepoError::Database(err.to_string()))?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Seller>, SellerRepoError> {
        let rows: Vec<(i64, String, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
            "SELECT id, company_name, contact_name, email, phone FROM sellers ORDER BY company_name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| SellerRepoError::Database(err.to_string()))?;

        rows.into_iter()
            .map(|(id, company_name, contact_name, email, phone)| {
                Ok(Seller {
                    company_name: SellerCompanyName::new(company_name).map_err(|err| SellerRepoError::Mapping(err.to_string()))?,
                    contact_name: contact_name.map(SellerContactName::new).transpose().map_err(|err| SellerRepoError::Mapping(err.to_string()))?,
                    email:        email.map(SellerEmail::new).transpose().map_err(|err| SellerRepoError::Mapping(err.to_string()))?,
                    id:           id.into(),
                    phone:        phone.map(SellerPhone::new).transpose().map_err(|err| SellerRepoError::Mapping(err.to_string()))?,
                })
            })
            .collect()
    }
}
