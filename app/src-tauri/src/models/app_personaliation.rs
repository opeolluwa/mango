use crate::{database::ModelTrait, error::DbError};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS, FromRow, Default)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppPersonalization {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub preferred_voice: Option<String>,
}


impl AppPersonalization {
    pub fn new(
        theme: Option<String>,
        language: Option<String>,
        preferred_voice: Option<String>,
    ) -> Self {
        Self {
            theme,
            language,
            preferred_voice,
        }
    }
}

impl ModelTrait for AppPersonalization {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError> {
        sqlx::query(r#"INSERT INTO app_personalization (theme, language, preferred_voice) VALUES (?, ?, ?)"#)
            .bind(self.theme.to_owned())
            .bind(self.language.to_owned())
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

