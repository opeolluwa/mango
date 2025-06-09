use libaudify::error::AudifyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    message: String,
}

impl From<DbError> for CommandError {
    fn from(err: DbError) -> Self {
        CommandError {
            message: err.to_string(),
        }
    }
}

impl From<AudifyError> for CommandError {
    fn from(err: AudifyError) -> Self {
        CommandError {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        CommandError {
            message: err.to_string(),
        }
    }
}

impl From<String> for CommandError {
    fn from(err: String) -> Self {
        CommandError { message: err }
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
