use crate::domain::sellers::repository::SellerRepoError;
use crate::domain::sellers::value_objects::{SellerCompanyNameError, SellerContactNameError, SellerEmailError, SellerPhoneError};

#[derive(Debug, thiserror::Error)]
pub enum SellersAppError {
    #[error("erro de repositório: {0}")]
    Repository(String),
    #[error("erro de validação: {0}")]
    Validation(String),
}

impl From<SellerCompanyNameError> for SellersAppError {
    fn from(error: SellerCompanyNameError) -> Self {
        Self::Validation(error.to_string())
    }
}

impl From<SellerContactNameError> for SellersAppError {
    fn from(error: SellerContactNameError) -> Self {
        Self::Validation(error.to_string())
    }
}

impl From<SellerEmailError> for SellersAppError {
    fn from(error: SellerEmailError) -> Self {
        Self::Validation(error.to_string())
    }
}

impl From<SellerPhoneError> for SellersAppError {
    fn from(error: SellerPhoneError) -> Self {
        Self::Validation(error.to_string())
    }
}

impl From<SellerRepoError> for SellersAppError {
    fn from(error: SellerRepoError) -> Self {
        Self::Repository(error.to_string())
    }
}
