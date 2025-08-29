use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "userEntity.d.ts")]
pub struct UserEntity {
    pub email: String,
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub is_active: bool,
    pub profile_picture: Option<String>,
    pub username: Option<String>,
}
