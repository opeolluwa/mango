use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse};

use crate::adapters::api_response::ApiResponseBuilder;
use crate::errors::auth_error::AuthenticationError;
use crate::errors::repository_error::RepositoryError;
use crate::errors::user_service_error::UserServiceError;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("an internal database error has occurred")]
    DatabaseError(#[from] sqlx::error::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
    #[error("an internal error occurred")]
    OperationFailed,
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),
    #[error(transparent)]
    UserServiceError(#[from] UserServiceError),
    #[error("badly formatted request")]
    BadRequest,
}

impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::AxumFormRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::OperationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::RepositoryError(err) => err.status_code(),
            ServiceError::AuthenticationError(err) => err.status_code(),
            ServiceError::UserServiceError(err) => err.status_code(),
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
