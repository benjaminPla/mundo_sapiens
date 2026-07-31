use postgresql_embedded::PostgreSQL;
use sqlx::{PgExecutor, PgPool};

use crate::app::{
    AdvanceProductionInput, AppError, AppFacade, CreateDesignInput, CreateSellerInput,
    DashboardRow, RecordPurchaseInput, RecordSaleInput,
};
use crate::domain::{BatchState, Design, Seller};

use super::state::state_to_str;

pub struct PgAppFacade {
    // Kept alive for the app's lifetime: dropping it stops the server.
    _postgresql: PostgreSQL,
    pool: PgPool,
    runtime: tokio::runtime::Runtime,
}

impl PgAppFacade {
    pub(super) fn new(postgresql: PostgreSQL, pool: PgPool, runtime: tokio::runtime::Runtime) -> Self {
        Self {
            _postgresql: postgresql,
            pool,
            runtime,
        }
    }
}

impl AppFacade for PgAppFacade {
    fn get_dashboard(&self) -> Vec<DashboardRow> {
        self.runtime.block_on(get_dashboard(&self.pool))
    }

    fn list_designs(&self) -> Vec<Design> {
        self.runtime.block_on(list_designs(&self.pool))
    }

    fn list_sellers(&self) -> Vec<Seller> {
        self.runtime.block_on(list_sellers(&self.pool))
    }

    fn record_purchase(&self, input: RecordPurchaseInput) -> Result<(), AppError> {
        self.runtime.block_on(record_purchase(&self.pool, input))
    }

    fn advance_production(&self, input: AdvanceProductionInput) -> Result<(), AppError> {
        self.runtime.block_on(advance_production(&self.pool, input))
    }

    fn record_sale(&self, input: RecordSaleInput) -> Result<(), AppError> {
        self.runtime.block_on(record_sale(&self.pool, input))
    }

    fn create_design(&self, input: CreateDesignInput) -> Result<Design, AppError> {
        self.runtime.block_on(create_design(&self.pool, input))
    }

    fn create_seller(&self, input: CreateSellerInput) -> Result<Seller, AppError> {
        self.runtime.block_on(create_seller(&self.pool, input))
    }
}

fn db_err(err: sqlx::Error) -> AppError {
    AppError(err.to_string())
}

async fn get_dashboard(pool: &PgPool) -> Vec<DashboardRow> {
    // Current stock per state per batch is never a mutated counter — it's
    // derived by netting stock_movements (into the state minus out of it),
    // per PLAN.md's ledger model.
    let rows = sqlx::query_as::<_, (i64, String, i64, i64)>(
        r#"
        WITH movement AS (
            SELECT batch_id, to_state AS state, qty FROM stock_movements
            UNION ALL
            SELECT batch_id, from_state AS state, -qty FROM stock_movements WHERE from_state IS NOT NULL
        ),
        net AS (
            SELECT batch_id, state, SUM(qty) AS qty FROM movement GROUP BY batch_id, state
        )
        SELECT
            d.id,
            d.name,
            COALESCE(SUM(CASE WHEN n.state IN ('Purchased', 'Magnetized', 'Cut') THEN n.qty ELSE 0 END), 0)::BIGINT,
            COALESCE(SUM(CASE WHEN n.state = 'Ready' THEN n.qty ELSE 0 END), 0)::BIGINT
        FROM designs d
        LEFT JOIN batches b ON b.design_id = d.id
        LEFT JOIN net n ON n.batch_id = b.id
        GROUP BY d.id, d.name
        ORDER BY d.name
        "#,
    )
    .fetch_all(pool)
    .await
    .expect("dashboard query failed");

    rows.into_iter()
        .map(|(design_id, design_name, in_production, ready)| DashboardRow {
            design_id,
            design_name,
            in_production,
            ready,
        })
        .collect()
}

