use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<Pool<Sqlite>>,
}
