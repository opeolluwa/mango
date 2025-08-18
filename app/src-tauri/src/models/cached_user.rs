use crate::{adapters::cached_user::CreateCachedUser, database::ModelTrait, error::DbError};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, TS, FromRow, Default)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct CachedUser {
    #[ts(type = "string")]
    pub identifier: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    // pub user_identifier: Option<String>
}

impl CachedUser {
    pub fn new(
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        avatar_url: Option<String>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            email,
            avatar_url,
            identifier: Uuid::new_v4(),
        }
    }

    pub fn from_adapter(adapter: CreateCachedUser) -> Self {
        let CreateCachedUser {
            email,
            first_name,
            last_name,
            avatar_url,
        } = adapter;

        Self::new(first_name, last_name, email, avatar_url)
    }
}

impl ModelTrait for CachedUser {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError> {
        sqlx::query(r#"INSERT INTO cached_user (identifier, first_name, last_name, email, avatar_url) VALUES (?, ?, ?, ?, ?)"#)
        .bind(self.identifier)
            .bind(self.first_name.to_owned())
            .bind(self.last_name.to_owned())
            .bind(self.email.to_owned())
            .bind(self.avatar_url.to_owned())
            .execute(db_conn)
            .await
            .map_err(|err| {
                        log::error!("{err}");
                DbError::Database(err.to_string())
            })?;
        Ok(())
    }
}
