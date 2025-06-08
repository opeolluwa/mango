use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    message: String,
}


#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Record not found")]
    NotFound,
}
