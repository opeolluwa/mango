use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::adapters::api_response::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Record not found")]
    RecordNotFound,
}

impl RepositoryError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            RepositoryError::RecordNotFound => StatusCode::NOT_FOUND,
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
