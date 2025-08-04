use crate::error::DbError;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use ts_rs::TS;

pub trait ModelTrait: Sized + Sync + Send {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError>;
}