async fn list_designs(pool: &PgPool) -> Vec<Design> {
    sqlx::query_as::<_, (i64, String, i64, Option<String>)>(
        "SELECT id, name, seller_id, image_path FROM designs ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .expect("list_designs query failed")
    .into_iter()
    .map(|(id, name, seller_id, image_path)| Design {
        id,
        name,
        seller_id,
        image_path,
    })
    .collect()
}

async fn list_sellers(pool: &PgPool) -> Vec<Seller> {
    sqlx::query_as::<_, (i64, String, Option<String>)>(
        "SELECT id, name, contact FROM sellers ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .expect("list_sellers query failed")
    .into_iter()
    .map(|(id, name, contact)| Seller { id, name, contact })
    .collect()
}

/// Batches of `design_id` currently sitting in `state`, oldest first, with
/// their available quantity in that state. Shared by production advancement
/// and sales, both of which draw stock FIFO.
async fn available_batches<'e, E>(
    executor: E,
    design_id: i64,
    state: BatchState,
) -> Result<Vec<(i64, i64)>, sqlx::Error>
where
    E: PgExecutor<'e>,
{
    sqlx::query_as::<_, (i64, i64)>(
        r#"
        WITH movement AS (
            SELECT batch_id, to_state AS state, qty FROM stock_movements
            UNION ALL
            SELECT batch_id, from_state AS state, -qty FROM stock_movements WHERE from_state IS NOT NULL
        )
        SELECT b.id, COALESCE(SUM(m.qty), 0)::BIGINT
        FROM batches b
        JOIN movement m ON m.batch_id = b.id AND m.state = $1
        WHERE b.design_id = $2
        GROUP BY b.id, b.date
        HAVING COALESCE(SUM(m.qty), 0) > 0
        ORDER BY b.date ASC, b.id ASC
        "#,
    )
    .bind(state_to_str(state))
    .bind(design_id)
    .fetch_all(executor)
    .await
}

async fn record_purchase(pool: &PgPool, input: RecordPurchaseInput) -> Result<(), AppError> {
    if input.qty <= 0 {
        return Err(AppError("quantity must be positive".into()));
    }

    let mut tx = pool.begin().await.map_err(db_err)?;
    let today = chrono::Local::now().date_naive();

    let purchase_id: i64 = sqlx::query_scalar(
        "INSERT INTO purchases (design_id, seller_id, cost, date) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(input.design_id)
    .bind(input.seller_id)
    .bind(input.cost)
    .bind(today)
    .fetch_one(&mut *tx)
    .await
    .map_err(db_err)?;

    let batch_id: i64 = sqlx::query_scalar(
        "INSERT INTO batches (design_id, purchase_id, qty_produced, date) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(input.design_id)
    .bind(purchase_id)
    .bind(input.qty)
    .bind(today)
    .fetch_one(&mut *tx)
    .await
    .map_err(db_err)?;

    sqlx::query(
        "INSERT INTO stock_movements (batch_id, from_state, to_state, qty, date) VALUES ($1, NULL, $2, $3, $4)",
    )
    .bind(batch_id)
    .bind(state_to_str(BatchState::Purchased))
    .bind(input.qty)
    .bind(today)
    .execute(&mut *tx)
    .await
    .map_err(db_err)?;

    tx.commit().await.map_err(db_err)
}

async fn advance_production(pool: &PgPool, input: AdvanceProductionInput) -> Result<(), AppError> {
    if input.qty <= 0 {
        return Err(AppError("quantity must be positive".into()));
    }
    if input.from_state == input.to_state {
        return Err(AppError("from and to state must differ".into()));
    }

    let mut tx = pool.begin().await.map_err(db_err)?;
    let today = chrono::Local::now().date_naive();

    let batches = available_batches(&mut *tx, input.design_id, input.from_state)
        .await
        .map_err(db_err)?;

    let mut remaining = input.qty;
    for (batch_id, available) in batches {
        if remaining == 0 {
            break;
        }
        let take = remaining.min(available);

        sqlx::query(
            "INSERT INTO stock_movements (batch_id, from_state, to_state, qty, date) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(batch_id)
        .bind(state_to_str(input.from_state))
        .bind(state_to_str(input.to_state))
        .bind(take)
        .bind(today)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        remaining -= take;
    }

    if remaining > 0 {
        return Err(AppError(format!(
            "not enough stock in that state for this design ({remaining} short)"
        )));
    }

    tx.commit().await.map_err(db_err)
}

async fn record_sale(pool: &PgPool, input: RecordSaleInput) -> Result<(), AppError> {
    if input.qty <= 0 {
        return Err(AppError("quantity must be positive".into()));
    }

    let mut tx = pool.begin().await.map_err(db_err)?;
    let today = chrono::Local::now().date_naive();

    let batches = available_batches(&mut *tx, input.design_id, BatchState::Ready)
        .await
        .map_err(db_err)?;

    let sale_id: i64 = sqlx::query_scalar("INSERT INTO sales (date) VALUES ($1) RETURNING id")
        .bind(today)
        .fetch_one(&mut *tx)
        .await
        .map_err(db_err)?;

    let mut remaining = input.qty;
    for (batch_id, available) in batches {
        if remaining == 0 {
            break;
        }
        let take = remaining.min(available);

        sqlx::query(
            "INSERT INTO stock_movements (batch_id, from_state, to_state, qty, date) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(batch_id)
        .bind(state_to_str(BatchState::Ready))
        .bind(state_to_str(BatchState::Sold))
        .bind(take)
        .bind(today)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        sqlx::query("INSERT INTO sale_lines (sale_id, batch_id, qty) VALUES ($1, $2, $3)")
            .bind(sale_id)
            .bind(batch_id)
            .bind(take)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        remaining -= take;
    }

    if remaining > 0 {
        return Err(AppError(format!(
            "not enough ready stock for this design ({remaining} short)"
        )));
    }

    tx.commit().await.map_err(db_err)
}

async fn create_design(pool: &PgPool, input: CreateDesignInput) -> Result<Design, AppError> {
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO designs (name, seller_id, image_path) VALUES ($1, $2, NULL) RETURNING id",
    )
    .bind(&input.name)
    .bind(input.seller_id)
    .fetch_one(pool)
    .await
    .map_err(db_err)?;

    Ok(Design {
        id,
        name: input.name,
        seller_id: input.seller_id,
        image_path: None,
    })
}

async fn create_seller(pool: &PgPool, input: CreateSellerInput) -> Result<Seller, AppError> {
    let id: i64 = sqlx::query_scalar("INSERT INTO sellers (name, contact) VALUES ($1, $2) RETURNING id")
        .bind(&input.name)
        .bind(&input.contact)
        .fetch_one(pool)
        .await
        .map_err(db_err)?;

    Ok(Seller {
        id,
        name: input.name,
        contact: input.contact,
    })
}
