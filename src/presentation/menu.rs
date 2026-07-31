pub enum Menu {
    Catalog,
    Dashboard,
    Production,
    Purchase,
    Sale,
}

impl Menu {
    pub const ALL: [Menu; 5] = [
        Self::Catalog,
        Self::Dashboard,
        Self::Production,
        Self::Purchase,
        Self::Sale,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::Catalog    => "Designs e Fornecedores",
            Self::Dashboard  => "Dashboard",
            Self::Production => "Avançar Produção",
            Self::Purchase   => "Registrar Compra",
            Self::Sale       => "Registrar Venda",
        }
    }
}
