use serde::{Deserialize, Serialize};
use sqlx::Connection;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use ts_rs::TS;



pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Self {}
    }
}
pub trait ModelTrait {
    async fn save(&self);
    async fn find(&self);
    async fn update(&self);
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Record not found")]
    NotFound,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct AudioBook {
    pub identifier: String,
    pub title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub is_loved: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Playlist {
    pub identifier: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct History {
    pub identifier: String,
    pub audio_book_identifier: String,
}

impl AudioBook {
    pub fn new(title: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        Self {
            identifier: uuid::Uuid::new_v4().to_string(),
            title,
            created_at: now.clone(),
            updated_at: now,
            is_loved: 0,
        }
    }

    // pub async fn create(&self, conn: &tauri_plugin_sql::Connection) -> Result<(), DbError> {
    //     conn.execute(
    //         "INSERT INTO audio_books (identifier, title, created_at, updated_at, is_loved) VALUES (?, ?, ?, ?, ?)",
    //         &[&self.identifier, &self.title, &self.created_at, &self.updated_at, &self.is_loved],
    //     )
    //     .await
    //     .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }

    // pub async fn find_by_id(conn: &tauri_plugin_sql::Connection, id: &str) -> Result<Self, DbError> {
    //     let book = conn
    //         .select_one::<Self>(
    //             "SELECT * FROM audio_books WHERE identifier = ?",
    //             &[id],
    //         )
    //         .await
    //         .map_err(|e| DbError::Database(e.to_string()))?
    //         .ok_or(DbError::NotFound)?;
    //     Ok(book)
    // }

    // pub async fn update(&self, conn: &tauri_plugin_sql::Connection) -> Result<(), DbError> {
    //     conn.execute(
    //         "UPDATE audio_books SET title = ?, updated_at = ?, is_loved = ? WHERE identifier = ?",
    //         &[&self.title, &self.updated_at, &self.is_loved, &self.identifier],
    //     )
    //     .await
    //     .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }

    // pub async fn delete(conn: &tauri_plugin_sql::Connection, id: &str) -> Result<(), DbError> {
    //     conn.execute("DELETE FROM audio_books WHERE identifier = ?", &[id])
    //         .await
    //         .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }
}

impl Playlist {
    pub fn new(name: Option<String>, description: Option<String>) -> Self {
        Self {
            identifier: uuid::Uuid::new_v4().to_string(),
            name,
            description,
        }
    }

    // pub async fn create(&self, conn: &tauri_plugin_sql::Connection) -> Result<(), DbError> {
    //     conn.execute(
    //         "INSERT INTO playlist (identifier, name, description) VALUES (?, ?, ?)",
    //         &[&self.identifier, &self.name, &self.description],
    //     )
    //     .await
    //     .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }

    // pub async fn find_by_id(conn: &tauri_plugin_sql::Connection, id: &str) -> Result<Self, DbError> {
    //     let playlist = conn
    //         .select_one::<Self>(
    //             "SELECT * FROM playlist WHERE identifier = ?",
    //             &[id],
    //         )
    //         .await
    //         .map_err(|e| DbError::Database(e.to_string()))?
    //         .ok_or(DbError::NotFound)?;
    //     Ok(playlist)
    // }

    // pub async fn update(&self, conn: &tauri_plugin_sql::Connection) -> Result<(), DbError> {
    //     conn.execute(
    //         "UPDATE playlist SET name = ?, description = ? WHERE identifier = ?",
    //         &[&self.name, &self.description, &self.identifier],
    //     )
    //     .await
    //     .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }

    // pub async fn delete(conn: &tauri_plugin_sql::Connection, id: &str) -> Result<(), DbError> {
    //     conn.execute("DELETE FROM playlist WHERE identifier = ?", &[id])
    //         .await
    //         .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }
}

impl History {
    pub fn new(audio_book_identifier: String) -> Self {
        Self {
            identifier: uuid::Uuid::new_v4().to_string(),
            audio_book_identifier,
        }
    }

    // pub async fn create(&self, conn: &tauri_plugin_sql::Connection) -> Result<(), DbError> {
    //     conn.execute(
    //         "INSERT INTO history (identifier, audio_book_identifier) VALUES (?, ?)",
    //         &[&self.identifier, &self.audio_book_identifier],
    //     )
    //     .await
    //     .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }

    // pub async fn find_by_id(conn: &tauri_plugin_sql::Connection, id: &str) -> Result<Self, DbError> {
    //     let history = conn
    //         .select_one::<Self>(
    //             "SELECT * FROM history WHERE identifier = ?",
    //             &[id],
    //         )
    //         .await
    //         .map_err(|e| DbError::Database(e.to_string()))?
    //         .ok_or(DbError::NotFound)?;
    //     Ok(history)
    // }

    // pub async fn delete(conn: &tauri_plugin_sql::Connection, id: &str) -> Result<(), DbError> {
    //     conn.execute("DELETE FROM history WHERE identifier = ?", &[id])
    //         .await
    //         .map_err(|e| DbError::Database(e.to_string()))?;
    //     Ok(())
    // }
}

impl ModelTrait for History {
    async fn save(&self) {
        todo!()
    }

    async fn find(&self) {
        todo!()
    }

    async fn update(&self) {
        todo!()
    }
}

impl ModelTrait for AudioBook {
    async fn save(&self) {
        todo!()
    }

    async fn find(&self) {
        todo!()
    }

    async fn update(&self) {
        todo!()
    }
}

impl ModelTrait for Playlist {
    async fn save(&self) {
        todo!()
    }

    async fn find(&self) {
        todo!()
    }

    async fn update(&self) {
        todo!()
    }
}
