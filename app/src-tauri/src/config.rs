use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub model_config_file: &'static str,
    pub media_dir_name: &'static str,
    pub database_file: &'static str,
    pub database_path: &'static str,
    pub lame_sidecar: &'static str,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model_config_file: "resources/en_US-libritts_r-medium.onnx.json",
            media_dir_name: "media",
            database_file: "sqlite:eckko.db",
            database_path: "eckko.db",
            lame_sidecar: "lame",
        }
    }
}
impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AppConfig {{ model_config_file: {}, media_dir_name: {}, database_file: {}, database_path: {}, lame_sidecar: {} }}",
            self.model_config_file,
            self.media_dir_name,
            self.database_file,
            self.database_path,
            self.lame_sidecar
        )
    }
}
