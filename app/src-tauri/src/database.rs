use std::{path::Path, sync::Arc};

use crate::{config::AppConfig, error::DbError, state::AppState};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

pub trait ModelTrait: Sized + Sync + Send {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError>;
}

/// Initialize the database connection and state
pub async fn initialize_database(
    app_data_dir: &Path,
    config: &AppConfig,
) -> Result<AppState, DbError> {
    let db_path = app_data_dir.join(config.database_path);
    log::info!("Initializing database at: {}", db_path.display());

    let connection_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .pragma("journal_mode", "WAL") // Enable WAL mode for better performance
        .pragma("synchronous", "NORMAL") // Balanced durability vs performance
        .pragma("cache_size", "1000") // Set cache size
        .pragma("temp_store", "memory"); // Use memory for temp storage

    let pool = SqlitePool::connect_with(connection_options)
        .await
        .map_err(DbError::from)?;

    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(DbError::from)?;

    log::info!("Database connection established successfully");

    Ok(AppState { db: Arc::new(pool) })
}
