use chrono::DateTime;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Default, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "audioBookEntity.d.ts")]
pub struct AudioBookEntity {
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub name: String,
    pub src: String,
    #[ts(type = "string")]
    pub user_identifier: Uuid,
    #[ts(type = "string")]
    pub playlist_identifier: Option<Uuid>,
    #[ts(type = "date")]
    pub created_at: DateTime<Local>,
    #[ts(type = "date")]
    pub updated_at: Option<DateTime<Local>>,
    #[ts(type = "date")]
    pub last_played: Option<DateTime<Local>>,
}
