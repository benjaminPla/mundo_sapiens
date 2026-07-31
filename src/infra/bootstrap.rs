use crate::app::{AppError, AppFacade};
use postgresql_embedded::{PostgreSQL, Settings};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use super::facade::PgAppFacade;

const DB_NAME: &str = "mundo_sapiens";

pub fn bootstrap() -> Result<Box<dyn AppFacade>, AppError> {
    let runtime = tokio::runtime::Runtime::new().map_err(|err| AppError(format!("failed to start async runtime: {err}")))?;
    let (postgresql, pool) = runtime.block_on(setup())?;
    Ok(Box::new(PgAppFacade::new(postgresql, pool, runtime)))
}

async fn setup() -> Result<(PostgreSQL, PgPool), AppError> {
    let data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("mundo_sapiens").join("pgdata");
    let settings = Settings {
        data_dir,
        password:  "mundo_sapiens".to_string(),
        port:      0,
        temporary: false,
        ..Default::default()
    };

    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await.map_err(|err| AppError(format!("failed to set up local database: {err}")))?;
    postgresql.start().await.map_err(|err| AppError(format!("failed to start local database: {err}")))?;

    let db_exists = postgresql.database_exists(DB_NAME).await.map_err(|err| AppError(format!("failed to check local database: {err}")))?;
    if !db_exists {
        postgresql.create_database(DB_NAME).await.map_err(|err| AppError(format!("failed to create local database: {err}")))?;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgresql.settings().url(DB_NAME))
        .await
        .map_err(|err| AppError(format!("failed to connect to local database: {err}")))?;

    sqlx::migrate!("./migrations").run(&pool).await.map_err(|err| AppError(format!("migration failed: {err}")))?;

    Ok((postgresql, pool))
}
