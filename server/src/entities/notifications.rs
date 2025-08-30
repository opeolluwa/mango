use std::fmt;

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

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} (User: {})\n{}\nCreated: {} | Updated: {} | Read: {}",
            self.identifier,
            self.subject,
            self.user_identifier,
            self.body,
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.updated_at
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "N/A".into()),
            self.is_read
        )
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Default, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "paginatedNotification.d.ts")]
pub struct PaginatedNotification {
    pub notifications: Vec<Notification>,
    pub total: i64,
}
