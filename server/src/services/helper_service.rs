use std::{fs, io, path::Path};

use crate::errors::service_error::ServiceError;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Clone)]
pub struct ServiceHelpers {}

impl ServiceHelpers {
    pub fn init() -> Self {
        Self {}
    }
}

pub trait ServiceHelpersTrait {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError>;
    fn validate_password(&self, raw_password: &str, hash: &str) -> Result<bool, ServiceError>;
    fn delete_file_if_exists(path: &str) -> io::Result<()>;
}

impl ServiceHelpersTrait for ServiceHelpers {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError> {
        hash(raw_password.trim(), DEFAULT_COST).map_err(|err| {
            log::error!("operation failed due to {}", err);
            ServiceError::OperationFailed
        })
    }
    fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        verify(password, hash).map_err(|err| {
            log::error!("operation failed due to {}", err);
            ServiceError::OperationFailed
        })
    }

    fn delete_file_if_exists(path: &str) -> io::Result<()> {
        let file_path = Path::new(path);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }
}
