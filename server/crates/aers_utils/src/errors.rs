use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error extracting env {0}")]
    EnvError(String),
}
