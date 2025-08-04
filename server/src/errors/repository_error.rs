use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::adapters::api_response::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Record not found: The requested record does not exist or has been deleted.")]
    RecordNotFound,
    #[error("Duplicate Record: The information you provided already exists.")]
    DuplicateRecord,
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Operation failed due to {0}")]
    OperationFailed(String),
}

impl RepositoryError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            RepositoryError::RecordNotFound => StatusCode::NOT_FOUND,
            RepositoryError::DuplicateRecord => StatusCode::CONFLICT,
            RepositoryError::SqlxError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RepositoryError::OperationFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}
impl IntoResponse for RepositoryError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
