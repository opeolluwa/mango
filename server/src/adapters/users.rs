use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "UserInformation.ts")]
pub struct UserDto {
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
//TODO: impl validation if the field is not null
pub struct PartialUserProfile {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
