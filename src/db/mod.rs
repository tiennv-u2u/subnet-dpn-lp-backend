pub mod models;
pub mod repo;

use std::time::Duration;

use sqlx::{postgres::{PgPoolOptions, Postgres}, Pool};

pub type DbPool = Pool<Postgres>;

/// Creates a connection pool to the PostgreSQL database
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

