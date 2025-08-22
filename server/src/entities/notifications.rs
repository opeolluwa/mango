use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Default, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "notification.d.ts")]
pub struct Notification {
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub subject: String,
    pub body: String,
    #[ts(type = "string")]
    pub created_at: DateTime<Local>,
    #[ts(type = "string")]
    pub updated_at: Option<DateTime<Local>>,
    pub is_read: bool,
    #[ts(type = "string")]
    pub user_identifier: Uuid,
}
