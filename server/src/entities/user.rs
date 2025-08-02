use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserEntity {
    pub email: String,
    pub identifier: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub is_active: bool,
}
