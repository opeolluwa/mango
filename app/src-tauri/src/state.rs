use sqlx::{Pool, Sqlite};
use std::sync::Arc;
pub struct AppState {
    pub db: Arc<Pool<Sqlite>>,
}
