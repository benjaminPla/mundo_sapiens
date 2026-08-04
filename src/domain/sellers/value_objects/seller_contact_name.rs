pub struct SellerContactName(String);

impl SellerContactName {
    const MAX_CHARS: usize = 50;
    const MIN_CHARS: usize = 2;

    pub fn new(value: impl Into<String>) -> Result<Self, SellerContactNameError> {
        let s = value.into().trim().to_owned();
        if s.is_empty()              { return Err(SellerContactNameError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(SellerContactNameError::TooLong(Self::MAX_CHARS)) }
        if s.len() < Self::MIN_CHARS { return Err(SellerContactNameError::TooShort(Self::MIN_CHARS)) }
        Ok(Self(s))
    }

    // ── Getters ──────────────────────────────────────────────────────────────
    pub fn value(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────
#[derive(Debug, thiserror::Error)]
pub enum SellerContactNameError {
    #[error("o nome do contato não pode estar vazio")]
    Empty,
    #[error("o nome do contato não pode ter mais de {0} caracteres")]
    TooLong(usize),
    #[error("o nome do contato não pode ter menos de {0} caracteres")]
    TooShort(usize),
}
