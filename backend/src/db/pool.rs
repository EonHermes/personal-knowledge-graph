use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::info;

pub type DbPool = Arc<SqlitePool>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    info!("Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(database_url).await?;
    info!("Database connection established");
    Ok(Arc::new(pool))
}
