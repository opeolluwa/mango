use sqlx::{Pool, Sqlite};
use std::sync::Arc;
pub struct AppState {
    pub db: Arc<Pool<Sqlite>>,
    pub setup: SetupState,
}

#[derive(Default)]
pub struct SetupState {
    pub frontend_task: bool,
    pub backend_task: bool,
}
