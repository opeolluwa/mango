use std::path::Path;

use crate::ImagekitError;

pub struct ImagekitUploadResponse {
    pub url: String,
}
pub struct ImagekitClient {
    pub public_key: String,
    pub private_key: String,
    pub endpoint: String,
}

impl ImagekitClient {
    pub fn new() -> Self {
        Self {
            public_key: todo!(),
            private_key: todo!(),
            endpoint: todo!(),
        }
    }

    pub fn upload(&self, file_path: &Path) -> Result<ImagekitUploadResponse, ImagekitError> {
        Ok(ImagekitUploadResponse {
            url: String::from("https://"),
        })
    }
}
