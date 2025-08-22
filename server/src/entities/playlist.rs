use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, TS)]
#[ts(export, export_to = "playlist.d.ts")]
pub struct Playlist {
    #[ts(type = "string")]
    pub identifier: Uuid,
    #[ts(type = "string")]
    pub user_identifier: Uuid,
    pub name: String,
    pub description: String,
    pub is_deleted: bool,
    #[ts(type = "string")]
    pub created_at: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: chrono::NaiveDateTime,
}
