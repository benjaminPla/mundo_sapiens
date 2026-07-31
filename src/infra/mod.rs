//! Postgres/sqlx adapters implementing `app::AppFacade`. Only `main.rs`
//! (the composition root) reaches into this module, to construct the real
//! facade; every presentation screen only ever sees `dyn AppFacade` from
//! `app`.

mod bootstrap;
mod facade;
mod state;

pub use bootstrap::bootstrap;
