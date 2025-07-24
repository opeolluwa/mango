use crate::error::DbError;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use ts_rs::TS;

pub trait ModelTrait: Sized + Sync + Send {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError>;
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, FromRow, Default)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppPersonalization {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub preferred_voice: Option<String>,
}

impl AppPersonalization {
    pub fn new(
        theme: Option<String>,
        language: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        preferred_voice: Option<String>,
    ) -> Self {
        Self {
            theme,
            language,
            first_name,
            last_name,
            email,
            preferred_voice,
        }
    }
}

impl ModelTrait for AppPersonalization {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError> {
        sqlx::query(r#"INSERT INTO app_personalization (theme, language, first_name, last_name, email, preferred_voice) VALUES (?, ?, ?, ?, ?, ?)"#)
            .bind(self.theme.to_owned())
            .bind(self.language.to_owned())
            .bind(self.first_name.to_owned())
            .bind(self.last_name.to_owned())
            .bind(self.email.to_owned())
            .bind(self.preferred_voice.to_owned())
            .execute(db_conn)
            .await
            .map_err(|err| {
                log::error!("{err}");
                DbError::QueryFailed
            })?;
        Ok(())
    }


}

#[derive(Debug, Serialize, Deserialize, Clone, TS, FromRow, Default)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub app_initialized: bool,
}
