use chrono::{DateTime, Local};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
#[serde(rename_all = "camelCase")]
pub struct Otp {
    pub identifier: Uuid,
    pub user_identifier: Uuid,
    pub code: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}
