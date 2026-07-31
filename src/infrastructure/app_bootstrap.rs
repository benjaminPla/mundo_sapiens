use postgresql_embedded::{PostgreSQL, Settings};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use super::errors::InfraError;

pub struct AppBootstrap {}

impl AppBootstrap {
    const DB_NAME:     &'static str = "mundo_sapiens";
    const DB_PASSWORD: &'static str = "mundo_sapiens";

    pub fn execute() -> Result<(), InfraError> {
        let runtime = tokio::runtime::Runtime::new().map_err(|err| InfraError::Database(format!("failed to start async runtime: {err}")))?;
        let (postgresql, pool) = runtime.block_on(Self::setup())?;
        Ok(())
    }

    async fn setup() -> Result<(PostgreSQL, PgPool), InfraError> {
        let data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("mundo_sapiens").join("pgdata");
        let settings = Settings {
            data_dir,
            password:  Self::DB_PASSWORD.to_string(),
            port:      0,
            temporary: false,
            ..Default::default()
        };

        let mut postgresql = PostgreSQL::new(settings);
        postgresql.setup().await.map_err(|err| InfraError::Database(format!("failed to set up local database: {err}")))?;
        postgresql.start().await.map_err(|err| InfraError::Database(format!("failed to start local database: {err}")))?;

        let db_exists = postgresql.database_exists(Self::DB_NAME).await.map_err(|err| InfraError::Database(format!("failed to check local database: {err}")))?;
        if !db_exists {
            postgresql.create_database(Self::DB_NAME).await.map_err(|err| InfraError::Database(format!("failed to create local database: {err}")))?;
        }

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&postgresql.settings().url(Self::DB_NAME))
            .await
            .map_err(|err| InfraError::Database(format!("failed to connect to local database: {err}")))?;

        sqlx::migrate!("./migrations").run(&pool).await.map_err(|err| InfraError::Database(format!("migration failed: {err}")))?;

        Ok((postgresql, pool))
    }
}
