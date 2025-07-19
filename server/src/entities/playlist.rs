use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Playlist {
    pub identifier: Uuid,
    pub user_identifier: Uuid,
    pub name: String,
    pub description: String,
    pub is_deleted: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
