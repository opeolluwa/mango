use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderValue},
    multipart,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use std::fmt;

#[derive(Debug)]
pub enum ImagekitError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    InvalidHeader(reqwest::header::InvalidHeaderValue),
    UploadFailed(String),
}

impl fmt::Display for ImagekitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImagekitError::Reqwest(err) => write!(f, "Request error: {}", err),
            ImagekitError::Io(err) => write!(f, "I/O error: {}", err),
            ImagekitError::InvalidHeader(err) => write!(f, "Invalid header: {}", err),
            ImagekitError::UploadFailed(msg) => write!(f, "Upload failed: {}", msg),
        }
    }
}

impl std::error::Error for ImagekitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ImagekitError::Reqwest(err) => Some(err),
            ImagekitError::Io(err) => Some(err),
            ImagekitError::InvalidHeader(err) => Some(err),
            ImagekitError::UploadFailed(_) => None,
        }
    }
}



impl From<reqwest::Error> for ImagekitError {
    fn from(err: reqwest::Error) -> Self {
        ImagekitError::Reqwest(err)
    }
}

impl From<std::io::Error> for ImagekitError {
    fn from(err: std::io::Error) -> Self {
        ImagekitError::Io(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for ImagekitError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        ImagekitError::InvalidHeader(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub file_id: String,
    pub name: String,
    pub url: String,
    pub file_path: String,
    pub size: u64,
}

pub struct ImagekitClient {
    client: Client,
    upload_url: String,
    public_key: String,
    private_auth: String,
}

impl ImagekitClient {
    pub fn new(public_key: &str, private_auth: &str) -> Result<Self, ImagekitError> {
        Ok(Self {
            client: Client::builder().build()?,
            upload_url: "https://upload.imagekit.io/api/v1/files/upload".to_string(),
            public_key: public_key.to_string(),
            private_auth: private_auth.to_string(),
        })
    }

    pub async fn upload_file<P: AsRef<Path>>(
        &self,
        path: P,
        display_name: &str,
    ) -> Result<UploadResponse, ImagekitError> {
        let file_bytes = fs::read(&path)?;
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Basic {}", self.private_auth))?,
        );

        let form = multipart::Form::new()
            .part(
                "file",
                multipart::Part::bytes(file_bytes).file_name(display_name.to_string()),
            )
            .text("fileName", display_name.to_string())
            .text("publicKey", self.public_key.clone());

        let response = self
            .client
            .request(Method::POST, &self.upload_url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ImagekitError::UploadFailed(format!(
                "Upload failed: {}",
                response.status()
            )));
        }

        let parsed = response.json::<UploadResponse>().await?;
        Ok(parsed)
    }
}
