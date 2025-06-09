use crate::error::DbError;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use ts_rs::TS;

pub trait ModelTrait: Sized + Sync + Send {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError>;
    async fn find_all(&self, db_conn: &Pool<Sqlite>) -> Result<Vec<Self>, DbError>;
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, FromRow, Default)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AudioBook {
    pub identifier: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_loved: bool,
}

impl AudioBook {
    pub(crate) async fn delete_by_title(
        &self,
        title: &str,
        db_conn: &Pool<Sqlite>,
    ) -> Result<(), DbError> {
        sqlx::query(r#"DELETE FROM audio_books WHERE title = ?"#)
            .bind(title)
            .execute(db_conn)
            .await
            .map_err(|_err| DbError::QueryFailed)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub identifier: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub identifier: String,
    pub audio_book_identifier: String,
}

impl AudioBook {
    pub fn new(title: String) -> Self {
        Self {
            identifier: uuid::Uuid::new_v4().to_string(),
            title,
            created_at: Local::now().to_string(),
            updated_at: Local::now().to_string(),
            is_loved: false,
        }
    }
}

#[allow(unused)]
impl Playlist {
    pub fn new(name: Option<String>, description: Option<String>) -> Self {
        Self {
            identifier: uuid::Uuid::new_v4().to_string(),
            name,
            description,
        }
    }
}
#[allow(unused)]
impl History {
    pub fn new(audio_book_identifier: String) -> Self {
        Self {
            identifier: uuid::Uuid::new_v4().to_string(),
            audio_book_identifier,
        }
    }
}

impl ModelTrait for AudioBook {
    async fn save(&self, db_conn: &Pool<Sqlite>) -> Result<(), DbError> {
        sqlx::query(
            r#"
            INSERT INTO audio_books (identifier, title, created_at, updated_at, is_loved) VALUES (?, ?, ?, ?, ?)"#
        )
            .bind(self.identifier.to_owned())
            .bind(self.title.to_owned())
            .bind(self.created_at.to_owned())
            .bind(self.updated_at.to_owned())
            .bind(self.is_loved.to_owned())
            .execute(db_conn)
            .await
            .map_err(|err| {
                log::error!("{}", err);
                DbError::QueryFailed
            })?;
        Ok(())
    }

    async fn find_all(&self, db_conn: &Pool<Sqlite>) -> Result<Vec<Self>, DbError> {
        sqlx::query_as::<_, AudioBook>(r#"SELECT * FROM audio_books"#)
            .fetch_all(db_conn)
            .await
            .map_err(|err| {
                log::error!("{}", err);
                DbError::QueryFailed
            })
    }
}
