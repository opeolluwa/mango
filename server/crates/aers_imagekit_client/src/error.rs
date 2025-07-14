use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImagekitError {
    #[error("Failed to read file: {0}")]
    FileReadError(String),

    #[error("Upload request failed: {0}")]
    UploadError(String),

    #[error("Invalid API key or unauthorized")]
    Unauthorized,

    #[error("Unexpected response from server: {0}")]
    UnexpectedResponse(String),

    #[error("Unexpected error happened")]
    OperationFailed(String),
}
