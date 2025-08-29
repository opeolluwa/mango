use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

use crate::entities::user::UserEntity;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export, export_to = "userProfile.d.ts")]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
    pub username: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Validate, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "partialUserProfile.d.ts")]

pub struct PartialUserProfile {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

impl From<UserEntity> for UserProfile {
    fn from(user: UserEntity) -> Self {
        UserProfile {
            identifier: user.identifier,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            profile_picture: user.profile_picture,
            username: user.username,
        }
    }
}
