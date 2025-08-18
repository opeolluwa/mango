use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub identifier: Uuid,
    pub subject: String,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub is_read: bool,
    pub user_identifier: Uuid,
}
