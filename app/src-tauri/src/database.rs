use crate::error::DbError;
use sqlx::{Pool, Sqlite};

pub trait ModelTrait: Sized + Sync + Send {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError>;
}
