pub struct SellerPhone(String);

impl SellerPhone {
    const MAX_CHARS: usize = 20;
    const MIN_CHARS: usize = 2;

    pub fn new(value: impl Into<String>) -> Result<Self, SellerPhoneError> {
        let s = value.into().trim().to_owned();
        if s.is_empty()              { return Err(SellerPhoneError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(SellerPhoneError::TooLong(Self::MAX_CHARS)) }
        if s.len() < Self::MIN_CHARS { return Err(SellerPhoneError::TooShort(Self::MIN_CHARS)) }
        let valid = s.chars().all(|c| c.is_ascii_digit() || matches!(c, ' ' | '+' | '-' | '(' | ')'));
        if !valid { return Err(SellerPhoneError::Invalid) }
        Ok(Self(s))
    }

    // ── Getters ──────────────────────────────────────────────────────────────
    pub fn value(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────
#[derive(Debug, thiserror::Error)]
pub enum SellerPhoneError {
    #[error("o telefone não pode estar vazio")]
    Empty,
    #[error("o telefone não pode ter mais de {0} caracteres")]
    TooLong(usize),
    #[error("o telefone não pode ter menos de {0} caracteres")]
    TooShort(usize),
    #[error("o telefone informado é inválido")]
    Invalid,
}
