use aers_utils::extract_env;
use base64::prelude::*;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, multipart};
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

use crate::ImagekitError;

const IMAGEKIT_UPLOAD_URL: &str = "https://upload.imagekit.io/api/v2/files/upload";

#[derive(Debug, Deserialize)]
pub struct ImagekitUploadResponse {
    pub url: String,
}

pub struct ImagekitClient {
    pub private_key: String,
    pub endpoint: String,
}

impl ImagekitClient {
    pub fn new() -> Self {
        let private_key_raw = extract_env::<String>("IMAGEKIT_PRIVATE_KEY").unwrap();
        let private_key = format!("Basic {}", BASE64_STANDARD.encode(private_key_raw));
        let endpoint = IMAGEKIT_UPLOAD_URL.to_string();

        Self {
            private_key,
            endpoint,
        }
    }

    pub async fn upload(
        &self,
        file_path: &Path,
        file_name: &String,
    ) -> Result<ImagekitUploadResponse, ImagekitError> {
        let client = Client::new();

        // Read file bytes
        let mut file =
            File::open(file_path).map_err(|e| ImagekitError::FileReadError(e.to_string()))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| ImagekitError::FileReadError(e.to_string()))?;

        let part = multipart::Part::bytes(buffer)
            .file_name(file_name.clone())
            .mime_str("application/octet-stream")
            .unwrap();

        let form = multipart::Form::new()
            .part("file", part)
            .text("fileName", file_name.to_owned())
            .text("useUniqueFileName", "true");

        let response = client
            .post(&self.endpoint)
            .header(AUTHORIZATION, &self.private_key)
            .multipart(form)
            .send()
            .await
            .map_err(|err| {
                log::error!("Upload failed with err {}", err);
                ImagekitError::UploadError(err.to_string())
            })?;

        if !response.status().is_success() {
            return Err(ImagekitError::UploadError(format!(
                "Upload failed with status: {}",
                response.status()
            )));
        }

        let json: ImagekitUploadResponse = response
            .json()
            .await
            .map_err(|e| ImagekitError::UploadError(e.to_string()))?;

        Ok(json)
    }
}
