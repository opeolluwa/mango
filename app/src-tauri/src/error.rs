// use libaudify::error::AudifyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    message: String,
}

impl<T> From<T> for CommandError
where
    T: ToString,
{
    fn from(err: T) -> Self {
        CommandError {
            message: err.to_string(),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Record not found")]
    NotFound,
    #[error("Failed to execute")]
    QueryFailed,
}
