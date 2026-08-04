pub struct SellerEmail(String);

impl SellerEmail {
    const MAX_CHARS: usize = 254;
    const MIN_CHARS: usize = 2;

    pub fn new(value: impl Into<String>) -> Result<Self, SellerEmailError> {
        let s = value.into().trim().to_owned();
        if s.is_empty()              { return Err(SellerEmailError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(SellerEmailError::TooLong(Self::MAX_CHARS)) }
        if s.len() < Self::MIN_CHARS { return Err(SellerEmailError::TooShort(Self::MIN_CHARS)) }
        let at_pos = s.find('@');
        let valid  = at_pos.is_some_and(|i| i > 0 && i < s.len() - 1) && !s.contains(char::is_whitespace);
        if !valid { return Err(SellerEmailError::Invalid) }
        Ok(Self(s))
    }

    // ── Getters ──────────────────────────────────────────────────────────────
    pub fn value(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────
#[derive(Debug, thiserror::Error)]
pub enum SellerEmailError {
    #[error("o email não pode estar vazio")]
    Empty,
    #[error("o email não pode ter mais de {0} caracteres")]
    TooLong(usize),
    #[error("o email não pode ter menos de {0} caracteres")]
    TooShort(usize),
    #[error("o email informado é inválido")]
    Invalid,
}
