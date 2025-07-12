use chrono::DateTime;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioBookEntity {
    pub identifier: Uuid,
    pub name: String,
    pub src: String,
    pub user_identifier: Uuid,
    pub playlist_identifier: Option<Uuid>,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub last_played: Option<DateTime<Local>>,
}
